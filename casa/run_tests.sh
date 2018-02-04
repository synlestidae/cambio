#!/usr/bin/env bash

set -e
set -o pipefail

clean_up () {
    #echo "DROP DATABASE test_database_only" | psql
    echo "Killing geth"
    #pkill geth
}


trap clean_up EXIT

echo "CREATE DATABASE test_database_only" | psql
cd sql/
psql test_database_only < all.sql
cd ..

echo "Running tests"
#cargo test -- --nocapture
