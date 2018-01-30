CREATE DOMAIN HASH VARCHAR(64);
CREATE DOMAIN TX_ADDRESS VARCHAR(40);
CREATE DOMAIN ECDSA_SIGNATURE VARCHAR(260);  

CREATE TABLE ethereum_block (
    time TIMESTAMP NOT NULL,
    block UINT PRIMARY KEY,
    block_hash HASH NOT NULL 
);

CREATE TABLE ethereum_outbound_transaction (
    id SERIAL PRIMARY KEY,
    nonce VARCHAR NOT NULL DEFAULT '0',
    gas_price BIGINT NOT NULL,
    gas_limit BIGINT NOT NULL,
    to_address TX_ADDRESS NOT NULL,
    from_address TX_ADDRESS NOT NULL,
    hash HASH NOT NULL,
    value BIGINT NOT NULL,
    signature ECDSA_SIGNATURE NOT NULL UNIQUE,
    transaction_block_id UINT REFERENCES ethereum_block(block),
    unique_id VARCHAR NOT NULL
);

CREATE TABLE ethereum_account_details (
    address TX_ADDRESS PRIMARY KEY,
    encrypted_private_key_base64 VARCHAR NOT NULL,
    private_key_sha256_hash VARCHAR NOT NULL,
    iv_base64 VARCHAR NOT NULL
);
