CREATE TYPE ACCOUNT_TYPE AS ENUM (
    'asset',
    'liability',
    'equity',
    'income',
    'expense'
);

CREATE TABLE accounting_period (
    id SERIAL PRIMARY KEY,
    from_date DATE,
    to_date DATE,
    UNIQUE(from_date, to_date)
);

CREATE TYPE account_business_type AS ENUM (
    'user_cash_wallet',
    'order_payment_hold',
    'system_fees_paid',
    'accounting_concept'
);

CREATE TYPE account_role AS ENUM (
    'primary',
    'system'
);

CREATE TABLE account_owner (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) NOT NULL UNIQUE
);

CREATE TYPE account_status_type AS ENUM (
    'active',
    'frozen',
    'closed'
);

CREATE TABLE account (
    id SERIAL NOT NULL PRIMARY KEY,
    owner_id SERIAL REFERENCES account_owner(id),
    asset_type ASSET_TYPE NOT NULL,
    account_type ACCOUNT_TYPE NOT NULL,
    account_business_type account_business_type NOT NULL,
    account_role account_role NOT NULL,
    account_status account_status_type NOT NULL DEFAULT 'active',
    UNIQUE (owner_id, asset_type, account_business_type, account_role)
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

