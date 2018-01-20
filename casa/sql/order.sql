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

CREATE TABLE order_settlement (
    id SERIAL PRIMARY KEY,
    started_at TIMESTAMP NOT NULL,
    settled_at TIMESTAMP,
    authorship_id SERIAL REFERENCES authorship(id) UNIQUE NOT NULL,
    status settlement_status NOT NULL,
    transaction_id SERIAL NOT NULL REFERENCES eth_transactions(id)
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
    ttl_milliseconds UINT NOT NULL, 
    registered_at TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    settlement_id SERIAL NOT NULL REFERENCES order_settlement(id),
    CONSTRAINT Unique_asset_order UNIQUE(owner_id, unique_id)
);


CREATE OR REPLACE FUNCTION place_order(
    buy_asset_type_var VARCHAR,
    buy_asset_denom_var VARCHAR, 
    sell_asset_type_var VARCHAR,
    sell_asset_denom_var VARCHAR,
    owner_id_var INTEGER,
    unique_id_var VARCHAR,
    sell_asset_units_var BIGINT,
    buy_asset_units_var BIGINT,
    ttl_milliseconds_var UINT
)
RETURNS VOID AS $$
DECLARE 
  buy_asset_type_id INTEGER;
  sell_asset_type_id INTEGER;
BEGIN
    SELECT id INTO buy_asset_type_id FROM get_asset_id(buy_asset_type_var, buy_asset_denom_var);
    SELECT id INTO sell_asset_type_id FROM get_asset_id(sell_asset_type_var, sell_asset_denom_var);

    INSERT INTO asset_order(owner_id, sell_asset_units, buy_asset_units, sell_asset_type_id,
        buy_asset_type_id, ttl_milliseconds) 
     VALUES(owner_id_var, unique_id_var, sell_asset_units_var, buy_asset_units_var, sell_asset_type_id_var, buy_asset_type_id_var, ttl_milliseconds_var);

END;
$$ LANGUAGE plpgsql;
