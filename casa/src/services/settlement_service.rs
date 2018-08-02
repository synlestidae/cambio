use db::CambioError;
use domain::Decimal;
use api::OrderRequest;
use domain::{User, Order, OrderSettlement, SettlementCriteria};
use postgres::GenericConnection;
use repository::{Creatable, Readable};
use services::{LedgerService, OrderService};
use web3::types::H160;

pub struct SettlementService { 
    order_service: OrderService
}

type SettleResult = Result<OrderSettlement, CambioError>;

impl SettlementService {
    pub fn new() -> Self {
        Self { 
            order_service: OrderService::new() 
        }
    }

    pub fn init_settlement_of_sell<C: GenericConnection>(
        &self,
        db: &mut C,
        sell_order: &Order,
        buying_user: &User,
        order_request: &OrderRequest) -> Result<(), CambioError> {
        let user_id = buying_user.id.unwrap();
        let mut tx = db.transaction()?;

        // place a complementary order before linking it 
        // with the sell_order in a settlement
        let buy_order = self.order_service.place_order(&mut tx, user_id, order_request)?;
        OrderSettlement::from(user_id, &buy_order, sell_order)
            .create(&mut tx)?;

        tx.commit();
        Ok(())
    }

    pub fn init_settlement_of_buy<C: GenericConnection>(
        &self,
        db: &mut C,
        counterparty_order: &Order,
        selling_user: &User,
        order_request: &OrderRequest,
        pledge: Decimal,
        source_eth_account_address: H160) -> SettleResult {
        let counterparty_order_id = counterparty_order.id.unwrap();
        let criteria: SettlementCriteria = counterparty_order_id.get(db)?;
        if criteria.min_pledge_amount != pledge {
            return Err(CambioError::not_permitted(
                "Pledged sum must equal the minimum pledge in counterparty order", 
                "Pledge in settlement proposal does not match counterparty criteria")
            );
        }

        // move the pledge to the pledge holding account

        unimplemented!()
    }
}
