use domain::Decimal;
use std::str::FromStr;

#[test]
fn test_addition() {
    assert_eq!(
        Decimal::from_str("0.00").unwrap() + Decimal::from_str("0.00").unwrap(), 
        Decimal::from_str("0.00").unwrap()
    );

    assert_eq!(
        Decimal::from_str("6.40").unwrap() + Decimal::from_str("3.50").unwrap(), 
        Decimal::from_str("9.90").unwrap()
    );

    assert_eq!(
        Decimal::from_str("1.20").unwrap() + Decimal::from_str("3.50").unwrap(), 
        Decimal::from_str("4.70").unwrap()
    );

    assert_eq!(
        Decimal::from_str("2.00").unwrap() + Decimal::from_str("2.00").unwrap(), 
        Decimal::from_str("4.00").unwrap()
    );

    assert_eq!(
        Decimal::from_str("0.00").unwrap() + Decimal::from_str("2.00").unwrap(), 
        Decimal::from_str("2.00").unwrap()
    );
}

#[test]
fn test_subtraction() {
    assert_eq!(
        Decimal::from_str("0.00").unwrap() - Decimal::from_str("0.00").unwrap(), 
        Decimal::from_str("0.00").unwrap()
    );

    assert_eq!(
        Decimal::from_str("0.00").unwrap() - Decimal::from_str("1.00").unwrap(), 
        Decimal::from_str("-1.00").unwrap()
    );

    assert_eq!(
        Decimal::from_str("0.00").unwrap() - Decimal::from_str("10.00").unwrap(), 
        Decimal::from_str("-10.00").unwrap()
    );
}
