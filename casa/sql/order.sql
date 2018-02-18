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
    'settled',
    'cancelled',
    'invalid'
);

CREATE TABLE eth_transactions (
    id SERIAL PRIMARY KEY
);

CREATE TABLE asset_order (
    id SERIAL PRIMARY KEY,
    owner_id SERIAL NOT NULL REFERENCES account_owner(id) ,
    unique_id VARCHAR(32) NOT NULL,

    sell_asset_units BIGUINT NOT NULL,
    buy_asset_units BIGUINT NOT NULL,
    sell_asset_type_id SERIAL REFERENCES asset_type(id) NOT NULL,
    buy_asset_type_id SERIAL REFERENCES asset_type(id) NOT NULL,

    status order_status NOT NULL DEFAULT 'active',
    expires_at TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    CONSTRAINT Unique_asset_order UNIQUE(owner_id, unique_id)
);

CREATE TABLE order_settlement (
    id SERIAL PRIMARY KEY,
    started_at TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    settled_at TIMESTAMP,
    starting_user SERIAL REFERENCES users(id) UNIQUE NOT NULL,
    status settlement_status NOT NULL DEFAULT 'settling',
    transaction_id SERIAL REFERENCES eth_transactions(id),
    buying_crypto_id SERIAL NOT NULL REFERENCES asset_order(id),
    buying_fiat_id SERIAL NOT NULL REFERENCES asset_order(id),
    CONSTRAINT Settle_only_two_orders UNIQUE(buying_crypto_id, buying_fiat_id)
);


CREATE OR REPLACE FUNCTION place_order(
    buy_asset_type_var ASSET_CODE_TYPE,
    buy_asset_denom_var DENOM_TYPE, 
    sell_asset_type_var ASSET_CODE_TYPE,
    sell_asset_denom_var DENOM_TYPE,
    unique_id_var VARCHAR,
    owner_id_var INTEGER,
    sell_asset_units_var BIGINT,
    buy_asset_units_var BIGINT,
    expires_at_var TIMESTAMP
)
RETURNS VOID AS $$
DECLARE 
  buy_asset_type_id_var INTEGER;
  sell_asset_type_id_var INTEGER;
BEGIN
    SELECT * INTO buy_asset_type_id_var FROM get_asset_id(buy_asset_type_var, buy_asset_denom_var);
    SELECT * INTO sell_asset_type_id_var FROM get_asset_id(sell_asset_type_var, sell_asset_denom_var);

    IF buy_asset_type_id_var IS NULL THEN
        RAISE EXCEPTION 'Buy asset ID not found';
    END IF;

    IF sell_asset_type_id_var IS NULL THEN
        RAISE EXCEPTION 'Buy asset ID not found';
    END IF;

    INSERT INTO asset_order(owner_id, unique_id, sell_asset_units, buy_asset_units, sell_asset_type_id,
        buy_asset_type_id, expires_at) 
     VALUES(owner_id_var, unique_id_var, sell_asset_units_var, buy_asset_units_var, sell_asset_type_id_var, buy_asset_type_id_var, expires_at_var);

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
  account_id_var INTEGER;
  asset_type_id INTEGER;
  existing_settlements INTEGER;
BEGIN
    -- buying in the sense that they're BUYING with fiat currency
    SELECT * INTO selling_order FROM asset_order WHERE id = buying_crypto_order_id;
    SELECT * INTO buying_order FROM asset_order WHERE id = buying_currency_order_id;

    IF selling_order IS NULL THEN
        RAISE EXCEPTION 'Crypto-buying order not found';
    END IF;

    IF buying_order IS NULL THEN
        RAISE EXCEPTION 'Currency-buying order not found';
    END IF;

    IF buying_order.sell_asset_type != selling_order.buy_asset_type THEN
        RAISE EXCEPTION 'Order sell type does not match other order buy type.';
    END IF;

    IF buying_order.buy_asset_type != selling_order.sell_asset_type THEN
        RAISE EXCEPTION 'Order buy type does not match other order sell type.';
    END IF;

    IF buying_order.sell_asset_units != selling_order.buy_asset_units THEN
        RAISE EXCEPTION 'Orders are not fairly matched.';
    END IF;

    IF buying_order.buy_asset_units != selling_order.sell_asset_units THEN
        RAISE EXCEPTION 'Orders are not fairly matched.';
    END IF;

    IF buying_order.expires_at >= (now() at time zone 'utc') THEN
        RAISE EXCEPTION 'Crypto-buying order has expired.';
    END IF;

    IF selling_order.expires_at >= (now() at time zone 'utc') THEN
        RAISE EXCEPTION 'Crypto-selling order has expired.';
    END IF;

    SELECT COUNT(*) INTO existing_settlements 
    WHERE buying_crypto_id = selling_order.id OR 
          buying_crypto_id = buying_order.id OR 
          selling_crypto_id = selling_order.id OR 
          selling_crypto_id = buying_order.id;

    IF existing_settlements > 0 THEN
        RAISE EXCEPTION 'Settlements with those orders already exist';
    END IF;

    INSERT INTO order_settlement(starting_user, buying_crypto_id, buying_fiat_id)  
        VALUES (starting_user, buying_crypto_order_id, buying_currency_order_id);
END;
$$ LANGUAGE plpgsql;

-- next stage involves logging an ethereum_outbound_transaction, and transferring fiat currency to the
-- selling user to the appropriate holding account

-- final stage marks the ethereum transaction as either completely passed or failed.
-- funds are released from the holding account to the selling user
