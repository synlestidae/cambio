use domain::OrderId; 
use domain::Decimal;
use domain::EthAccountId;
use postgres;
use db::TryFromRow;
use db::TryFromRowError;

#[derive(TryFromRow)]
pub struct SettlementCriteria {
    pub order_id: OrderId,
    pub time_limit_minutes: i32,
    pub pledge_amount: Decimal,
    pub from_account: Option<EthAccountId>,
    pub to_account: Option<EthAccountId>
}

impl SettlementCriteria {
    pub fn criteria_for_buy(order_id: OrderId, 
        time_limit_minutes: u32,
        pledge_amount: Decimal,
        to_account: EthAccountId) -> Self {
        Self {
            order_id: order_id,
            time_limit_minutes: time_limit_minutes as i32,
            pledge_amount: pledge_amount,
            from_account: None,
            to_account: Some(to_account),
        }
    }

    pub fn criteria_for_sell(order_id: OrderId, 
        time_limit_minutes: u32,
        pledge_amount: Decimal,
        from_account: EthAccountId) -> Self {
        Self {
            order_id: order_id,
            time_limit_minutes: time_limit_minutes as i32,
            pledge_amount: pledge_amount,
            from_account: Some(from_account),
            to_account: None
        }
    }
}
