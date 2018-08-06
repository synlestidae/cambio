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
use domain::EthAccount;
use domain::EthAccountId;

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

    pub fn init_settlement<C: GenericConnection>(
        &self,
        db: &mut C,
        user: &User,
        counterparty_order: &Order,
        request: &OrderRequest) -> Result<OrderSettlement, CambioError> {
        let mut tx = db.transaction()?;
        if !request.is_fair(counterparty_order) {
            return Err(CambioError::unfair_operation(
                "Order request is not compatible with counterparty's order", 
                "Order request is not fair")
            );
        }

        let user_id = user.id.unwrap();
        let criteria: SettlementCriteria = counterparty_order.id.unwrap().get(&mut tx)?;

        // check request meets criteria
        if criteria.pledge_amount != request.pledge {
            return Err(CambioError::not_permitted(
                "Pledged sum must equal the minimum pledge in counterparty order", 
                "Pledge in settlement proposal does not match counterparty criteria")
            );
        }

        let find_err = CambioError::not_found_search(
            "Eth account not found", 
            "Eth account not found"
        );

        // update criteria to point to the correct eth account
        let eth_account = request.address.get_vec(&mut tx)?
            .into_iter()
            .filter(|a| a.owner_id == user.owner_id.unwrap())
            //.map(|a| a.id.unwrap())
            .collect::<Vec<EthAccount>>()
            .pop()
            .ok_or(find_err)?;
        let eth_account_id = eth_account.id.unwrap();

        let account_set = AccountSet::from(user.owner_id.unwrap().get_vec(&mut tx)?)?;
        let (hold_account, amount) = if request.is_buy {
            (account_set.nzd_hold(), counterparty_order.amount_fiat)
        } else {
            (account_set.nzd_pledge_hold(), request.pledge)
        };
        self.ledger_service.transfer_money(&mut tx, 
            account_set.nzd_wallet(), 
            hold_account,
            AssetType::NZD,
            amount)?;

        let mut order = self.order_service.place_order(&mut tx, user_id, request)?;
        let mut updated_cp_order: Order = counterparty_order.id.unwrap().get(&mut tx)?;
        order.begin_settling();
        updated_cp_order.begin_settling();
        order.update(&mut tx)?;
        updated_cp_order.update(&mut tx)?;
        let mut settlement = 
            OrderSettlement::from(user_id, &order, counterparty_order, eth_account_id).create(&mut tx)?;
        settlement.eth_account = eth_account_id;
        tx.commit()?;
        Ok(settlement)
    }
}
