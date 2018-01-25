CREATE TYPE denom_type AS ENUM (
    'dollar',
    'cent',
    'satoshi',
    'wei',
    'szabo'
);

CREATE TYPE asset_code_type AS ENUM (
    'nzd',
    'btc',
    'eth'
);

CREATE TABLE asset_type (
    id SERIAL NOT NULL PRIMARY KEY,
    asset_code asset_code_type NOT NULL,
    denom denom_type NOT NULL,
    unique(asset_code, denom)
);
