CREATE TABLE vendor (
    id SERIAL NOT NULL PRIMARY KEY,
    name VARCHAR(256) NOT NULL
);

CREATE TABLE user_payment (
    id SERIAL NOT NULL PRIMARY KEY,
    vendor SERIAL NOT NULL REFERENCES vendor(id),
    payment_datetime TIMESTAMP NOT NULL,
    asset_type SERIAL NOT NULL REFERENCES asset_type(id),
    units INT8 NOT NULL,
    unique_id VARCHAR(256) NOT NULL,
    CONSTRAINT Unique_payment_each_vendor UNIQUE(vendor, unique_id)
);
