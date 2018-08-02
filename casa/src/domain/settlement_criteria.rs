use domain::OrderId; 
use domain::Decimal;
use domain::EthAccountId;
use postgres;
use db::TryFromRow;
use db::TryFromRowError;

#[derive(TryFromRow)]
pub struct SettlementCriteria {
    pub order_id: OrderId,
    pub time_limit: i32,
    pub min_pledge_amount: Decimal,
    pub destination_account: EthAccountId
}
