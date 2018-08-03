use services::*;
use api::OrderApiImpl;
use tests::test_utils::*;
use repository::*;
use domain::*;
use api::*;

#[test]
fn test_places_buy_order() {
    const JAN: &'static str = "jan@theoffice.com";
    let user = create_user(JAN, 2000);
    place_order(JAN, 0xFFFFFFFF, 20, true);
}

#[test]
fn test_places_sell_order() {
    const JOE: &'static str = "joe@theoffice.com";
    let user = create_user(JOE, 2000);
    place_order(JOE, 0xFFFFFFFF, 20, false);
}

#[test]
fn test_creates_settlement_for_sell() {
    const JACK: &'static str = "jack@theoffice.com";
    const JOHN: &'static str = "john@theoffice.com";
    let jack = create_user(JACK, 2000);
    let john = create_user(JOHN, 2000);
    let jack_order = place_order(JACK, 0xFFFFFFFF, 20, false);
    let mut order_api = OrderApiImpl::new(get_db_connection());
    order_api.complete_sell_order(&john, &TradeRequest {
        counterparty_order: jack_order.id.unwrap(),
        order_request: OrderRequest {
            unique_id: format!("jack_order"),
            amount_fiat: Decimal::from_dollars(20),
            amount_crypto: (0xFFFFFFFF).into(),
            is_buy: true,
            minutes_active: 15
        }
    }).unwrap();
}

fn place_order(who: &str, wei: u64, dollars: u64, is_buy: bool) -> Order {
    let mut order_api = OrderApiImpl::new(get_db_connection());
    let mut db = get_db_connection();
    let request = OrderRequest {
        unique_id: format!("test_{}_{}_{}_{}", who, wei, dollars, is_buy), 
        amount_fiat: Decimal::from_dollars(dollars as i64),
        amount_crypto: wei.into(),
        is_buy: is_buy,
        minutes_active: 15,
        minutes_to_settle: 60 * 2,
        pledge: Decimal::from_dollars(5),
        address: unimplemented!()
    };
    //order_api.post_new_order(&request, who).unwrap()
    unimplemented!()
}

fn create_user(email: &str, dollars: i64) -> User {
    let mut db = get_db_connection();
    log_in(email, "password123");
    let user = Readable::get(email, &mut db).unwrap();
    let ledger_service = LedgerService::new();
    let account_set = AccountSet::from(user.owner_id.unwrap().get_vec(&mut db).unwrap()).unwrap();
    let poli = PaymentVendor::Poli.get(&mut db).unwrap();
    ledger_service.transfer_money(&mut db, poli, account_set.nzd_wallet(), AssetType::NZD, Decimal::from_dollars(dollars)).unwrap();
    user
}
