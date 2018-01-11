CREATE SEQUENCE correspondence_id_seq;

CREATE OR REPLACE FUNCTION transfer_asset(
    asset_code_var VARCHAR(4),
    asset_denom_var VARCHAR(6),
    account_period_start DATE,
    account_period_end DATE,
    debit_account INTEGER,
    credit_account INTEGER,
    units UINT,
    authorship_id INTEGER
)
RETURNS VOID AS $$
DECLARE 
correspondence_id INTEGER;
asset_type_id INTEGER;
account_period_id INTEGER;
last_debit_account_balance INT8;
last_credit_account_balance INT8;
last_transaction_id INTEGER;
BEGIN
    SELECT asset_type.id INTO asset_type_id FROM asset_type WHERE asset_code = asset_code_var AND denom = asset_denom_var LIMIT 1;
    IF asset_type_id IS NULL THEN
        RAISE EXCEPTION 'Cannot move asset with unknown asset type (% in %s)', asset_code_var, asset_denom_var; 
    END IF;

    SELECT id INTO account_period_id FROM accounting_period WHERE account_period_start = from_date AND account_period_end = to_date LIMIT 1;
    IF account_period_id IS NULL THEN
        RAISE EXCEPTION 'Not match for accounting period';
    END IF;


    SELECT MAX(journal.id) INTO last_transaction_id 
        FROM JOURNAL 
        JOIN account ON journal.account_id = account.id 
        WHERE account.id = debit_account;

    SELECT balance INTO last_debit_account_balance FROM journal 
        JOIN account ON journal.account_id = account.id 
        WHERE account.id = debit_account AND journal.id = last_transaction_id;
      
    SELECT balance INTO last_credit_account_balance FROM journal 
        JOIN account ON journal.account_id = account.id 
        WHERE account.id = credit_account AND journal.id = last_transaction_id;

    IF last_debit_account_balance IS NULL THEN
       last_debit_account_balance = 0;
    END IF;

    IF last_credit_account_balance IS NULL THEN
       last_credit_account_balance = 0;
    END IF;

    -- Still need to do the whole authorship thing
    
    correspondence_id := nextval('correspondence_id_seq');
    INSERT INTO journal(accounting_period, account_id, asset_type, correspondence_id, credit, debit, balance, authorship_id)
    VALUES 
    (account_period_id, debit_account, asset_type_id, correspondence_id, units, null, last_credit_account_balance + units, authorship_id), 
    (account_period_id, credit_account, asset_type_id, correspondence_id, null, units, last_debit_account_balance - units, authorship_id);

END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_asset_id(
    asset_code_var VARCHAR(4),
    asset_denom_var VARCHAR(6)
)
RETURNS INTEGER AS $$
DECLARE 
asset_id INTEGER;
BEGIN
    SELECT id INTO asset_id FROM asset_type WHERE asset_code = asset_code_var AND denom = asset_denom_var;
    return asset_id;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION make_order(
    -- identify the user
    email_address_var VARCHAR(128),
    user_session_id VARCHAR(128),

    -- what the user wants to sell
    sell_asset_type VARCHAR(4),
    sell_asset_denom VARCHAR(6),
    min_sell_units UINT,
    max_sell_units UINT,

    -- what the user wants to buy
    deposit_asset_type VARCHAR(4),
    deposit_asset_denom VARCHAR(6),
    min_buy_units UINT,
    max_buy_units UINT,

    -- for completing the order
    credit_account INTEGER,
    debit_account INTEGER

)
RETURNS VOID AS $$
DECLARE 
user_id INTEGER;
sell_asset_id INTEGER;
buy_asset_id INTEGER;
order_info_id INTEGER;
authorship_id INTEGER;
desired_ttl_milliseconds UINT;
BEGIN
    PERFORM check_user_session(email_address_var, user_session_id);

    -- get the asset ID - will check the asset types match up

    SELECT * INTO sell_asset_id FROM get_asset_id(sell_asset_type, sell_asset_denom);
    SELECT * INTO buy_asset_id FROM get_asset_id(buy_asset_type, buy_asset_denom);

    SELECT users.id INTO user_id FROM users 
        JOIN user_role ON users.id = user_role.user_id
        JOIN user_session ON users.id = user_session.user_id 
        JOIN session_info ON user_session.session_info_id = session_info.id
        WHERE user_role.user_role_type = 'make_order' AND
              users.email_address = email_address_var;

    IF (user_id IS NULL) THEN
        RAISE EXCEPTION 'Error UserAccountMatchError: Cannot find a match for user';
    END IF;

    -- Check the debit account exists
    IF (NOT EXISTS(SELECT * FROM account
      JOIN account_owner ON account.owner_id = account_owner.id
      JOIN account_owner ON account_owner.user_id = user_id 
    WHERE account.id = debit_account AND 
          account.asset_type = sell_asset_id)) THEN
        RAISE EXCEPTION 'Error DebitAccountMatchError: Cannot find an account matching the owning user and/or with the specified asset type';
    END IF;

    -- Check the credit account exists
    IF (NOT EXISTS(SELECT * FROM account
      JOIN account_owner ON account.owner_id = account_owner.id
      JOIN account_owner ON account_owner.user_id = user_id 
    WHERE account.id = debit_account AND 
          account.asset_type = sell_asset_id)) THEN
        RAISE EXCEPTION 'Error CreditAccountMatchError: Cannot find an account matching the owning user and/or with the specified asset type';
    END IF;

    -- author the order
    INSERT INTO authorship(business_ends, authoring_user, authoring_user_session, approved_by, approving_session) 
        VALUES('asset_transfer_user_to_user', user_id, user_session_id)
        RETURNING id INTO authorship_id;

    INSERT INTO order_info(splittable) VALUES(FALSE) 
        RETURNING id INTO order_info_id;

    INSERT INTO asset_order(sell_units, buy_units, sell_asset_id, buy_asset_id, debit_account, credit_account, order_info, author_info, ttl_milliseconds, status)
        VALUES(sell_units, buy_units, sell_asset_id, buy_asset_id, debit_account_id, credit_account_id, order_info_id, author_info_id, desired_ttl_milliseconds, 'active');
END;
$$ LANGUAGE plpgsql;
