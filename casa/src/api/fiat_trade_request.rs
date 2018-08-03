use api::OrderRequest;
use chrono::prelude::*;
use chrono::NaiveDate;
use domain::Decimal;
use domain::ByteAddress;
use api::TradeRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatTradeRequest {
    pub trade_request: TradeRequest,
    pub min_pledge_amount: Decimal,
    pub dest_eth_account_address: ByteAddress,
    pub time_limit_minutes: i32
}

