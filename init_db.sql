INSERT INTO asset_type (asset_code, denom) VALUES ('nzd', 'cent');
INSERT INTO accounting_period (from_date, to_date) VALUES ('2017-12-01', '2017-12-31');

SELECT open_credit_normal_account(
    'nzd',
    'cent'
    'just.mate.antunovic@gmail.com'
);

SELECT open_credit_normal_account(
    'nzd',
    'cent'
    'mate@antunovic.nz'
);

SELECT transfer_asset('nzd', 'cents', '2017-12-01', '2017-12-31', 0, 1, 100);
