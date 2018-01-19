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
Order    'settling',
    'settled',
    'cancelled',
    'invalid'
);

CREATE TABLE asset_order (
    id SERIAL PRIMARY KEY,
    account_owner_id account_owner(id),

    sell_asset_units UINT,
    buy_asset_units UINT,
    sell_asset_type SERIAL REFERENCES asset_type(id),
    buy_asset_type SERIAL REFERENCES asset_type(id),

    debit_account SERIAL REFERENCES account(id),
    crebit_account SERIAL REFERENCES account(id),

    author_info SERIAL REFERENCES authorship(id),

    ttl_milliseconds UINT NOT NULL, 
    status order_status,
    registered_at TIMESTAMP NOT NULL,
    settlement_id SERIAL order_settlement(id)
);

CREATE TABLE order_settlement (
    id SERIAL PRIMARY KEY,
    started_at TIMESTAMP NOT NULL,
    settled_at TIMESTAMP,
    authorship_id SERIAL REFERENCES authorship(id) UNIQUE NOT NULL,
    status settlement_status NOT NULL,
    transaction_id SERIAL NOT NULL eth_transactions(id)
);
