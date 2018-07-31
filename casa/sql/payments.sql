CREATE TYPE payment_method AS ENUM ('credit_card', 'nz_bank_deposit');
CREATE TYPE payment_vendor AS ENUM ('Poli');

CREATE TABLE vendor (
    id SERIAL NOT NULL PRIMARY KEY,
    name payment_vendor UNIQUE NOT NULL,
    intake_account SERIAL NOT NULL REFERENCES account(id)
);

CREATE TYPE business_ends_type AS ENUM (
    'wallet_deposit',
    'wallet_withdrawal',
    'system_fee_charge',
    'order_placement',
    'order_settlement'
);

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
    id SERIAL PRIMARY KEY,
    user_id SERIAL REFERENCES users(id),
    amount UINT NOT NULL,
    amount_paid_cents UINT NOT NULL,
    unique_code VARCHAR(12) NOT NULL,
    started_at TIMESTAMP NOT NULL,
    payment_status payment_status_type NOT NULL,
    transaction_ref_no VARCHAR(12) NOT NULL
);

CREATE TABLE poli_error_log (
    id SERIAL PRIMARY KEY,
    user_id SERIAL REFERENCES users(id),
    message VARCHAR NOT NULL,
    info VARCHAR,
    datetime TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc')
);

CREATE TABLE entry (
    id SERIAL NOT NULL PRIMARY KEY,
    user_payment SERIAL NOT NULL REFERENCES user_payment(id)
);

CREATE TABLE authorship (
    id SERIAL NOT NULL PRIMARY KEY,
    business_ends business_ends_type NOT NULL,
    authoring_user SERIAL REFERENCES users(id) NOT NULL, 
    message TEXT,
    entry SERIAL REFERENCES entry
);

ALTER TABLE authorship ALTER COLUMN entry DROP NOT NULL;
CREATE TABLE journal (
    id SERIAL PRIMARY KEY,
    accounting_period SERIAL REFERENCES accounting_period(id),
    account_id SERIAL NOT NULL REFERENCES account(id),
    asset_type ASSET_TYPE NOT NULL,  
    transaction_time TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    correspondence_id SERIAL NOT NULL,
    credit bigint,
    debit bigint,
    balance bigint,
    authorship_id INTEGER REFERENCES authorship(id) DEFAULT NULL,
    CHECK((credit IS NOT NULL AND debit IS NULL) OR (credit IS NULL AND debit IS NOT NULL)),
    CHECK(credit >= 0 OR debit >= 0)
);

ALTER TABLE authorship ALTER COLUMN entry DROP NOT NULL;
