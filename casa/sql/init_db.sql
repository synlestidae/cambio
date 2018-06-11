DO $$
DECLARE 
  session_token VARCHAR(128);
  poli_account INTEGER;
  admin_owner INTEGER;
  nzd_id INTEGER;
BEGIN
    PERFORM register_user('admin@cambio.co.nz', '$2a$04$tGJRoFRrsvRQYir0MWLbjefwa.otStWb/rR4VyGP8gcBsvmwLitay'); -- password1235

    SELECT o.id INTO admin_owner FROM
        account_owner o, 
        users u
    WHERE
        o.user_id = u.id AND
        u.email_address = 'admin@cambio.co.nz';
        
    INSERT INTO account(owner_id, asset_type, account_type, account_business_type, account_role) 
    VALUES(admin_owner, 'nzd_cent', 'asset', 'accounting_concept', 'system') RETURNING id INTO poli_account;
            
    INSERT INTO vendor(name, intake_account) VALUES('Poli', poli_account);
END $$
