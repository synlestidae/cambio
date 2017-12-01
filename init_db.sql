INSERT INTO asset_type VALUES ('nzd', 'cent');
INSERT INTO accounting_period VALUES ('2017-12-01', '2017-12-31');

PERFORM open_credit_normal_account(
    'nzd',
    'cents'
    'just.mate.antunovic@gmail.com'
);

PERFORM open_credit_normal_account(
    'nzd',
    'cents'
    'mate@antunovic.nz'
);

PERFORM transfer_asset('nzd', 'cents', '2017-12-01', '2017-12-31', 0, 1, 100);
