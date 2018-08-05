use services::*;
use api::OrderApiImpl;
use tests::test_utils::*;
use repository::*;
use domain::*;
use api::*;
use web3::types::H160;

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
    let mut db = get_db_connection();
    let jack_order = place_order(JACK, 0xFFFFFFFF, 20, false);
    make_settlement(JOHN, jack_order.id.unwrap());
}

#[test]
fn test_settlement_state() {
    const JERRY: &'static str = "jerry@theoffice.com";
    const JESUS: &'static str = "jesus@theoffice.com";
    let jack = create_user(JERRY, 2000);
    let john = create_user(JESUS, 2000);
    let mut db = get_db_connection();
    let jerry_order = place_order(JERRY, 0xFFFFFFFF, 20, false);
    make_settlement(JESUS, jerry_order.id.unwrap());
    let settlement: OrderSettlement = jerry_order.id.unwrap().get(&mut db).unwrap();
    let updated_order: Order = jerry_order.id.unwrap().get(&mut db).unwrap();
    let eth_account: EthAccount = settlement.eth_account.get(&mut db).unwrap();
    let h160: H160 = eth_account.address.into();
    assert_eq!(SettlementStatus::WaitingEth, settlement.status);
    assert_eq!(OrderStatus::Settling, updated_order.status);
    assert_eq!(h160.low_u64(), Readable::get(JESUS, &mut db).unwrap().id.unwrap().0 as u64);
}

fn make_settlement(settler: &str, order_id: OrderId) {
    let mut order_api = OrderApiImpl::new(get_db_connection());
    let mut db = get_db_connection();
    let order: Order = order_id.get(&mut db).unwrap();
    let user = Readable::get(settler, &mut db).unwrap(); 
    let completion_request = OrderCompletionRequest {
        counterparty_order: order_id,
        order_request: OrderRequest {
            unique_id: format!("user_{:?}_order_id", order_id),
            amount_fiat: order.amount_fiat,
            amount_crypto: order.amount_crypto,
            is_buy: !order.is_buy(),
            minutes_active: 15,
            minutes_to_settle: 60 * 24,
            pledge: Decimal::from_dollars(5),
            address: get_test_address(user.id.unwrap())
        }
    };
    if order.is_buy() {
        order_api.complete_sell_order(&user, &completion_request).unwrap();
    } else {
        order_api.complete_buy_order(&user, &completion_request).unwrap();
    }
}

fn place_order(who: &str, wei: u64, dollars: u64, is_buy: bool) -> Order {
    use web3::types::H160;
    let mut order_api = OrderApiImpl::new(get_db_connection());
    let mut db = get_db_connection();
    let user = Readable::get(who, &mut db).unwrap(); 
    let request = OrderRequest {
        unique_id: format!("test_{}_{}_{}_{}", who, wei, dollars, is_buy), 
        amount_fiat: Decimal::from_dollars(dollars as i64),
        amount_crypto: wei.into(),
        is_buy: is_buy,
        minutes_active: 15,
        minutes_to_settle: 60 * 2,
        pledge: Decimal::from_dollars(5),
        address: ByteAddress::from(H160::random())
    };
    order_api.post_new_order(&user, &request).unwrap()
}

fn create_user(email: &str, dollars: i64) -> User {
    let mut db = get_db_connection();
    log_in(email, "password123");
    let user = Readable::get(email, &mut db).unwrap();
    let test_address = get_test_address(user.id.unwrap());
    let eth_account = EthAccount {
        id: None,
        address: test_address.clone(),
        name: "Test Eth Account".to_owned(),
        owner_id: user.owner_id.unwrap()
    };
    eth_account.create(&mut db).unwrap();
    let ledger_service = LedgerService::new();
    let account_set = AccountSet::from(user.owner_id.unwrap().get_vec(&mut db).unwrap()).unwrap();
    let poli = PaymentVendor::Poli.get(&mut db).unwrap();
    ledger_service.transfer_money(&mut db, poli, account_set.nzd_wallet(), AssetType::NZD, Decimal::from_dollars(dollars)).unwrap();
    user
}

fn get_test_address(id: UserId) -> ByteAddress {
    let h160_addr = H160::from(id.0 as u64);
    ByteAddress::from(h160_addr)
}
