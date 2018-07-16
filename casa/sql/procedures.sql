CREATE SEQUENCE correspondence_id_seq;

CREATE OR REPLACE FUNCTION transfer_asset(
    asset_type_var ASSET_TYPE,
    account_period_start DATE,
    account_period_end DATE,
    debit_account INTEGER,
    credit_account INTEGER,
    units UINT
)
RETURNS INTEGER AS $$
DECLARE 
correspondence_id INTEGER;
account_period_id INTEGER;
last_debit_account_balance INT8;
last_credit_account_balance INT8;
last_debit_transaction_id INTEGER;
last_credit_transaction_id INTEGER;
BEGIN
    SELECT id INTO account_period_id FROM accounting_period WHERE account_period_start = from_date AND account_period_end = to_date LIMIT 1;
    IF account_period_id IS NULL THEN
        RAISE EXCEPTION 'Not match for accounting period';
    END IF;

    SELECT MAX(journal.id) INTO last_debit_transaction_id 
        FROM JOURNAL 
        JOIN account ON journal.account_id = account.id 
        WHERE account.id = debit_account;

    SELECT MAX(journal.id) INTO last_credit_transaction_id 
        FROM JOURNAL 
        JOIN account ON journal.account_id = account.id 
        WHERE account.id = credit_account;

    SELECT balance INTO last_debit_account_balance FROM journal 
        JOIN account ON journal.account_id = account.id 
        WHERE account.id = debit_account AND journal.id = last_debit_transaction_id;
      
    SELECT balance INTO last_credit_account_balance FROM journal 
        JOIN account ON journal.account_id = account.id 
        WHERE account.id = credit_account AND journal.id = last_credit_transaction_id;

    IF last_debit_account_balance IS NULL THEN
       last_debit_account_balance = 0;
    END IF;

    IF last_credit_account_balance IS NULL THEN
       last_credit_account_balance = 0;
    END IF;

    IF last_debit_account_balance - units < 0 THEN
        IF EXISTS(SELECT * FROM account WHERE 
            account_business_type IN ('user_cash_wallet', 'order_payment_hold') AND id = debit_account) THEN
            RAISE EXCEPTION 'Insufficient funds. Account % has balance %, cannot deduct %', debit_account, last_debit_account_balance, units;
        END IF;
    END IF;

    -- Still need to do the whole authorship thing
    
    correspondence_id := nextval('correspondence_id_seq');
    INSERT INTO journal(accounting_period, account_id, asset_type, correspondence_id, debit, credit, balance)
    VALUES 
    (account_period_id, debit_account, asset_type_var, correspondence_id, units, null, last_debit_account_balance - units), 
    (account_period_id, credit_account, asset_type_var, correspondence_id, null, units, last_credit_account_balance + units);

    RETURN correspondence_id;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION make_order(
    -- identify the user
    email_address_var VARCHAR(128),
    user_session_id VARCHAR(128),

    -- what the user wants to sell
    sell_asset_type VARCHAR(4),
    min_sell_units UINT,
    max_sell_units UINT,

    -- what the user wants to buy
    deposit_asset_type ASSET_TYPE,
    min_buy_units UINT,
    max_buy_units UINT,

    -- for completing the order
    credit_account INTEGER,
    debit_account INTEGER

)
RETURNS VOID AS $$
DECLARE 
user_id INTEGER;
order_info_id INTEGER;
authorship_id INTEGER;
desired_ttl_milliseconds UINT;
BEGIN
    PERFORM check_user_session(email_address_var, user_session_id);

    -- get the asset ID - will check the asset types match up

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
          (account.account.business_type !== 'user_cash_wallet' OR account.balance - units > 0) AND
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

CREATE OR REPLACE FUNCTION credit_account_from_payment(
    -- all these IDs belong together
    user_id_var INTEGER,
    email_address_var VARCHAR,
    credited_account_id INTEGER,
    asset_type_var ASSET_TYPE,

    -- stuff that comes 'over the wire' from the broker
    vendor_name payment_vendor,
    payment_method_var payment_method,
    datetime_payment_made_var TIMESTAMP,
    unique_id VARCHAR,
    units INT8,
    message_var TEXT
)
RETURNS VOID AS $$
DECLARE 
authorship_id INTEGER;
entry_id INTEGER;
intake_account_var INTEGER;
user_credited_account INTEGER;
vendor_id INTEGER;
user_payment_id INTEGER;
accounting_period_start_var DATE;
accounting_period_end_var DATE;

debit_account_id INTEGER;
credit_account_id INTEGER;

BEGIN
    SELECT intake_account INTO intake_account_var
        FROM vendor 
        WHERE name = vendor_name;

    SELECT account.id INTO user_credited_account 
        FROM account
        JOIN account_owner ON account.owner_id = account_owner.id
        JOIN users ON account_owner.user_id = users.id
    WHERE 
        users.id = user_id_var AND 
        users.email_address = email_address_var AND 
        account.id = credited_account_id;

    IF user_credited_account IS NULL THEN
        RAISE EXCEPTION 'Could not find the account to credit with payment';
    END IF;

    SELECT vendor.id INTO vendor_id FROM vendor WHERE name = vendor_name;

    -- this payment will be linked to the actual transfer in the ledger
    INSERT INTO user_payment(vendor, payment_method, datetime_payment_made, asset_type, units, unique_id)
        VALUES (vendor_id, payment_method_var, datetime_payment_made_var, asset_type_var, units, unique_id)
        RETURNING id into user_payment_id;

    INSERT INTO entry(user_payment) VALUES(user_payment_id) RETURNING id INTO entry_id;

    -- declare why the transfer of assets is made
    INSERT INTO authorship(business_ends, authoring_user, message, entry)
        VALUES ('wallet_deposit', user_id_var, message_var, entry_id)
        RETURNING id INTO authorship_id;

    SELECT from_date INTO accounting_period_start_var FROM accounting_period
        WHERE id = (SELECT MAX(id) FROM accounting_period);

    SELECT to_date INTO accounting_period_end_var FROM accounting_period
        WHERE id = (SELECT MAX(id) FROM accounting_period);

    -- if units are positive then it is a standard credit to the users account
    -- they have bought money and get it added to their account

    IF units >= 0 THEN
        credit_account_id = user_credited_account; 
        debit_account_id = intake_account_var;
    ELSE 
        credit_account_id = intake_account_var; 
        debit_account_id = user_credited_account;
    END IF;

    PERFORM transfer_asset(
        asset_type_var := asset_type_var, 
        account_period_start := accounting_period_start_var, 
        account_period_end := accounting_period_end_var, 
        debit_account := debit_account_id,
        credit_account := credit_account_id, 
        units := CAST (abs(units) AS UINT), 
        authorship_id := authorship_id);
    RETURN;
END;
$$ LANGUAGE plpgsql;
