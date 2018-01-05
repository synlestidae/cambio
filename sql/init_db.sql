DO $$
DECLARE 
  session_token VARCHAR(128);
BEGIN
    INSERT INTO asset_type(asset_code, denom) VALUES ('nzd', 'cents');
    INSERT INTO asset_type(asset_code, denom) VALUES ('btc', 'sat');
END $$
