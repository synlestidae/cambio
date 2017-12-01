CREATE TABLE asset_type (
    id SERIAL,
    code VARCHAR(4) NOT NULL,
    denom VARCHAR(6) NOT NULL,
    PRIMARY KEY (id, code, denom)
);

CREATE TABLE account (
    id SERIAL PRIMARY KEY,
    owner_id SERIAL,
    asset_type SERIAL,
    account_type ACCOUNT_TYPE,
    PRIMARY KEY id,
    FOREIGN KEY (asset_type) REFERENCES asset_type(id),
    FOREIGN KEY (owner_id) REFERENCES account_owner(id),
);

CREATE TABLE accounting_period (
    id SERIAL,
    from_date DATE,
    to_date DATE,
    UNIQUE(from_date, to_date)
);

CREATE TABLE account_owner (
    id SERIAL PRIMARY KEY,
    email_address CITEXT NOT NULL,
    UNIQUE (email_address)
);

CREATE TABLE journal (
    id SERIAL,
    accounting_period SERIAL,
    account_id SERIAL NOT NULL,
    asset_type SERIAL, 
    correspondence_id SERIAL NOT NULL,
    credit CURRENCY NOT NULL,
    debit CURRENCY NOT NULL,
    PRIMARY KEY (id), 
    FOREIGN KEY (account_id) REFERENCES account(id),
    FOREIGN KEY (asset_type) REFERENCES asset_type(id),
    CHECK((credit IS NOT NULL AND debit IS NULL) OR (credit IS NULL AND debit IS NOT NULL)),
);

CREATE CONSTRAINT TRIGGER zero_balance
AFTER INSERT OR UPDATE ON journal
DEFERRABLE
FOR EACH ROW EXECUTE PROCEDURE check_zero_balance();

CREATE OR REPLACE FUNCTION check_zero_balance
  RETURNS VOID AS $$
  BEGIN
    IF EXISTS(SELECT * FROM journal GROUP BY (accounting_period) HAVING sum(credit) != sum(debit)) THEN
        RAISE EXCEPTION 'Each account period must have zero balance';
    END IF

    IF EXISTS(SELECT * FROM journal HAVING sum(credit) != sum(debit)) THEN
        RAISE EXCEPTION 'Journal must have zero balance';
    END IF
END;

CREATE OR REPLACE FUNCTION check_insert_double
  RETURNS VOID AS $$
  BEGIN
    -- Check that row1[credit] = row2[debit]
    IF EXISTS(SELECT * FROM INSERTED HAVING SUM(debit) != sum(credit) OR COUNT(*) !== 2 
        GROUP BY accounting_period, asset_type, correspondence_id) THEN
        RAISE EXCEPTION 'Each inserted transaction must have exactly ONE corresponding transaction with maching currency, account period, and where debit == credit';
    END IF
END

CREATE CONSTRAINT TRIGGER double_entry
BEFORE INSERT ON journal
DEFERRABLE
EXECUTE check_insert_double

CREATE DOMAIN CURRENCY AS int8
   CHECK(VALUE >= 0);


CREATE TYPE ACCOUNT_TYPE AS ENUM (
    'credit_normal',
    'debit_normal'
);
