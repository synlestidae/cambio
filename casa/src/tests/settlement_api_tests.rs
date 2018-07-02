use domain::*;
use api::*;
use tests::test_utils::*;
use tests::order_utils::*;
use chrono::prelude::*;
use chrono::Duration;
use web3::types::U256;
use serde_json;

#[test]
fn test_settlement_gets_saved() {
    const JERRY: &'static str = "jerry@jackson.com";
    const JOE : &'static str = "joe@jackson.com";

    let jerry = log_in(JERRY, "grassword123");
    let joe = log_in(JOE, "grassword123");

    let szabo = 10000000;
    let money = 500000;

    quick_credit(JOE, money);

    post("http://www.cambio.co.nz/orders/new", Some(&jerry), Some(OrderRequest {
        unique_id: "3429or4890ptu98".to_string(),
        sell_asset_type: AssetType::NZD,
        sell_asset_units: money as i64,
        buy_asset_type: AssetType::ETHSzabo,
        buy_asset_units: szabo,
        expires_at: Utc::now() + Duration::minutes(15),
        max_wei: None
    }));

    quick_credit(JERRY, 100 * 100);

    let order: Order = serde_json::from_str(&post("http://www.cambio.co.nz/orders/new", Some(&joe), Some(OrderRequest {
        unique_id: "3429or4890ptu99".to_string(),
        buy_asset_type: AssetType::NZD,
        buy_asset_units: money as i64,
        sell_asset_type: AssetType::ETHSzabo,
        sell_asset_units: szabo,
        expires_at: Utc::now() + Duration::minutes(15),
        max_wei: Some(U256::from(1000000 as u64))
    }))).unwrap();

    post("http://www.cambio.co.nz/orders/buy", Some(&joe), Some(OrderBuy {
        order_id: order.id.unwrap().into(),
        order_request: OrderRequest {
            unique_id: "093215893th".to_string(),
            buy_asset_type: order.sell_asset_type,
            buy_asset_units: order.sell_asset_units,
            sell_asset_type: order.buy_asset_type,
            sell_asset_units: order.buy_asset_units,
            expires_at: Utc::now() + Duration::minutes(15),
            max_wei: None
        }
    }));
}
