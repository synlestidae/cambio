use domain::{AssetType, Denom, Id};
use chrono::Duration;
use chrono::prelude::*;
use db::{TryFromRow, TryFromRowError};
use chrono::{DateTime, Utc};
use std;
use postgres::rows::Row;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: Option<Id>,
    pub unique_id: String,
    pub sell_asset_units: u64,
    pub buy_asset_units: u64,
    pub sell_asset_type: AssetType,
    pub sell_asset_denom: Denom,
    pub buy_asset_type: AssetType,
    pub buy_asset_denom: Denom,
    pub expires_at: DateTime<Utc>,
}

impl TryFromRow for Order {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError>
    where
        Self: std::marker::Sized,
    {
        let id_match: Option<Id> = row.get("order_id");

        let owner_id_match: Option<Id> = row.get("owner_id");
        let owner_id = try!(owner_id_match.ok_or(TryFromRowError::missing_field(
            "Order",
            "owner_id",
        )));

        let buy_asset_units_match: Option<i64> = row.get("buy_asset_units");
        let buy_asset_units_i64 = try!(buy_asset_units_match.ok_or(TryFromRowError::missing_field(
            "Order",
            "buy_asset_units",
        )));

        let unique_id_match: Option<String> = row.get("unique_id");
        let unique_id: String = try!(unique_id_match.ok_or(TryFromRowError::missing_field(
            "Order",
            "unique_id",
        )));

        let sell_asset_units_match: Option<i64> = row.get("sell_asset_units");
        let sell_asset_units_i64: i64 = try!(sell_asset_units_match.ok_or(
            TryFromRowError::missing_field(
                "Order",
                "sell_asset_units",
            ),
        ));

        let sell_asset_type_match: Option<String> = row.get("sell_asset_code");
        let sell_asset_type_string: String =
            try!(sell_asset_type_match.ok_or(TryFromRowError::missing_field(
                "Order",
                "sell_asset_code",
            )));

        let sell_asset_denom_match: Option<String> = row.get("sell_asset_denom");
        let sell_asset_denom_string: String = try!(sell_asset_denom_match.ok_or(
            TryFromRowError::missing_field("Order", "sell_asset_denom"),
        ));

        let buy_asset_type_match: Option<String> = row.get("buy_asset_code");
        let buy_asset_type_string: String =
            try!(buy_asset_type_match.ok_or(TryFromRowError::missing_field(
                "Order",
                "buy_asset_code",
            )));

        let buy_asset_denom_match: Option<String> = row.get("buy_asset_denom");
        let buy_asset_denom_string: String =
            try!(buy_asset_denom_match.ok_or(TryFromRowError::missing_field(
                "Order",
                "buy_asset_denom",
            )));

        let expires_at_match: Option<NaiveDateTime> = row.get("expires_at");
        let expires_at: NaiveDateTime =
            try!(expires_at_match.ok_or(TryFromRowError::missing_field(
                "Order",
                "registered_at",
            )));

        let buy_asset_units: u64;
        let sell_asset_units: u64;

        let sell_asset_type: AssetType;
        let sell_asset_denom: Denom;

        let buy_asset_type: AssetType;
        let buy_asset_denom: Denom;

        // God damn it this is boring - why haven't I learnt to macro yet?!

        if (buy_asset_units_i64 < 0) {
            return Err(TryFromRowError::bad_value(
                "Order",
                "buy_asset_units",
                buy_asset_units_i64,
            ));
        }

        if (sell_asset_units_i64 < 0) {
            return Err(TryFromRowError::bad_value(
                "Order",
                "buy_asset_units",
                buy_asset_units_i64,
            ));
        }

        buy_asset_units = buy_asset_units_i64 as u64;
        sell_asset_units = sell_asset_units_i64 as u64;

        match (
            AssetType::parse(&sell_asset_type_string),
            AssetType::parse(&buy_asset_type_string),
        ) {
            (Some(sat), Some(bat)) => {
                sell_asset_type = sat;
                buy_asset_type = bat;
            }
            _ => {
                return Err(TryFromRowError::new(
                    "Could not parse the buy_asset_units or sell_asset_units field",
                ))
            }
        }

        match (
            Denom::parse(&sell_asset_denom_string),
            Denom::parse(&buy_asset_denom_string),
        ) {
            (Some(sad), Some(bad)) => {
                sell_asset_denom = sad;
                buy_asset_denom = bad;
            }
            _ => {
                return Err(TryFromRowError::new(
                    "Could not parse the buy_asset_denom or sell_asset_denom field",
                ))
            }
        }

        Ok(Order {
            id: id_match,
            unique_id: unique_id,
            sell_asset_units: sell_asset_units,
            buy_asset_units: buy_asset_units,
            sell_asset_type: sell_asset_type,
            buy_asset_type: buy_asset_type,
            sell_asset_denom: sell_asset_denom,
            buy_asset_denom: buy_asset_denom,
            expires_at: DateTime::from_utc(expires_at, Utc),
        })
    }
}

impl Order {
    pub fn can_exchange(&self, other_order: &Order) -> bool {
        return self.buy_asset_type == other_order.sell_asset_type &&
            self.buy_asset_denom == other_order.sell_asset_denom;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderInfo {
    pub splittable: bool,
}
