CREATE TABLE vendor (
    id SERIAL NOT NULL PRIMARY KEY,
    name VARCHAR(256) NOT NULL,
    intake_account SERIAL NOT NULL REFERENCES account(id)
);

CREATE TABLE user_payment (
    id SERIAL NOT NULL PRIMARY KEY,
    vendor SERIAL NOT NULL REFERENCES vendor(id),
    datetime_payment_made TIMESTAMP NOT NULL,
    datetime_recorded TIMESTAMP NOT NULL,
    asset_type SERIAL NOT NULL REFERENCES asset_type(id),
    units INT8 NOT NULL,
    unique_id VARCHAR(256) NOT NULL,
    CONSTRAINT Unique_payment_each_vendor UNIQUE(vendor, unique_id)
);

CREATE OR REPLACE FUNCTION credit_account_from_payment(
    -- all these IDs belong together
    user_id INTEGER,
    user_session_token VARCHAR,
    email_address VARCHAR,
    credited_account_id INTEGER,

    asset_type VARCHAR,
    asset_denom VARCHAR,

    -- stuff that comes 'over the write' from the broker
    vendor_name VARCHAR,
    datetime_payment_made TIMESTAMP,
    unique_id VARCHAR(256),
    units INT8 NOT NULL,

    -- must be logged in on the "app" side
    internal_user_id INTEGER,
    internal_user_session_token VARCHAR
)
RETURNS VOID AS $$
DECLARE 
asset_type_id INTEGER;
vendor_id INTEGER;
intake_account INTEGER;
user_credited_account INTEGER;
entry_id INTEGER;
BEGIN
    SELECT asset_type.id INTO asset_type_id FROM asset_type WHERE asset_code = asset_code_var AND denom = asset_denom_var LIMIT 1;
    IF asset_type_id IS NULL THEN
        RAISE EXCEPTION 'Cannot complete credit payment with unknown asset type (% in %s)', asset_type, asset_denom; 
    END IF;

    SELECT vendor.id INTO vendor_id, vendor.intake_account INTO intake_account FROM vendor WHERE name = vendor_name;

    SELECT account.id INTO user_credited_account FROM account
        JOIN account_owner ON account.owner_id = account_owner.id
        JOIN users ON account_owner.user_id = users.id
    WHERE 
        users.id = user_id AND users.email_address = email_address;

    IF user_credited_account IS NULL THEN
        RAISE EXCEPTION 'Could not find the account to credit (or debit) with payment';
    END IF;

    -- this payment will be linked to the actual transfer in the ledger
    INSERT INTO user_payment(vendor, datetime_payment_made, asset_type, units, unique_id)
        VALUES (asset_type_id, datetime_payment_made, asset_type_id, units, unique_id);

    INSERT INTO entry VALUES(user_payment) RETURNING id INTO entry_id;

    SELECT INTO internal_user_session_token 

    -- declare why the transfer of assets is made
    INSERT INTO authorship(business_ends, authoring_user, authoring_user_session, message, approved_by, approving_session, entry)
        VALUES ('wallet_deposit', user_id, user_session_id, message, internal_user_id, internal_user_session_id, entry_id)
        RETURNING id INTO authorship_id;

    -- if units are positive then it is a standard credit to the users account
    -- they have bought money and get it added to their account
    IF units >= 0 THEN
        SELECT transfer_asset(asset_type, asset_denom, account_period_start, 
            account_period_end, intake_account, user_credited_account, abs(units), authorship_id);
    ELSE 
    -- possible uses are reversals, refunds, chargebacks
        SELECT transfer_asset(asset_type, asset_denom, account_period_start, 
            account_period_end, user_credited_account, intake_account, abs(units), authorship_id);
    END IF;

END;
$$ LANGUAGE plpgsql;
