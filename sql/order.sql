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

CREATE TABLE asset_order (
    sell_asset_units UINT,
    buy_asset_units UINT,
    sell_asset_type SERIAL REFERENCES asset_type(id),
    buy_asset_type SERIAL REFERENCES asset_type(id),
    debit_account SERIAL REFERENCES account(id),
    crebit_account SERIAL REFERENCES account(id),
    order_info SERIAL REFERENCES order_info(id),
    author_info SERIAL REFERENCES authorship(id),
    ttl_milliseconds UINT NOT NULL, 
    status order_status,
    registered_at TIMESTAMP NOT NULL
);
