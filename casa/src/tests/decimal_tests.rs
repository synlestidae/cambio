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
