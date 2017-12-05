DO $$
DECLARE 
  session_token VARCHAR(128);
BEGIN
    INSERT INTO asset_type(asset_code, denom) VALUES ('nzd', 'cents');
    INSERT INTO asset_type(asset_code, denom) VALUES ('btc', 'sat');
    --PERFORM register_user('just.mate.antunovic@gmail.com', '$2a$10$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy'); 
    --SELECT * into session_token FROM user_login('just.mate.antunovic@gmail.com', '$2a$10$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy');
    --RAISE NOTICE 'Session active: %', session_token;
    --PERFORM check_user_session('just.mate.antunovic@gmail.com', session_token);
END $$
