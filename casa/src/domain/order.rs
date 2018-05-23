use chrono::prelude::*;
use chrono::Duration;
use chrono::{DateTime, Utc};
use db::{CambioError, PostgresHelper, TryFromRow, TryFromRowError};
use domain;
use domain::{AssetType, Denom, Id, OrderId, OrderStatus, OwnerId, User};
use postgres;
use postgres::rows::Row;
use rand;
use repositories::UserRepository;
use repository::Retrievable;
use std;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, TryFromRow)]
pub struct Order {
    #[column_id(order_id)]
    pub id: Option<OrderId>,
    pub owner_id: OwnerId,
    pub unique_id: String,
    #[column_id(sell_asset_code)]
    pub sell_asset_type: AssetType,
    pub sell_asset_denom: Denom,
    pub sell_asset_units: i64,
    #[column_id(buy_asset_code)]
    pub buy_asset_type: AssetType,
    pub buy_asset_denom: Denom,
    pub buy_asset_units: i64,
    pub expires_at: DateTime<Utc>,
    pub status: OrderStatus,
}

impl Order {
    pub fn buy_szabo(owner: OwnerId, buy: u64, nzd_cents: u32, ttl_minutes: u32) -> Self {
        let now = Utc::now();
        let expiry = now + Duration::minutes(ttl_minutes as i64);

        Order {
            id: None,
            owner_id: owner,
            unique_id: random_string(),
            sell_asset_units: nzd_cents as i64,
            buy_asset_units: buy as i64,
            sell_asset_type: domain::AssetType::NZD,
            sell_asset_denom: domain::Denom::Cent,
            buy_asset_type: domain::AssetType::ETH,
            buy_asset_denom: domain::Denom::Szabo,
            expires_at: expiry,
            status: domain::OrderStatus::Active,
        }
    }

    pub fn sell_szabo(owner: OwnerId, buy_cents: u32, szabo: u64, ttl_minutes: u32) -> Self {
        let now = Utc::now();
        let expiry = now + Duration::minutes(ttl_minutes as i64);

        Order {
            id: None,
            owner_id: owner,
            unique_id: random_string(),
            sell_asset_units: szabo as i64,
            buy_asset_units: buy_cents as i64,
            sell_asset_type: domain::AssetType::ETH,
            sell_asset_denom: domain::Denom::Szabo,
            buy_asset_type: domain::AssetType::NZD,
            buy_asset_denom: domain::Denom::Cent,
            expires_at: expiry,
            status: domain::OrderStatus::Active,
        }
    }

    pub fn can_exchange(
        &self,
        buy_currency: &domain::Currency,
        sell_currency: &domain::Currency,
    ) -> bool {
        let are_compatible = (self.buy_asset_type == sell_currency.asset_type
            && self.buy_asset_denom == sell_currency.denom
            && self.sell_asset_type == buy_currency.asset_type
            && self.sell_asset_denom == buy_currency.denom);
        let one_is_crypto = self.buy_asset_type.is_crypto() || buy_currency.asset_type.is_crypto();
        return are_compatible && one_is_crypto;
    }

    pub fn is_fair(
        &self,
        buy_currency: &domain::Currency,
        sell_currency: &domain::Currency,
        buy_units: u64,
        sell_units: u64,
    ) -> bool {
        let units_match =
            self.sell_asset_units as u64 == buy_units && self.buy_asset_units as u64 == sell_units;
        let asset_types_match = self.sell_asset_type == buy_currency.asset_type
            && self.buy_asset_type == sell_currency.asset_type;
        let denoms_match = self.sell_asset_denom == buy_currency.denom
            && self.buy_asset_denom == sell_currency.denom;
        return units_match && asset_types_match && denoms_match;
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }

    pub fn is_active(&self) -> bool {
        !self.is_expired() && self.status == domain::OrderStatus::Active
    }
}

/*impl Retrievable<User> for Order {
    fn get<H: PostgresHelper>(&self, db: H) -> Result<User, CambioError> {
        match self.get_option(db) {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(CambioError::not_found_search("No user found for that order", 
                "Owner ID does not correspond to a user")),
            err => err
        }
    }

    fn get_option<H: PostgresHelper>(&self, db: H)  -> Result<Option<User>, CambioError> {
        let mut repo = UserRepository::new(db);
        match repo.get_owner(self.owner_id) {
            Ok(owner) => Ok(Some(owner)),
            Err(err) => err
        }
    }
}*/

fn random_string() -> String {
    let mut token = String::new();
    for _ in (0..32) {
        token.push(rand::random::<char>() as char);
    }
    token
}
