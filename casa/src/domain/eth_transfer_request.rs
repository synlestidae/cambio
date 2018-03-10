use web3::types;

pub struct EthTransferRequest {   
    pub from_address: types::H160,
    pub to_address: types::H160,
    pub value_wei: types::U256,
    pub max_fee: types::U256
}
