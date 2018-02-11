#!/usr/bin/env bash

set -e
set -o pipefail

clean_up () {
    echo "Killing geth"
    pkill geth
    echo "DROP DATABASE test_database_only" | psql
}


source run_eth.sh &

trap clean_up EXIT

echo "CREATE DATABASE test_database_only" | psql
cd sql/
psql test_database_only < all.sql
cd ..

echo "Running tests"
cargo test  -- --nocapture
