echo "CREATE DATABASE test_database_only" | psql
cd sql/
psql test_database_only < all.sql
cd ..
cargo test
echo "DROP DATABASE test_database_only" | psql