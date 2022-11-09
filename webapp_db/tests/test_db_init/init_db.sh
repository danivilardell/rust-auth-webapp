#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username postgres <<-EOSQL

    CREATE DATABASE webapp;
    \c webapp
    CREATE TABLE IF NOT EXISTS users (
      username text not null,
      password text not null
    );
    CREATE ROLE webapp_test WITH LOGIN PASSWORD 'webapp-test-password';
    GRANT CONNECT ON DATABASE webapp TO webapp_test;
    GRANT ALL PRIVILEGES ON TABLE users TO webapp_test;
    insert into users (username, password) values ('test_user', 'test_password');

EOSQL

#docker run -v $(pwd)/webapp_db/tests/test_db_init:/docker-entrypoint-initdb.d -e POSTGRES_PASSWORD=password -d -p 5433:5432 postgres
