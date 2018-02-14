        /*let settlement_result = self.db_helper.query(SELECT_ORDER_BY_ID_SQL, &[&order_id]);
        let settlement: OrderSettlementBuilder;

        if let Some(s) = try!(settlement_result).pop() {
            settlement = s;
        } else {
            return Ok(None);
        }

        let settlement_id = settlement.id.unwrap();

        let mut orders_in_settlement_result: Vec<OrderSettlementBuilder> =
            try!(self.db_helper.query(
                SELECT_ORDERS_IN_SETTLEMENT_SQL,
                &[&settlement_id],
            ));

        let order_settlement_builder: OrderSettlementBuilder;


            /*let buy_id = buying_crypto_order.id.unwrap();
            let sell_id = selling_order.id.unwrap();
            // steps to settle 
            // -1 check that orders aren't settled already
            let buying_settlement = try!(self.get_order_settlement_status(buy_id));
            let selling_settlement = try!(self.get_order_settlement_status(sell_id));
            if buying_settlement.is_none() || selling_settlement.is_none() {
                    return Err(CambioError::unfair_operation("At least one order is already in settlement.",
                        "At least one order has an existing settlement status."));
            }
            // 0 check that past transactions haven't already been made for this order
            // TODO! 
            // retrieve two orders from DB 

            // check orders both match - they must be what user is looking for
            if !selling_order.is_fair(&buying_crypto_order) {
                return Err(CambioError::unfair_operation("Cannot settle two orders who aren't mutual.",
                    "Order is_fair returned false"));
                unimplemented!() // TODO return an error 
            }

            // retrieve the monetary account for the fiat-currency receiver
            let buying_user = self.get_order_owner(buy_id).unwrap();
            let selling_user = self.get_order_owner(sell_id).unwrap();

            // retrieve the ethereum account for the crypto-currency seller 
            let account = try!(self._get_order_account(sell_id, selling_order));

            // check that both accounts have sufficient funds
            let account_id = account.id.unwrap();
            let statement = try!(self.account_service.get_latest_statement(account_id));
            if statement.closing_balance < buying_crypto_order.sell_asset_units {
                let mut error = CambioError::unfair_operation("Insufficient funds to buy crypto",
                    "User closing balance is less than order value");
                return Err(error);
            }

            // insert settlement into DB
            // transfer fiat funds from crypto-currency receiver to holding account
            // mark settlement as pending ethereum transaction
            // perform ethereum transaction
            // 10 check ethereum transaction has been confirmed, and do one API lookup
            // 11 mark settlement as ethereum confirmed, pending fiat fund transfer
            // 12 transfer fiat funds from holding account to fiat currency receiver
            // 13 mark settlement as finished
        */

        if let Some(s) = orders_in_settlement_result.pop() {
            order_settlement_builder = s;
        } else {
            return Ok(None);
        }

        let mut order_result: Vec<Order> = try!(self.db_helper.query(
            SELECT_ORDERS_IN_SETTLEMENT_SQL,
            &[&settlement_id],
        ));

        let buying_order: Order;
        let selling_order: Order;
        if order_result.len() != 2 {
            let sys_message = format!("Expected 2 orders, got {}", order_result.len());
            return Err(CambioError::shouldnt_happen("Failed to match the orders in settlement. ", &sys_message));
        }
        buying_order = order_result.pop().unwrap();
        selling_order = order_result.pop().unwrap();

        let settlement = order_settlement_builder.build(buying_order, selling_order);

        Ok(Some(settlement))*/


    /*fn _get_order_account(&mut self, user_id: Id, order: &Order) -> Result<Account, CambioError> {
        let q = repository::UserClause::Id(user_id);
        let user_match = try!(self.user_repo.read(&q)).pop();
        if let None = user_match  {
            return Err(CambioError::not_found_search("Cannot find the user who made part of this order", 
                "User id was None"));
        }
        let user = user_match.unwrap();
        let q = repository::UserClause::EmailAddress(user.email_address.clone());
        let accounts = try!(self.account_repo.read(&q));
        for account in accounts.into_iter() {
            if order.buy_asset_type == account.asset_type && 
            order.buy_asset_denom == account.asset_denom && 
            account.account_business_type == AccountBusinessType::UserCashWallet {
                    return Ok(account)
            }
        }
        Err(CambioError::not_found_search("Could not found an account to credit for this order", 
            "No account matches order asset type and wallet business type"))
    }*/
/*
 *


    pub fn get_order_settlement_status(
        &mut self,
        order_id: Id,
    ) -> Result<Option<OrderSettlement>, CambioError> {
        unimplemented!();
    }

    // ALL METHODS MUST BE IMMUNE TO REPLAY ATTACKS
    pub fn begin_order_settlement(&mut self, buying_crypto_order: &Order, selling_order: &Order) 
        -> Result<OrderSettlement, CambioError> {
            unimplemented!()
    }

*/
