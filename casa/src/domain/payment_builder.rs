use chrono::prelude::*;
use domain::{AssetType, Payment, PaymentMethod, PaymentVendor};
use uuid::Uuid;

pub struct PaymentBuilder {
    asset_type: AssetType,
    payment_method: PaymentMethod,
    vendor: PaymentVendor,
}

impl PaymentBuilder {
    pub fn new(
        asset_type: AssetType,
        payment_method: PaymentMethod,
        vendor: PaymentVendor,
    ) -> PaymentBuilder {
        PaymentBuilder {
            asset_type: asset_type,
            payment_method: payment_method,
            vendor: vendor,
        }
    }

    pub fn transaction_details(
        self,
        unique_id: &str,
        datetime_payment_made: DateTime<Utc>,
        credit: i64,
    ) -> Result<Payment, PaymentBuilderError> {
        if !check_unique_id(&unique_id, &self.vendor) {
            return Err(PaymentBuilderError::MalformedUniqueId);
        }

        Ok(Payment {
            id: None,
            asset_type: self.asset_type,
            payment_method: self.payment_method,
            vendor: self.vendor,
            unique_id: unique_id.to_owned(),
            datetime_payment_made: datetime_payment_made,
            user_credit: credit,
        })
    }
}

pub struct PaymentBuilderGeneralInfo {
    builder: PaymentBuilder,
}

#[derive(Debug)]
pub enum PaymentBuilderError {
    MalformedUniqueId,
}

fn check_unique_id(unique_id: &str, vendor: &PaymentVendor) -> bool {
    match vendor {
        &PaymentVendor::Poli => {
            return Uuid::parse_str(unique_id).is_ok();
        }
    }
}
