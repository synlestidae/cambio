use payment::poli::*;
use domain::{User, PoliPaymentRequest};
use hyper::client::{Client};
use hyper::Url;
use services::PoliError;
use serde;
use serde::{Serialize, Deserialize};
use serde_json::{from_str, from_reader, to_string, to_vec};

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
            transaction_token: transaction_token.clone()
        };
        self.make_request(&self.poli_config.get_transaction_url, get_transaction)
    }

    fn make_request<'a, E: Serialize, T>(&self, url: &Url, obj: E) -> Result<T, PoliError> where for<'de> T: serde::Deserialize<'de> {
        //let mut buffer = Vec::new();
        let body = try!(to_vec(&obj));
        let result = try!(Client::new()
            .post(&url.to_string())
            .body(&body as &[u8])
            .send()
        );
        let result_obj = try!(from_reader(result));
        Ok(result_obj)
    }
}