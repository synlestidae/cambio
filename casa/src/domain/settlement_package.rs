use domain::*;
use web3::types::*;
use chrono::Duration;
use postgres::GenericConnection;
use postgres::Error;
use db::CambioError;
use repository::Updateable;

pub struct SettlementPackage {
    pub settlement: OrderSettlement,
    pub original_order: Order,
    pub settling_order: Order,
    pub criteria: SettlementCriteria,
    pub eth_transfer: EthTransfer,
    pub settlement_addr: H160,
    pub criteria_addr: H160
}

impl SettlementPackage {
    pub fn can_proceed(&self) -> bool {
        self.settlement.can_proceed()
    }

    pub fn addresses_match(&self) -> bool {
        let (expected_from, expected_to) = if self.settlement.settles_buy {
            // if it settles a buy, a customer specified that the order amount goes into
            // criteria_addr
            (self.settlement_addr, self.criteria_addr)
        } else {
            (self.criteria_addr, self.settlement_addr)
        };

        return self.eth_transfer.from == expected_from && 
            self.eth_transfer.to == expected_to;
    }

    pub fn completes_settlement(&self) -> bool {
        let expected_value = self.original_order.amount_crypto.clone().into();
        return self.addresses_match() && self.eth_transfer.value == expected_value;

    }

    // Returns true if the Ethereum transfer was made after settlement but before the due time
    pub fn is_on_time(&self) -> bool {
        let start_time = self.settlement.started_at.timestamp() as u64;
        let end_time = (self.settlement.started_at + Duration::minutes(self.criteria.time_limit_minutes as i64)).timestamp() as u64;

        let eth_timestamp = self.eth_transfer.timestamp.low_u64();

        start_time <= eth_timestamp && eth_timestamp <= end_time
    }

    pub fn mark_settled(&mut self) {
        self.original_order.mark_settled();
        self.settling_order.mark_settled();
        self.settlement.mark_settled();
    }

    pub fn update_fields<C: GenericConnection>(&mut self, db: &mut C) -> Result<(), CambioError> {
        self.original_order.update(db)?;
        self.settling_order.update(db)?;
        self.settlement.update(db)?;
        Ok(())
    }
}

mod test {
    use domain::*; 
    use chrono::prelude::*;

    #[test]
    fn it_works() {
        let value: u64 = 0x0FFFFFFFFFFFF;
        let mut buy = OrderBuilder::new(OwnerId(0))
            .trade_nzd_eth(Decimal::from_dollars(20), value.into())
            .buy_fiat(15, "test_buy");
        let buy_id = OrderId(0);
        buy.id = Some(buy_id);

        let criteria = SettlementCriteria::criteria_for_buy(
            buy_id,
            24 * 60,
            Decimal::from_dollars(5),
            EthAccountId(0)
        );

        let mut sell = OrderBuilder::new(OwnerId(1))
            .trade_nzd_eth(Decimal::from_dollars(20), value.into())
            .sell_fiat(15, "test_buy");
        let sell_id = OrderId(1);
        sell.id = Some(sell_id);

        let settlement = OrderSettlement::from(
            UserId(0), 
            &sell,
            &buy, 
            EthAccountId(1)
        );

        let eth_transfer = EthTransfer {
            block_number: 0.into(),
            hash: 0.into(),
            from: 4.into(),
            to: 5.into(),
            value: value.into(),
            timestamp: Utc::now().timestamp().into()
        };

        let package = SettlementPackage {
            settlement: settlement,
            original_order: buy,
            settling_order: sell,
            criteria: criteria,
            settlement_addr: 4.into(),
            criteria_addr: 5.into(),
            eth_transfer: eth_transfer
        };

        assert!(package.can_proceed());
        assert!(package.addresses_match());
        assert!(package.completes_settlement());
        assert!(package.is_on_time());
    }
}
