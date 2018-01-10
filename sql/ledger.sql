CREATE TYPE ACCOUNT_TYPE AS ENUM (
    'credit_normal',
    'debit_normal'
);
CREATE TABLE accounting_period (
    id SERIAL PRIMARY KEY,
    from_date DATE,
    to_date DATE,
    UNIQUE(from_date, to_date)
);

CREATE TYPE account_business_type AS ENUM (
    'user_cash_wallet',
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

CREATE TYPE payment_method AS ENUM ('credit_card', 'nz_bank_deposit');
CREATE TYPE payment_vendor AS ENUM ('Poli');

CREATE OR REPLACE FUNCTION credit_account_from_payment(
    -- all these IDs belong together
    user_id INTEGER,
    email_address VARCHAR,
    credited_account_id INTEGER,

    asset_type VARCHAR,
    asset_denom VARCHAR,

    -- stuff that comes 'over the wire' from the broker
    vendor_name payment_vendor,
    payment_method payment_method,
    datetime_payment_made TIMESTAMP,
    unique_id VARCHAR,
    units INT8
)
RETURNS VOID AS $$
DECLARE 
asset_type_id INTEGER;
authorship_id INTEGER;
entry_id INTEGER;
intake_account INTEGER;
user_credited_account INTEGER;
vendor_id INTEGER;
BEGIN
    SELECT asset_type.id INTO asset_type_id FROM asset_type WHERE asset_code = asset_code_var AND denom = asset_denom_var LIMIT 1;
    IF asset_type_id IS NULL THEN
        RAISE EXCEPTION 'Cannot complete credit payment with unknown asset type (% in %s)', asset_type, asset_denom; 
    END IF;

    SELECT vendor.id, vendor.intake_account INTO vendor_id, intake_account FROM vendor WHERE name = vendor_name;

    SELECT account.id INTO user_credited_account FROM account
        JOIN account_owner ON account.owner_id = account_owner.id
        JOIN users ON account_owner.user_id = users.id
    WHERE 
        users.id = user_id AND users.email_address = email_address AND account.id = credited_account_id;

    IF user_credited_account IS NULL THEN
        RAISE EXCEPTION 'Could not find the account to credit (or debit) with payment';
    END IF;

    -- this payment will be linked to the actual transfer in the ledger
    INSERT INTO user_payment(vendor, datetime_payment_made, asset_type, units, unique_id)
        VALUES (asset_type_id, datetime_payment_made, asset_type_id, units, unique_id);

    INSERT INTO entry VALUES(user_payment) RETURNING id INTO entry_id;

    -- declare why the transfer of assets is made
    INSERT INTO authorship(business_ends, authoring_user, authoring_user_session, message, approved_by, approving_session, entry)
        VALUES ('wallet_deposit', user_id, user_session_id, message, entry_id)
        RETURNING id INTO authorship_id;

    -- if units are positive then it is a standard credit to the users account
    -- they have bought money and get it added to their account
    IF units >= 0 THEN
        SELECT transfer_asset(asset_type, asset_denom, account_period_start, 
            account_period_end, intake_account, user_credited_account, abs(units), authorship_id);
    ELSE 
    -- otherwise the transaction is a reversal, refund, chargeback
        SELECT transfer_asset(asset_type, asset_denom, account_period_start, 
            account_period_end, user_credited_account, intake_account, abs(units), authorship_id);
    END IF;

END;
$$ LANGUAGE plpgsql;

CREATE TABLE account (
    id SERIAL NOT NULL PRIMARY KEY,
    owner_id SERIAL REFERENCES account_owner(id),
    asset_type SERIAL REFERENCES asset_type(id),
    account_type ACCOUNT_TYPE NOT NULL,
    account_business_type account_business_type NOT NULL,
    account_role account_role NOT NULL,
    account_status account_status_type NOT NULL DEFAULT 'active'
);

CREATE TABLE vendor (
    id SERIAL NOT NULL PRIMARY KEY,
    name VARCHAR(256) NOT NULL,
    intake_account SERIAL NOT NULL REFERENCES account(id)
);

CREATE TABLE user_payment (
    id SERIAL NOT NULL PRIMARY KEY,
    vendor SERIAL NOT NULL REFERENCES vendor(id),
    payment_method payment_method NOT NULL,
    datetime_payment_made TIMESTAMP NOT NULL,
    datetime_recorded TIMESTAMP NOT NULL,
    asset_type SERIAL NOT NULL REFERENCES asset_type(id),
    units INT8 NOT NULL,
    unique_id VARCHAR(256) NOT NULL,
    CONSTRAINT Unique_payment_each_vendor UNIQUE(vendor, unique_id)
);

CREATE TYPE business_ends_type AS ENUM (
    'wallet_deposit',
    'wallet_withdrawal',
    'system_fee_charge',
    'cryptocurrency_purchase'
);

CREATE TABLE entry (
    id SERIAL NOT NULL PRIMARY KEY,
    user_payment SERIAL NOT NULL REFERENCES user_payment(id)
);

CREATE TABLE authorship (
    id SERIAL NOT NULL PRIMARY KEY,
    business_ends business_ends_type NOT NULL,
    authoring_user SERIAL REFERENCES users(id) NOT NULL, 
    authoring_user_session SERIAL REFERENCES user_session(id) NOT NULL,
    message TEXT NOT NULL,
    approving_session SERIAL REFERENCES app_session(id) NOT NULL,
    entry SERIAL UNIQUE REFERENCES entry NOT NULL
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

