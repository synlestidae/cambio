use domain::EthereumAccountDetails;

#[test]
fn test_decrypts_private_key() {
    let private_key = "49913d3c5df358817e10749cdf9a32700e90a0ce4f93787381458c03846671da";
    let address = "4d3ab46248812802cabec071417cce67ce4593807f12b8afe40c90aa22150e01";
    let password = "11235ilovedogsdogsarenice";
    let d = EthereumAccountDetails::new(address,
        private_key.to_owned(),
        password.to_owned());

    let decrypted_private_key = d.decrypt_private_key(password.to_owned()).unwrap();
    assert_eq!(private_key, decrypted_private_key);
        
}

#[test]
fn test_fails_decrypt_private_key() {
    let private_key = "49913d3c5df358817e10749cdf9a32700e90a0ce4f93787381458c03846671da";
    let address = "4d3ab46248812802cabec071417cce67ce4593807f12b8afe40c90aa22150e01";
    let password = "11235ilovedogsdogsarenice";
    let d = EthereumAccountDetails::new(address,
        private_key.to_owned(),
        password.to_owned());

    let decrypted_private_key = d.decrypt_private_key("idontknowthepasswordletmein".to_owned());
    assert!(decrypted_private_key.is_err());
}
