CREATE TYPE ACCOUNT_TYPE AS ENUM (
    'credit_normal',
    'debit_normal'
);

CREATE TABLE asset_type (
    id SERIAL NOT NULL PRIMARY KEY,
    asset_code VARCHAR(4) NOT NULL,
    denom VARCHAR(6) NOT NULL,
    unique(asset_code, denom)
);

CREATE TABLE accounting_period (
    id SERIAL PRIMARY KEY,
    from_date DATE,
    to_date DATE,
    UNIQUE(from_date, to_date)
);

CREATE TYPE account_business_type AS ENUM (
    'user_asset',
    'fee_cashin',
    'fee_cashout',
    'cashin_from_user',
    'cashout_to_user'
);

CREATE TABLE account_owner (
    id SERIAL PRIMARY KEY,
    internal_user_id INTEGER REFERENCES internal_user(id),
    user_id INTEGER REFERENCES users(id)--,
    CHECK ((internal_user_id IS NOT NULL AND user_id IS NULL) OR (internal_user_id IS NULL AND user_id IS NOT NULL)),
    UNIQUE(internal_user_id, user_id)
);

CREATE TYPE account_status_type AS ENUM (
    'active',
    'frozen',
    'closed'
);

CREATE TABLE account (
    id SERIAL NOT NULL PRIMARY KEY,
    owner_id SERIAL REFERENCES account_owner(id),
    asset_type SERIAL REFERENCES asset_type(id),
    account_type ACCOUNT_TYPE NOT NULL,
    account_business_type account_business_type NOT NULL,
    account_status account_status_type NOT NULL DEFAULT 'active'
);

CREATE TABLE journal (
    id SERIAL PRIMARY KEY,
    accounting_period SERIAL REFERENCES accounting_period(id),
    account_id SERIAL NOT NULL REFERENCES account(id),
    asset_type SERIAL NOT NULL REFERENCES asset_type(id), 
    transaction_time TIMESTAMP NOT NULL,
    correspondence_id SERIAL NOT NULL,
    credit UINT,
    debit UINT,
    balance INT8,
    authorship_id SERIAL REFERENCES authorship(id),
    CHECK((credit IS NOT NULL AND debit IS NULL) OR (credit IS NULL AND debit IS NOT NULL))
);

CREATE OR REPLACE FUNCTION check_zero_balance() RETURNS TRIGGER AS 
$$
  BEGIN
    IF EXISTS (SELECT accounting_period, SUM(credit), SUM(debit) FROM journal 
        GROUP BY (accounting_period) 
        HAVING SUM(credit) != SUM(debit)) THEN
        RAISE EXCEPTION 'Each account period must have zero balance';
    END IF;
    RETURN NULL;
END;
$$
LANGUAGE 'plpgsql' IMMUTABLE;

CREATE TRIGGER zero_balance
BEFORE INSERT OR UPDATE ON journal
EXECUTE PROCEDURE check_zero_balance();
