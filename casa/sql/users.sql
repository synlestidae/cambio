CREATE DOMAIN UINT AS INT8
   CHECK(VALUE >= 0);

CREATE DOMAIN BIGUINT AS BIGINT
   CHECK(VALUE >= 0);

CREATE TYPE user_role_type AS ENUM (
    'make_order',
    'deposit_bitcoin',
    'deposit_nzd',
    'withdraw_bitcoin',
    'withdraw_nzd'
);

CREATE TYPE app_role_type AS ENUM (
    'create_user',
    'delete_user',
    'modify_user',
    'create_order',
    'delete_order',
    'settle_order',
    'auth_deposit_bitcoin',
    'auth_withdrawal_bitcoin',
    'auth_deposit_nzd',
    'auth_withdrawal_nzd'
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email_address VARCHAR(256) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

CREATE TABLE registration (
    id SERIAL PRIMARY KEY,
    email_address VARCHAR(256) NOT NULL,
    password_hash VARCHAR(256) NOT NULL,
    confirmation_code VARCHAR(5) NOT NULL,
    identifier_code VARCHAR(20) NOT NULL UNIQUE,
    requested_at TIMESTAMP NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    confirmed_at TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE registration_confirm_email (
    id SERIAL PRIMARY KEY,
    registration_id SERIAL REFERENCES registration(id),
    sent_at TIMESTAMP
);

CREATE TABLE user_role (
    user_id SERIAL REFERENCES users(id),
    role user_role_type NOT NULL,
    PRIMARY KEY (user_id, role)
);

CREATE TYPE session_state_type AS ENUM(
    'valid',
    'invalidated'
);

CREATE TABLE session_info (
    id SERIAL NOT NULL PRIMARY KEY,
    session_token VARCHAR(128) NOT NULL UNIQUE,
    started_at TIMESTAMP NOT NULL DEFAULT (now() at time zone 'utc'),
    session_state session_state_type NOT NULL DEFAULT 'valid',
    ttl_milliseconds UINT NOT NULL, -- session_state can be changed to expired if started_at + ttl_milliseconds < current_time
    CHECK (ttl_milliseconds < 86400000) -- right now, no sessions longer than a day
);

CREATE TABLE user_session (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id SERIAL REFERENCES users(id),
    session_info_id SERIAL REFERENCES session_info(id)
);

CREATE TABLE app_session (
    id SERIAL NOT NULL PRIMARY KEY,
    session_info_id SERIAL REFERENCES session_info(id)
);
