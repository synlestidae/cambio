use api::OrderRequest;
use chrono::prelude::*;
use chrono::NaiveDate;
use domain::OrderId;
use domain::Decimal;
use domain::ByteAddress;
use api::TradeRequest;
use web3::types::H160;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoTradeRequest {
    pub trade_request: TradeRequest,
    pub pledge_amount: Decimal,
    pub source_eth_account_address: ByteAddress 
}
