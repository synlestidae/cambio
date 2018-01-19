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

CREATE TABLE asset_order (
    id SERIAL PRIMARY KEY,
    owner_id account_owner(id) NOT NULL,
    unique_id VARCHAR(32) NOT NULL,

    sell_asset_units BIGUINT NOT NULL,
    buy_asset_units BIGUINT NOT NULL,
    sell_asset_type_id SERIAL REFERENCES asset_type(id) NOT NULL,
    buy_asset_type_id SERIAL REFERENCES asset_type(id) NOT NULL,

    ttl_milliseconds UINT NOT NULL, 
    status order_status NOT NULL DEFAULT 'active',
    registered_at TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    settlement_id SERIAL NOT NULL order_settlement(id),
    CONSTRAINT Unique_asset_order UNIQUE(owner_id, unique_id)
);

CREATE TABLE order_settlement (
    id SERIAL PRIMARY KEY,
    started_at TIMESTAMP NOT NULL,
    settled_at TIMESTAMP,
    authorship_id SERIAL REFERENCES authorship(id) UNIQUE NOT NULL,
    status settlement_status NOT NULL,
    transaction_id SERIAL NOT NULL eth_transactions(id)
);
