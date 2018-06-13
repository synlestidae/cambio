CREATE TYPE order_status AS ENUM (
    'active',
    'settling',
    'settled',
    'settlement_failed',
    'user_cancelled',
    'admin_cancelled',
    'expired',
    'deleted'
);

CREATE TYPE settlement_status AS ENUM (
    'settling',
    'waiting_eth',
    'waiting_eth_credentials',
    'settled',
    'cancelled',
    'invalid',
    'eth_failed'
);

CREATE TABLE eth_transactions (
    id SERIAL PRIMARY KEY
);

CREATE TABLE asset_order (
    id SERIAL PRIMARY KEY,
    owner_id SERIAL NOT NULL REFERENCES account_owner(id) ,
    unique_id VARCHAR(32) NOT NULL UNIQUE,
    sell_asset_units BIGUINT NOT NULL,
    buy_asset_units BIGUINT NOT NULL,
    sell_asset_type ASSET_TYPE NOT NULL,
    buy_asset_type ASSET_TYPE NOT NULL,
    status order_status NOT NULL DEFAULT 'active',
    expires_at TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    CONSTRAINT Unique_asset_order UNIQUE(owner_id, unique_id)
);

CREATE TABLE order_settlement (
    id SERIAL PRIMARY KEY,
    started_at TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    settled_at TIMESTAMP,
    starting_user SERIAL REFERENCES users(id) NOT NULL,
    status settlement_status NOT NULL DEFAULT 'settling',
    transaction_id SERIAL REFERENCES eth_transactions(id),
    buying_crypto_id SERIAL NOT NULL REFERENCES asset_order(id),
    buying_fiat_id SERIAL NOT NULL REFERENCES asset_order(id),
    CONSTRAINT Settle_only_two_orders UNIQUE(buying_crypto_id, buying_fiat_id)
);

-- TODO Find out why the FUCK I have to do this
ALTER TABLE order_settlement ALTER COLUMN transaction_id DROP NOT NULL;

CREATE OR REPLACE FUNCTION place_order(
    buy_asset_type_var ASSET_TYPE,
    sell_asset_type_var ASSET_TYPE,
    unique_id_var VARCHAR,
    owner_id_var INTEGER,
    sell_asset_units_var BIGINT,
    buy_asset_units_var BIGINT,
    expires_at_var TIMESTAMP
)
RETURNS VOID AS $$
BEGIN
    INSERT INTO asset_order(owner_id, 
        unique_id, 
        sell_asset_units, 
        buy_asset_units, 
        sell_asset_type,
        buy_asset_type, 
        expires_at) 
     VALUES(owner_id_var, 
        unique_id_var, 
        sell_asset_units_var,
        buy_asset_units_var,
        sell_asset_type_var,
        buy_asset_type_var,
        expires_at_var);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION begin_settlement(
   buying_crypto_order_id INTEGER,
   buying_currency_order_id INTEGER,
   starting_user INTEGER
)
RETURNS VOID AS $$
DECLARE 
  buying_order RECORD;
  selling_order RECORD;
  existing_settlements INTEGER;

  fiat_account INTEGER;
  hold_account INTEGER;

  accounting_period_start_var DATE;
  accounting_period_end_var DATE;

  authoring_user_var INTEGER;
  authorship_id_var INTEGER;
BEGIN
    -- buying in the sense that they're BUYING with fiat currency
    SELECT * INTO buying_order FROM asset_order WHERE id = buying_crypto_order_id;
    SELECT * INTO selling_order FROM asset_order WHERE id = buying_currency_order_id;

    IF selling_order IS NULL THEN
        RAISE EXCEPTION 'Crypto-buying order not found';
    END IF;

    IF buying_order IS NULL THEN
        RAISE EXCEPTION 'Currency-buying order not found';
    END IF;

    IF buying_order.sell_asset_type_id != selling_order.buy_asset_type_id THEN
        RAISE EXCEPTION 'Order sell type does not match other order buy type.';
    END IF;

    IF buying_order.buy_asset_type_id != selling_order.sell_asset_type_id THEN
        RAISE EXCEPTION 'Order buy type does not match other order sell type.';
    END IF;

    IF buying_order.sell_asset_units != selling_order.buy_asset_units THEN
        RAISE EXCEPTION 'Orders are not fairly matched.';
    END IF;

    IF buying_order.buy_asset_units != selling_order.sell_asset_units THEN
        RAISE EXCEPTION 'Orders are not fairly matched.';
    END IF;

    IF buying_order.expires_at < (now() at time zone 'utc') THEN
        RAISE EXCEPTION 'Crypto-buying order has expired. Current time is %s, but order expired at %s', (now() at time zone 'utc'), buying_order.expires_at;
    END IF;

    IF selling_order.expires_at < (now() at time zone 'utc') THEN
        RAISE EXCEPTION 'Crypto-selling order has expired.';
    END IF;

    SELECT id INTO hold_account FROM account
    WHERE 
        account.owner_id = buying_order.owner_id AND
        account.asset_type = buying_order.sell_asset_type_id AND
        account.account_business_type  = 'order_payment_hold' AND
        account.account_role = 'system';

    IF hold_account IS NULL THEN
        RAISE EXCEPTION 'Failed to find hold account with owner % and asset type %', buying_order.owner_id, buying_order.sell_asset_type_id;
    END IF;

    SELECT id INTO fiat_account FROM account
    WHERE 
        account.owner_id = buying_order.owner_id AND
        account.asset_type = buying_order.sell_asset_type_id AND
        account.account_business_type  = 'user_cash_wallet' AND
        account.account_role = 'primary'
    LIMIT 1;

    IF fiat_account IS NULL THEN
        RAISE EXCEPTION 'Failed to find fiat account';
    END IF;

    SELECT from_date INTO accounting_period_start_var FROM accounting_period
        WHERE id = (SELECT MAX(id) FROM accounting_period);

    SELECT to_date INTO accounting_period_end_var FROM accounting_period
        WHERE id = (SELECT MAX(id) FROM accounting_period);

    SELECT user_id INTO authoring_user_var FROM account_owner WHERE id = buying_order.owner_id;

    INSERT INTO authorship(business_ends, authoring_user, message, entry) 
    VALUES ('order_settlement', authoring_user_var, 'Holding funds for settlement', NULL)
    RETURNING id INTO authorship_id_var;

    -- do the transfer here
    PERFORM transfer_asset(
        asset_type_var := fiat_account.asset_type, 
        account_period_start := accounting_period_start_var, 
        account_period_end := accounting_period_end_var, 
        debit_account := fiat_account,
        credit_account := hold_account, 
        units := buying_order.sell_asset_units, 
        authorship_id := authorship_id_var);

    SELECT COUNT(*) INTO existing_settlements 
    FROM order_settlement
    WHERE buying_crypto_id = selling_order.id OR 
          buying_crypto_id = buying_order.id OR 
          buying_fiat_id = selling_order.id OR 
          buying_fiat_id = buying_order.id;

    IF existing_settlements > 0 THEN
        RAISE EXCEPTION 'Settlements with those orders already exist';
    END IF;

    UPDATE asset_order SET status = 'settling' WHERE id in (selling_order.id, buying_order.id); 

    INSERT INTO order_settlement(starting_user, buying_crypto_id, buying_fiat_id, transaction_id) 
        VALUES (starting_user, buying_crypto_order_id, buying_currency_order_id, NULL);

END;
$$ LANGUAGE plpgsql;

-- next stage involves logging an ethereum_outbound_transaction, and transferring fiat currency to the
-- selling user to the appropriate holding account

-- final stage marks the ethereum transaction as either completely passed or failed.
-- funds are released from the holding account to the selling user
