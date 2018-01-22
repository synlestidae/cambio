use domain::{Order, OrderSettlement, SettlementStatus};
use chrono::prelude::*;
use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;

pub struct OrderSettlementBuilder {
    pub id: Option<i32>,
    pub started_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
    pub settlement_status: SettlementStatus
}

impl OrderSettlementBuilder {
    pub fn new(id: Option<i32>,
        started_at: DateTime<Utc>,
        settled_at: Option<DateTime<Utc>>,
        settlement_status: SettlementStatus) -> Self {
        OrderSettlementBuilder {
            id: id,
            started_at: started_at,
            settled_at: settled_at,
            settlement_status: settlement_status,
        }
    }

    pub fn build(self, buying_order: Order, selling_order: Order) -> OrderSettlement {
        OrderSettlement {
            id: self.id,
            started_at: self.started_at,
            settled_at: self.settled_at,
            settlement_status: self.settlement_status,
            buying_order: buying_order,
            selling_order: selling_order
        }
    }
}

impl TryFromRow for OrderSettlementBuilder {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let id_match: Option<i32> = row.get("id");
        let id = try!(id_match.ok_or(TryFromRowError::missing_field("OrderSettlementBuilder", "id")));
        let started_at_match: Option<NaiveDateTime> = row.get("started_at");
        let started_at =
            try!(started_at_match.ok_or(TryFromRowError::missing_field("OrderSettlementBuilder", "started_at")));
        let settled_at_match: Option<NaiveDateTime> = row.get("settled_at");
        let settled_at =
            try!(settled_at_match.ok_or(TryFromRowError::missing_field("OrderSettlementBuilder", "settled_at")));
        let settlement_status_match: Option<SettlementStatus> = row.get("settlement_status");
        let settlement_status =
            try!(settlement_status_match.ok_or(TryFromRowError::missing_field("OrderSettlementBuilder",
                                                                         "settlement_status")));

        Ok(OrderSettlementBuilder {
            id: Some(id),
            started_at: DateTime::from_utc(started_at, Utc),
            settled_at: settled_at_match.map(|s| DateTime::from_utc(s, Utc)),
            settlement_status: settlement_status
        })
    }
}
