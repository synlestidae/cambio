CREATE TABLE asset_type (
    id SERIAL NOT NULL PRIMARY KEY,
    asset_code VARCHAR(4) NOT NULL,
    denom VARCHAR(6) NOT NULL,
    unique(asset_code, denom)
);
