CREATE OR REPLACE FUNCTION transfer_asset(
    asset_code_var VARCHAR(4),
    asset_denom_var VARCHAR(6),
    account_period_start DATE,
    account_period_end DATE,
    debit_account INTEGER,
    credit_account INTEGER,
    units UINT
)
RETURNS VOID AS $$
DECLARE 
correspondence_id INTEGER;
asset_type_id INTEGER;
account_period_id INTEGER;
BEGIN
    SELECT id INTO asset_type_id FROM asset_type WHERE asset_code = asset_code_var AND denom = asset_denom_var LIMIT 1;
    IF asset_type_id IS NULL THEN
        RAISE EXCEPTION 'Cannot move asset with unknown asset type';
    END IF;
    SELECT id INTO account_period_id FROM accounting_period WHERE account_period_start = from_date AND account_period_start = to_date LIMIT 1;
    IF account_period_id IS NULL THEN
        RAISE EXCEPTION 'Cannot move asset with unknown asset type (% in %s)', asset_code_var, asset_denom_var;
    END IF;
    correspondence_id := (SELECT COUNT(*) FROM JOURNAL);
    INSERT INTO journal(accounting_period, account_id, asset_type_id, correspondence_id, credit, debit)
    VALUES (account_period_id, debit_account, asset_type_id, correspondence_id, units, null), 
           (account_period_id, credit_account, asset_type_id, correspondence_id, null, units);

END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION open_credit_normal_account(
    asset_code_var VARCHAR(4),
    asset_denom_var VARCHAR(4),
    owner_email VARCHAR(256)
) RETURNS VOID AS
$$
DECLARE 
owner_id INTEGER;
asset_type_id INTEGER;
BEGIN
    SELECT id INTO owner_id FROM account_owner WHERE email_address = owner_email LIMIT 1;
    IF owner_id IS NULL THEN
        INSERT INTO account_owner(email_address) VALUES (owner_email);
        SELECT id INTO owner_id FROM account_owner WHERE email_address = owner_email LIMIT 1;
    END IF;
    SELECT id INTO asset_type_id FROM asset_type WHERE asset_code = asset_code_var AND denom = asset_denom_var LIMIT 1;
    IF asset_type_id IS NULL THEN
        RAISE EXCEPTION 'Cannot create account with unknown asset type';
    END IF;
    INSERT INTO account(owner_id, asset_type, account_type) VALUES (owner_id, asset_type_id, 'credit_normal');
END;
$$ LANGUAGE plpgsql;
