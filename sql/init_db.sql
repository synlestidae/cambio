DO $$
DECLARE 
  session_token VARCHAR(128);
  poli_account INTEGER;
BEGIN
    INSERT INTO asset_type(asset_code, denom) VALUES ('nzd', 'cent');
    INSERT INTO asset_type(asset_code, denom) VALUES ('btc', 'sat');
    INSERT INTO accounting_period(from_date, to_date) VALUES('2018-01-01', '2018-03-30');

    PERFORM register_user('admin@cambio.co.nz', '$2a$04$tGJRoFRrsvRQYir0MWLbjefwa.otStWb/rR4VyGP8gcBsvmwLitay'); -- password1235

    UPDATE account SET account_role = 'system' 
        FROM account_owner, users
        WHERE account.owner_id = account_owner.id AND 
            account_owner.user_id = users.id AND
            users.email_address = 'admin@cambio.co.nz';

    SELECT account.id INTO poli_account
        FROM account, account_owner, users
        WHERE account.owner_id = account_owner.id AND 
            account_owner.user_id = users.id AND
            users.email_address = 'admin@cambio.co.nz';
            
    INSERT INTO vendor(name, intake_account) VALUES('Poli', poli_account);
END $$
