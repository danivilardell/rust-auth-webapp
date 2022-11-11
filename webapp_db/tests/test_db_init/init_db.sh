#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username postgres <<-EOSQL

    CREATE DATABASE webapp;
    \c webapp
    CREATE ROLE webapp_test WITH LOGIN PASSWORD 'webapp-test-password';
    GRANT CONNECT ON DATABASE webapp TO webapp_test;
    CREATE TABLE IF NOT EXISTS users (
      username text not null,
      password text not null
    );
    insert into users (username, password) values ('test_user', 'test_password');

    CREATE TYPE valid_activity AS ENUM ('run', 'swim', 'bike');
    CREATE TABLE IF NOT EXISTS activities (
          activity_type valid_activity not null,
          date text not null,
          username text not null
        );
        insert into activities (activity_type, date, username) values ('run', 'test_date', 'test_user');

    GRANT ALL PRIVILEGES ON TABLE users TO webapp_test;
    GRANT ALL PRIVILEGES ON TABLE activities TO webapp_test;

EOSQL
