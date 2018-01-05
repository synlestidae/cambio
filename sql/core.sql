CREATE DOMAIN UINT AS int8
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

CREATE TYPE business_ends_type AS ENUM (
    /*'asset_transfer_bch_sat', -- blockchain transfer of bitcoin cash
    'asset_transfer_eth_eth', -- blockchain transfer of ether
    'cashout_eth_eth', 
    'cashout_bch_sat',
    'cashout_eth_eth',
    'cashin_bch_sat',
    'cashin_eth_eth' */
    'asset_deposit_nzd',
    'asset_withdrawal_nzd',
    'asset_transfer_nzd'
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email_address VARCHAR(256) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

CREATE TABLE internal_user (
    id SERIAL PRIMARY KEY,
    username VARCHAR(64) NOT NULL,
    password_hash TEXT NOT NULL
);

CREATE TABLE user_role (
    user_id SERIAL REFERENCES users(id),
    role user_role_type NOT NULL,
    PRIMARY KEY (user_id, role)
);

CREATE TABLE internal_user_roles (
    internal_user_id SERIAL REFERENCES internal_user(id),
    role app_role_type NOT NULL,
    PRIMARY KEY (internal_user_id, role)
);

CREATE TYPE session_state_type AS ENUM(
    'valid',
    'invalidated'
);

CREATE TABLE session_info (
    id SERIAL NOT NULL PRIMARY KEY,
    session_token VARCHAR(128) NOT NULL UNIQUE,
    started_at TIMESTAMP NOT NULL,
    session_state session_state_type NOT NULL,
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
    internal_user_id SERIAL REFERENCES internal_user(id),
    session_info_id SERIAL REFERENCES session_info(id)
);

CREATE TABLE authorship (
    id SERIAL NOT NULL PRIMARY KEY,
    business_ends business_ends_type NOT NULL,
    authoring_user SERIAL REFERENCES users(id) NOT NULL, 
    authoring_user_session SERIAL REFERENCES user_session(id) NOT NULL,
    approved_by SERIAL REFERENCES internal_user(id) NOT NULL,
    approving_session SERIAL REFERENCES app_session(id) NOT NULL,
    entry SERIAL UNIQUE REFERENCES relevant_entry NOT NULL
);

CREATE TABLE entry (
    id SERIAL NOT NULL PRIMARY KEY,
    user_payment REFERENCES user_payment(id),
    CONSTRAINT one_associated_table CHECK user_payment IS NOT NULL
)
