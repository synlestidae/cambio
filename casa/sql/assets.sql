CREATE TABLE asset_type (
    id SERIAL NOT NULL PRIMARY KEY,
    asset_code VARCHAR(8) NOT NULL,
    denom VARCHAR(8) NOT NULL,
    unique(asset_code, denom)
);
