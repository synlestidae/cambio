CREATE OR REPLACE FUNCTION internal_login(
    username_var VARCHAR(64),
    password_hash_var TEXT 
)
RETURNS VARCHAR(128) AS $$
DECLARE 
session_token VARCHAR(128);
session_info_id INTEGER;
user_id INTEGER;
BEGIN
    -- check that password matches bcrypt
    -- insert new session into session table
    -- return that session token
    IF (EXISTS(SELECT * FROM internal_users WHERE internal_users.username = username_var AND internal_users.password_hash = password_hash_var)) THEN
        session_token = random_string(128);

        -- invalidate other sessions for this user
        UPDATE session_info 
            SET session_state = 'invalidated' 
            FROM app_session,
                 internal_users 
            WHERE internal_users.username = username_var AND
                  app_session.internal_user_id = internal_user.id AND
                  app_session.session_info_id = session_info.id;

        -- get the user's id
        SELECT id INTO user_id FROM users WHERE username = username_var AND password_hash = password_hash_var;

        -- create the new session
        INSERT INTO session_info (session_token, started_at, session_state, ttl_milliseconds) 
            VALUES (session_token, now() at time zone 'utc', 'valid', 1000 * 60 * 60 * 6)
            RETURNING id INTO session_info_id;

        -- get that id
        INSERT INTO app_session(internal_user_id, session_info_id) 
            VALUES (user_id, session_info_id);

        RETURN session_token;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION activate_user_session (
    email_address_var VARCHAR(64)
)
RETURNS VARCHAR(128) AS $$
DECLARE 
session_token VARCHAR(128);
session_info_id INTEGER;
user_id INTEGER;
BEGIN
    -- check that password matches bcrypt
    -- insert new session into session table
    -- return that session token
    IF (EXISTS(SELECT * FROM users WHERE users.email_address = email_address_var)) THEN
        session_token = random_string(128);

        -- invalidate other sessions for this user
        UPDATE session_info 
        SET session_state = 'invalidated' 
        FROM user_session,
             users 
        WHERE users.email_address = email_address_var AND
              user_session.user_id = users.id AND
              user_session.session_info_id = session_info.id;

        -- get the user's id
        SELECT id INTO user_id FROM users WHERE email_address = email_address_var;

        -- create the new session
        INSERT INTO session_info (session_token, started_at, session_state, ttl_milliseconds) 
            VALUES (session_token, now() at time zone 'utc', 'valid', 1000 * 60 * 60 * 6) 
            RETURNING id INTO session_info_id;

        -- get that id
        INSERT INTO user_session (user_id, session_info_id) 
            VALUES (user_id, session_info_id);

        RETURN session_token;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Check that there is a session active for the internal user

CREATE OR REPLACE FUNCTION check_internal_user_session(
    username_var VARCHAR(64),
    session_token_var VARCHAR(128)
)
RETURNS VOID AS $$
DECLARE 
BEGIN
    IF (EXISTS(SELECT * FROM session_info
        JOIN app_session ON app_session.session_info_id = session_info.id 
        JOIN internal_user ON app_session.internal_user_id = internal_user.id
        WHERE 
            session_info.session_token = session_token_var AND 
            session_info.session_state = 'valid' AND 
            session_info.started_at + (session_info.ttl_milliseconds * ('1 millisecond'::INTERVAL)) < now() at time zone 'utc'

    )) THEN
        RETURN;
    END IF; 
    RAISE EXCEPTION 'Error NoActiveInternalSession: Could not find valid session for session token';
END;
$$ LANGUAGE plpgsql;

-- Check that there is a session active for ordinary user

CREATE OR REPLACE FUNCTION check_user_session(
    email_address_var VARCHAR(64),
    session_token_var VARCHAR(128)
)
RETURNS VOID AS $$
DECLARE 
BEGIN
    IF (EXISTS(SELECT * FROM session_info
        JOIN user_session ON user_session.session_info_id = session_info.id 
        JOIN users ON user_session.user_id = users.id
        WHERE 
            session_info.session_token = session_token_var AND 
            session_info.session_state = 'valid' AND 
            now() at time zone 'utc' < session_info.started_at + (session_info.ttl_milliseconds * ('1 millisecond'::INTERVAL))

    )) THEN
        RETURN;
    END IF; 
    RAISE EXCEPTION 'Error NoActiveUserSession: Could not find valid session for session token';
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION register_user(
    email_address_var VARCHAR(64),
    password_hash TEXT,
    OUT user_id  INTEGER
)
RETURNS INTEGER AS $$
DECLARE 
owner_id INTEGER;
bitcoin_asset_type_id INTEGER;
nzd_asset_type_id INTEGER;
BEGIN
    IF (password_hash IS NULL) THEN
        RAISE 'Error UserPasswordHashIsNull: Password hash cannot be null';
    END IF;
    IF (length(password_hash) != 60) THEN
        RAISE 'Error UserPasswordHashWrongSize: Password must be 60 characters';
    END IF;

    -- add the entry for logging in
    INSERT INTO users(email_address, password_hash) VALUES(email_address_var, password_hash)
        RETURNING id INTO user_id;     

    -- add the roles to allow them to do stuff
    INSERT INTO user_role(user_id, role) VALUES 
        (user_id, 'make_order'),
        (user_id, 'deposit_bitcoin'),
        (user_id, 'deposit_nzd'),
        (user_id, 'withdraw_bitcoin');

    -- give them a way to own accounts
    INSERT INTO account_owner(user_id, internal_user_id) VALUES(user_id, NULL) RETURNING id into owner_id;

    SELECT * INTO bitcoin_asset_type_id FROM get_asset_id('nzd', 'cents');
    SELECT * INTO nzd_asset_type_id FROM get_asset_id('btc', 'sat');

    -- create an account for bitcoins
    INSERT INTO account (owner_id, asset_type, account_type, account_business_type)
        VALUES(owner_id, bitcoin_asset_type_id, 'credit_normal', 'user_asset');

    -- create an account for nzd 
    INSERT INTO account (owner_id, asset_type, account_type, account_business_type)
        VALUES(owner_id, nzd_asset_type_id, 'credit_normal', 'user_asset');
END;
$$ LANGUAGE plpgsql;
