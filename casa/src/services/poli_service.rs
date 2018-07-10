use payment::poli::*;
//use db::{PostgresHelper, Transaction};
use domain::{User, PoliPaymentRequest};
//use db::CambioError;
use hyper::client::Client;
use serde_xml_rs::{deserialize, serialize};
use services::PoliError;

pub struct PoliService {
    poli_config: PoliConfig,
    //db: H
}

impl PoliService {
    pub fn new(poli_config: PoliConfig) -> Self {
        Self {
            poli_config: poli_config
        }
    }

    pub fn initiate_transaction(&mut self, poli_payment_request: &PoliPaymentRequest) 
        -> Result<InitiateTransactionResponse, PoliError> {
        let init_tx = InitiateTransaction::from_request(
            &self.poli_config,
            poli_payment_request
        );

        // All unwraps to be removed

        let mut buffer = Vec::new();
        serialize(&init_tx, &mut buffer).unwrap();
        let body: &[u8] = &buffer;

        let res_result = Client::new()
            .post(&self.poli_config.initiate_transaction_url.to_string())
            .body(body)
            .send();

        let res = res_result.unwrap();
        let response: InitiateTransactionResponse = deserialize(res).unwrap();
        Ok(response)
    }

    pub fn get_transaction(&mut self, transaction_token: &TransactionToken) 
        -> Result<GetTransactionResponse, PoliError> {
        let poli_config = &self.poli_config;
        let get_transaction = GetTransaction {
            merchant_code: poli_config.merchant_code.clone(),
            authentication_code: poli_config.authentication_code.clone(),
            transaction_token: transaction_token.clone()
        };
        unimplemented!()
    }
}
