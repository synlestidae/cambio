use domain::{PoliPaymentRequest, User};
use hyper::client::Client;
use hyper::header::{Authorization, Basic, Headers};
use hyper::Url;
use payment::poli::*;
use serde;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, from_str, to_string, to_vec};
use services::PoliError;

pub struct PoliService {
    poli_config: PoliConfig,
}

impl PoliService {
    pub fn new(poli_config: &PoliConfig) -> Self {
        Self {
            poli_config: poli_config.clone(),
        }
    }

    pub fn initiate_transaction(
        &self,
        poli_payment_request: &PoliPaymentRequest,
    ) -> Result<InitiateTransactionResponse, PoliError> {
        let init_tx = InitiateTransaction::from_request(&self.poli_config, poli_payment_request);
        self.post(&self.poli_config.initiate_transaction_url, init_tx)
    }

    pub fn get_transaction(
        &self,
        transaction_token: &TransactionToken,
    ) -> Result<GetTransactionResponse, PoliError> {
        let poli_config = &self.poli_config;
        let mut url = self.poli_config.get_transaction_url.clone();
        url.query_pairs_mut()
            .append_pair("token", &transaction_token.to_string());
        self.get(&self.poli_config.get_transaction_url)
    }

    fn get<'a, T>(&self, url: &Url) -> Result<T, PoliError>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let result = try!(
            Client::new()
                .get(&url.to_string())
                .headers(self.get_headers())
                .send()
        );
        let result_obj = try!(from_reader(result));
        Ok(result_obj)
    }

    fn post<'a, E: Serialize, T>(&self, url: &Url, obj: E) -> Result<T, PoliError>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let body = try!(to_vec(&obj));
        let result = try!(
            Client::new()
                .post(&url.to_string())
                .headers(self.get_headers())
                .body(&body as &[u8])
                .send()
        );
        let result_obj = try!(from_reader(result));
        Ok(result_obj)
    }

    fn get_headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: self.poli_config.merchant_code.to_string(),
            password: Some(self.poli_config.authentication_code.to_string()),
        }));
        headers
    }
}
