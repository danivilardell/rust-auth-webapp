#!/bin/bash

docker run -v $(pwd)/webapp_db/tests/test_db_init:/docker-entrypoint-initdb.d -e POSTGRES_PASSWORD=password -d -p 5433:5432 postgres
redis-server