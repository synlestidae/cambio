DO $$
DECLARE 
  session_token VARCHAR(128);
  poli_account INTEGER;
  admin_owner INTEGER;
  nzd_id INTEGER;
BEGIN
    INSERT INTO asset_type(asset_code, denom) VALUES ('nzd', 'cent') RETURNING id INTO nzd_id;
    INSERT INTO asset_type(asset_code, denom) VALUES ('btc', 'satoshi');
    INSERT INTO asset_type(asset_code, denom) VALUES ('eth', 'szabo');
    INSERT INTO accounting_period(from_date, to_date) VALUES('2018-01-01', '2018-03-30');

    PERFORM register_user('admin@cambio.co.nz', '$2a$04$tGJRoFRrsvRQYir0MWLbjefwa.otStWb/rR4VyGP8gcBsvmwLitay'); -- password1235

    /*UPDATE account SET account_role = 'system' 
        FROM account_owner, users
        WHERE account.owner_id = account_owner.id AND 
            account_owner.user_id = users.id AND
            users.email_address = 'admin@cambio.co.nz';*/
    
    SELECT o.id INTO admin_owner FROM
        account_owner o, 
        users u
    WHERE
        o.user_id = u.id AND
        u.email_address = 'admin@cambio.co.nz';
        
    INSERT INTO account(owner_id, asset_type, account_type, account_business_type, account_role) 
    VALUES(admin_owner, nzd_id, 'asset', 'accounting_concept', 'system') RETURNING id INTO poli_account;
            
    INSERT INTO vendor(name, intake_account) VALUES('Poli', poli_account);
END $$
