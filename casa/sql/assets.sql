CREATE TYPE asset_type AS ENUM (
    'nzd_cent',
    'aud_cent',
    'eth_wei',
    'eth_szabo'
);

CREATE TYPE crypto_type AS ENUM (
    'ether'
);

CREATE TYPE currency_code AS ENUM (
    'nzd',
    'aud'
);
