use config::EmailConfig;
use lettre::EmailAddress;
use payment::poli::*;
use serde_derive;
use std::io;
use toml;
use url::Url;

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    merchant_code: MerchantCode,
    authentication_code: AuthenticationCode,
    #[serde(with = "url_serde")]
    successful_url: Url,
    #[serde(with = "url_serde")]
    initiate_transaction_url: Url,
    #[serde(with = "url_serde")]
    get_transaction_url: Url,
    #[serde(with = "url_serde")]
    merchant_home_page_url: Url,
    #[serde(with = "url_serde")]
    notification_url: Url,
    #[serde(with = "url_serde")]
    unsuccessful_url: Url,
    #[serde(with = "url_serde", default)]
    merchant_checkout_url: Option<Url>,
    connection_string: String,
    web3_address: String,
    noreply_email_address: String,
    noreply_password: String,
    email_server_host: String,
}

impl ServerConfig {
    pub fn get_connection_string(&self) -> String {
        self.connection_string.to_string()
    }

    pub fn get_poli_config(&self) -> PoliConfig {
        PoliConfig {
            merchant_code: self.merchant_code.clone(),
            authentication_code: self.authentication_code.clone(),
            successful_url: self.successful_url.clone(),
            initiate_transaction_url: self.initiate_transaction_url.clone(),
            get_transaction_url: self.get_transaction_url.clone(),
            merchant_home_page_url: self.merchant_home_page_url.clone(),
            notification_url: self.notification_url.clone(),
            unsuccessful_url: self.unsuccessful_url.clone(),
            merchant_checkout_url: self.merchant_checkout_url.clone(),
        }
    }

    pub fn get_email_noreply_config(&self) -> EmailConfig {
        EmailConfig {
            login: EmailAddress::new(self.noreply_email_address.clone()).unwrap(),
            email_address: EmailAddress::new(self.noreply_email_address.clone()).unwrap(),
            password: EmailAddress::new(self.noreply_password.clone()).unwrap(),
            server_host: EmailAddress::new(self.email_server_host.clone()).unwrap(),
        }
    }

    pub fn get_web3_address(&self) -> String {
        self.web3_address.clone()
    }

    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        use std::fs::File;
        use std::io::Read;
        let mut config_file = File::open(path)?;
        let mut contents = String::new();
        config_file.read_to_string(&mut contents)?;
        Ok(toml::from_str(&contents).unwrap())
    }
}
