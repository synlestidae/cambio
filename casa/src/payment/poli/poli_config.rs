use hyper::Url;
use payment::poli::*;

#[derive(Clone)]
pub struct PoliConfig {
    pub merchant_code: MerchantCode,
    pub authentication_code: AuthenticationCode,
    pub successful_url: Url,
    pub initiate_transaction_url: Url,
    pub get_transaction_url: Url,
    pub merchant_home_page_url: Url,
    pub notification_url: Url,
    pub unsuccessful_url: Url,
    pub merchant_checkout_url: Option<Url>,
}
