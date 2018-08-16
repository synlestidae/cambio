use api::UserSettlement;
use domain::OrderSettlement;
use domain::ByteAddress;
use domain::Order;
use db::CambioError;
use db::ConnectionSource;
use postgres::GenericConnection;
use repository::Readable;
use web3::types::U256;
use domain::User;
use chrono::Duration;
use chrono::prelude::*;
use domain::SettlementCriteria;

pub struct SettlementApiImpl<C: GenericConnection> {
    db: C,
}

impl<C: GenericConnection> SettlementApiImpl<C> {
    pub fn new(db: C) -> Self {
        Self {
            db: db,
        }
    }

    pub fn get_settlements(&mut self, user: &User) -> Result<Vec<UserSettlement>, CambioError> {
        let owner_id = user.owner_id.unwrap();
        let settlements: Vec<OrderSettlement> = user.id.unwrap().get_vec(&mut self.db)?; 
        let mut user_settlements = Vec::new();
        for settlement in settlements {
            let buy_order: Order = settlement.buying_crypto_id.get(&mut self.db)?;
            let sell_order: Order = settlement.buying_fiat_id.get(&mut self.db)?;
            let (user_order, other_order) = if buy_order.owner_id == owner_id { 
                (buy_order, sell_order)
            } else { 
                (sell_order, buy_order)
            };
            let (from_address, to_address, due) = self.get_blockchain_info(&settlement, &user_order, &other_order)?;
            user_settlements.push(UserSettlement {
                source_order: unimplemented!(),
                settlement_status: settlement.status,
                from_address: from_address,
                to_address: to_address,
                value: user_order.amount_crypto.into(),
                due_on_blockchain_at: due
            });
        }
        Ok(user_settlements)
    }

    pub fn get_blockchain_info(&mut self, settlement: &OrderSettlement, user_order: &Order, other_order: &Order) 
        -> Result<(ByteAddress, ByteAddress, DateTime<Utc>), CambioError> {
        let order_with_criteria_id = if user_order.is_buy() {
            user_order.id.unwrap()
        } else {
            other_order.id.unwrap()
        };
        let criteria: SettlementCriteria = order_with_criteria_id.get(&mut self.db)?;
        let due_on_blockchain_at = settlement.started_at + Duration::minutes(criteria.time_limit_minutes as i64);
        let (from_id, to_id) = match (user_order.is_buy(), criteria.from_account, criteria.to_account, settlement.eth_account) {
            (true, Some(from_id), None, to_id) => (from_id, to_id),
            (false, None, Some(to_id), from_id) => (from_id, to_id),
            _ => return Err(CambioError::shouldnt_happen("The status for the settlement unexpected.", 
                "Settlement state incorrect with respect to criteria."))
        };
        Ok((from_id.get(&mut self.db)?.address, to_id.get(&mut self.db)?.address, due_on_blockchain_at))
    }
}
