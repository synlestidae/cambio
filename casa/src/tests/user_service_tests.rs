/*use api::PersonalDetails;
use chrono::prelude::*;
use services::*;
use tests::order_utils::*;
use tests::test_utils::*;

#[test]
fn test_creates_eth_account() {
    let (eloop, web3) = get_web3();
    let service = UserService::new(web3);
    let mut conn = get_db_connection();
    let mut db = conn.transaction().unwrap();
    let user = service
        .create_user(
            &mut db,
            "frankie@antunovic.nz",
            "$2a$04$M1/oryAUgM7IsWaeDpoqfeWvh9oSEao0E1X4z3xEfwX.tEKTpCXRK",
            &PersonalDetails {
                first_names: "Frankie".to_string(),
                family_name: "Antunovic".to_string(),
                address_line_1: "42 Dog St".to_string(),
                address_line_2: None,
                post_code: "1123".to_string(),
                city: "New Plymouth".to_string(),
                country: "NEW ZEALAND".to_string(),
                dob: NaiveDate::from_ymd(2007, 1, 1),
                id_type: "NZPassport".to_string(),
                id_number: "DOG112358".to_string(),
            },
            "iloveeatingbacon",
        )
        .unwrap();
    assert_eq!("frankie@antunovic.nz", user.email_address);
    drop(service);
    drop(eloop);
}
*/
