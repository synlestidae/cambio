use payment::poli::*;
use domain::{User, PoliPaymentRequest};
use hyper::client::{Client};
use hyper::Url;
use serde_xml_rs::{deserialize, serialize};
use services::PoliError;
use serde::{Serialize, Deserialize};

pub struct PoliService {
    poli_config: PoliConfig,
}

impl PoliService {
    pub fn new(poli_config: &PoliConfig) -> Self {
        Self {
            poli_config: poli_config.clone()
        }
    }

    pub fn initiate_transaction(&self, poli_payment_request: &PoliPaymentRequest) 
        -> Result<InitiateTransactionResponse, PoliError> {
        let init_tx = InitiateTransaction::from_request(
            &self.poli_config,
            poli_payment_request
        );
        self.make_request(&self.poli_config.initiate_transaction_url, init_tx)
    }

    pub fn get_transaction(&self, transaction_token: &TransactionToken) 
        -> Result<GetTransactionResponse, PoliError> {
        let poli_config = &self.poli_config;
        let get_transaction = GetTransaction {
            merchant_code: poli_config.merchant_code.clone(),
            authentication_code: poli_config.authentication_code.clone(),
            transaction_token: transaction_token.clone()
        };
        self.make_request(&self.poli_config.get_transaction_url, get_transaction)
    }

    fn make_request<'a, E: Serialize, T: Deserialize<'a>>(&self, url: &Url, obj: E) -> Result<T, PoliError> {
        let mut buffer = Vec::new();
        try!(serialize(&obj, &mut buffer));
        let body: &[u8] = &buffer;
        let result = try!(Client::new()
            .post(&url.to_string())
            .body(body)
            .send()
        );
        let result_obj = try!(deserialize(result));
        Ok(result_obj)
    }
}
