CREATE TYPE payment_status_type AS ENUM(
    'started_by_user',
    'started_with_poli',
    'cancelled',
    'failed',
    'unknown',
    'completed'
);

CREATE TABLE user_payment (
    id SERIAL NOT NULL PRIMARY KEY,
    vendor SERIAL NOT NULL REFERENCES vendor(id),
    payment_method payment_method NOT NULL,
    datetime_payment_made TIMESTAMP NOT NULL,
    datetime_recorded TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    asset_type ASSET_TYPE NOT NULL,
    units INT8 NOT NULL,
    unique_id VARCHAR(256) NOT NULL,
    CONSTRAINT Unique_payment_each_vendor UNIQUE(vendor, unique_id)
);

CREATE TABLE poli_payment_request (
    id SERIAL NOT NULL PRIMARY KEY,
    unique_code VARCHAR(12) NOT NULL,
    amount MONEY NOT NULL,
    user_id SERIAL REFERENCES users(id),
    started_at DATETIME NOT NULL,
    payment_status payment_status_type NOT NULL,
    transaction_ref_no VARCHAR(12) NOT NULL
);
