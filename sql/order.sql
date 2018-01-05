CREATE TABLE order_info (
    id SERIAL PRIMARY KEY,
    splittable BOOLEAN
);

CREATE TYPE order_status AS ENUM (
    'active',
    'user_cancelled',
    'admin_cancelled',
    'settled',
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
    min_sell_asset_units UINT,
    max_sell_asset_units UINT,
    min_buy_asset_units UINT,
    max_buy_asset_units UINT,
    sell_asset_type SERIAL REFERENCES asset_type(id),
    buy_asset_type SERIAL REFERENCES asset_type(id),
    debit_account SERIAL REFERENCES account(id),
    crebit_account SERIAL REFERENCES account(id),
    order_info SERIAL UNIQUE REFERENCES order_info(id),
    author_info SERIAL REFERENCES authorship(id),
    ttl_milliseconds UINT NOT NULL, 
    status order_status,
    registered_at TIMESTAMP NOT NULL
);

CREATE TABLE orders_in_settlement (
    id SERIAL PRIMARY KEY,
    order_id SERIAL REFERENCES asset_order(id) NOT NULL,
    settlement_id  SERIAL REFERENCES orders_in_settlement(id) NOT NULL,
    UNIQUE(settlement_id, order_id)
);

CREATE TABLE order_settlement (
    id SERIAL PRIMARY KEY,
    settled_at TIMESTAMP NOT NULL,
    authorship_id SERIAL REFERENCES authorship(id) UNIQUE NOT NULL,
    status settlement_status NOT NULL
)
