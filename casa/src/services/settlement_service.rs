use db::CambioError;
use domain::Decimal;
use api::OrderRequest;
use domain::{User, Order, OrderSettlement, SettlementCriteria, BigInteger, SettlementTransaction};
use chrono::prelude::*;
use chrono::Duration;
use postgres::GenericConnection;
use repository::*;
use services::LedgerService;
use services::OrderService;
use domain::AccountSet;
use domain::AssetType;
use domain::ByteAddress;

pub struct SettlementService { 
    order_service: OrderService,
    ledger_service: LedgerService
}

type SettleResult = Result<OrderSettlement, CambioError>;

impl SettlementService {
    pub fn new() -> Self {
        Self { 
            order_service: OrderService::new() ,
            ledger_service: LedgerService::new()
        }
    }

    pub fn init_settlement_of_sell<C: GenericConnection>(
        &self,
        db: &mut C,
        sell_order: &Order,
        buying_user: &User,
        order_request: &OrderRequest) -> Result<OrderSettlement, CambioError> {
        let user_id = buying_user.id.unwrap();
        // place a complementary order before linking it 
        // with the sell_order in a settlement
        let buy_order = self.order_service.place_order(db, user_id, order_request)?;
        Ok(OrderSettlement::from(user_id, &buy_order, sell_order).create(db)?)
    }

    pub fn init_settlement_of_buy<C: GenericConnection>(
        &self,
        db: &mut C,
        counterparty_order: &Order,
        selling_user: &User,
        order_request: &OrderRequest,
        pledge: Decimal,
        from_address: ByteAddress) -> SettleResult {
        let user_id = selling_user.id.unwrap();
        let owner_id = selling_user.owner_id.unwrap();
        let mut tx = db.transaction()?;
        
        let counterparty_order_id = counterparty_order.id.unwrap();
        let settlement_criteria: SettlementCriteria = counterparty_order_id.get(&mut tx)?;
        if settlement_criteria.min_pledge_amount != pledge {
            return Err(CambioError::not_permitted(
                "Pledged sum must equal the minimum pledge in counterparty order", 
                "Pledge in settlement proposal does not match counterparty criteria")
            );
        }

        // move the pledge to the pledge holding account
        let accounts = AccountSet::from(owner_id.get_vec(&mut tx)?)?;
        self.ledger_service.transfer_money(&mut tx,
            accounts.nzd_wallet(),
            accounts.nzd_pledge_hold(),
            AssetType::NZD,
            pledge
        )?;

        let sell_order = self.order_service.place_order(&mut tx, user_id, order_request)?;
        let settlement = 
            OrderSettlement::from(user_id, &counterparty_order, &sell_order).create(&mut tx)?;

        let to_address = settlement_criteria.destination_account.get(&mut tx)?.address;

        let settlement_transaction = SettlementTransaction {
            settlement_id: settlement.id.unwrap(),
            from_address: from_address,
            to_address: to_address,
            amount_wei: counterparty_order.amount_crypto,
            blockchain_due_datetime: Utc::now() + 
                Duration::minutes(settlement_criteria.time_limit_minutes as i64)
        };
        settlement_transaction.create(&mut tx)?;
        tx.commit();
        Ok(settlement)
    }
}
