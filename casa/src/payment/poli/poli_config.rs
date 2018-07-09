use payment::poli::*;
use hyper::Url;

pub struct PoliConfig {
    merchant_code: MerchantCode,
    authentication_code: AuthenticationCode,
    notification_url: Url,
    unsuccessful_url: Url,
    merchant_checkout_url: Url, 
    merchant_home_page_url: Url
}
