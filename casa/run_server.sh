#!/usr/bin/env bash

set -e
set -o pipefail

clean_up () {
    echo "Killing geth"
    pkill geth
    echo "DROP DATABASE cambio_test" | psql
}


source run_eth.sh &

trap clean_up EXIT

echo "CREATE DATABASE cambio_test" | psql
cd sql/
psql cambio_test < all.sql
cd ..

echo "Running tests"
cargo run #$1 -- --nocapture --test-threads=1
