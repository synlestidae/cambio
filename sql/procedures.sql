CREATE PROCEDURE transfer_asset
(
    @asset_code VARCHAR(4),
    @asset_denom VARCHAR(6),
    @account_period_start DATE,
    @account_period_end DATE,
    @debit_account SERIAL,
    @credit_account SERIAL,
    @units CURRENCY
)
AS 
DECLARE 
correspondence_id SERIAL;
asset_type_id SERIAL;
account_period_id SERIAL;
BEGIN
    IF NOT EXISTS (SELECT id FROM asset_type INTO @asset_type_id WHERE code = @asset_code AND asset_denom = @asset_denom LIMIT 1)
        RAISE EXCEPTION 'Cannot move asset with unknown asset type'
    END IF;
    IF NOT EXISTS (SELECT id FROM accounting_period INTO @account_period_id WHERE @account_period_start = from_date AND @account_period_start = end_date LIMIT 1)
        RAISE EXCEPTION 'Cannot move asset with unknown asset type'
    END IF;
    SET @correspondence_id = SELECT COUNT(*) FROM JOURNAL;
    INSERT INTO journal(accounting_period, account_id, asset_type_id, correspondence_id, credit, debit)
    VALUES (@account_period_id, @debit_account, @asset_type_id, @correspondence_id, @units, null), 
           (@account_period_id, @credit_account, @asset_type_id, @correspondence_id, null, @units)

END;

CREATE PROCEDURE open_credit_normal_account(
    @asset_code VARCHAR(4),
    @asset_denom VARCHAR(4),
    @owner_email CITEXT,
)
AS
DECLARE 
owner_id SERIAL;
asset_type_id SERIAL;
BEGIN
    IF NOT EXISTS(SELECT id FROM account_owner INTO @owner_id WHERE email_address = @owner_email LIMIT 1) THEN
        INSERT INTO account_owner VALUES (@owner_emmail);
    END IF;
    IF NOT EXISTS (SELECT id FROM asset_type INTO @asset_type_id WHERE code = @asset_code AND asset_denom = @asset_denom LIMIT 1)
        RAISE EXCEPTION 'Cannot create account with unknown asset type'
    END IF;
    INSERT INTO account VALUES (@owner_id, @asset_type, @asset_type_id, 'credit_normal')
END;
