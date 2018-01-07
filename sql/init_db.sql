DO $$
DECLARE 
  session_token VARCHAR(128);
BEGIN
    INSERT INTO asset_type(asset_code, denom) VALUES ('nzd', 'cent');
    INSERT INTO asset_type(asset_code, denom) VALUES ('btc', 'sat');
END $$
