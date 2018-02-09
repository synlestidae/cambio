CREATE TYPE order_status AS ENUM (
    'active',
    'settling',
    'settled',
    'settlement_failed',
    'user_cancelled',
    'admin_cancelled',
    'expired'
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
    started_at TIMESTAMP NOT NULL,
    settled_at TIMESTAMP,
    authorship_id SERIAL REFERENCES authorship(id) UNIQUE NOT NULL,
    status settlement_status NOT NULL,
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

    INSERT INTO asset_order(owner_id, unique_id, sell_asset_units, buy_asset_units, sell_asset_type_id,
        buy_asset_type_id, expires_at, settlement_id) 
     VALUES(owner_id_var, unique_id_var, sell_asset_units_var, buy_asset_units_var, sell_asset_type_id_var, buy_asset_type_id_var, expires_at_var, NULL);

END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION begin_settlement(
   buying_crypto_order_id INTEGER,
   buying_currency_order_id INTEGER,
   starting_user INTEGER
)
RETURNS VOID AS $$
DECLARE 
  account_id_var INTEGER;
  asset_type_id INTEGER;
BEGIN
    -- Create an authorship
    INSERT INTO authorship(business_ends, authoring_user, message) VALUES ('order_settlement', starting_user, 'Settling two orders');

    -- Transfer fiat currency from buying_crypto guy
    SELECT account.id INTO account_id_var FROM
        account
    JOIN account_owner ON account.owner_id = account_owner.id
    JOIN orders ON orders.owner_id = account_owner.id
    JOIN account ON account_owner.id = account.owner_id
    WHERE orders.id = buying_crypto_id AND
          account.account_business_type = 'order_payment_hold';


    -- literally cant be fucked



END;
$$ LANGUAGE plpgsql;
