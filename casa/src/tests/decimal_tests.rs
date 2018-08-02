use domain::Decimal;
use std::str::FromStr;

#[test]
fn test_addition() {
    run_add_test("0.00", "0.00", "0.00");
    run_add_test("6.40", "3.50", "9.90");
    run_add_test("1.20", "3.50", "4.70");
    run_add_test("0.00", "2.00", "2.00");
    run_add_test("0.00", "1.99", "1.99");
    run_add_test("0.01", "1.99", "2.00");
    run_add_test("1.99", "0.01", "2.00");
}

#[test]
fn test_subtraction() {
    run_sub_test("0.00", "0.00", "0.00");
    run_sub_test("0.00", "10.00", "-10.00");
    run_sub_test("5.00", "10.00", "-5.00");
    run_sub_test("2.00", "0.01", "1.99");
}

#[test]
fn test_conversion_to_cents() {
    assert_eq!(Decimal::from_str("8.60").unwrap().to_cents(), 860);
    assert_eq!(Decimal::from_str("1.60").unwrap().to_cents(), 160);
    assert_eq!(Decimal::from_str("0.60").unwrap().to_cents(), 60);
    assert_eq!(Decimal::from_str("0.01").unwrap().to_cents(), 1);
    assert_eq!(Decimal::from_str("0.02").unwrap().to_cents(), 2);
    assert_eq!(Decimal::from_str("0.05").unwrap().to_cents(), 5);
    assert_eq!(Decimal::from_str("1.27").unwrap().to_cents(), 127);
}

fn run_add_test(lhs: &str, rhs: &str, expected: &str) {
    assert_eq!(
        Decimal::from_str(lhs).unwrap() + Decimal::from_str(rhs).unwrap(),
        Decimal::from_str(expected).unwrap()
    );
}

fn run_sub_test(lhs: &str, rhs: &str, expected: &str) {
    assert_eq!(
        Decimal::from_str(lhs).unwrap() - Decimal::from_str(rhs).unwrap(),
        Decimal::from_str(expected).unwrap()
    );
}
