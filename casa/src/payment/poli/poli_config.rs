use payment::poli::*;
use hyper::Url;

pub struct PoliConfig {
    pub merchant_code: MerchantCode,
    pub authentication_code: AuthenticationCode,
    pub notification_url: Url,
    pub successful_url: Url,
    pub unsuccessful_url: Url,
    pub merchant_checkout_url: Url, 
    pub merchant_home_page_url: Url,
    pub initiate_transaction_url: Url
}
