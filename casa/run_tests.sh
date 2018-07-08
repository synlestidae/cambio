#!/usr/bin/env bash

clean_db () {
    echo "DROP DATABASE test_database_only" | psql
}

clean_db

set -e
set -o pipefail

clean_up () {
    echo "Killing geth"
    pkill geth
    fg
    #clean_db
}

cargo build

source run_eth.sh &

trap clean_up EXIT

cd ../eth_test/
npm install
cd -

echo "CREATE DATABASE test_database_only" | psql
cd sql/
psql test_database_only < all.sql
cd ..

echo "Running tests"
cargo test $1 -- --nocapture --test-threads=1
