# Rust-auth-webapp
Basic webapp that uses postgres db, redis server and fred as a client and docker for testing purposes.

### Basic setup
First of all copy the environmental variables to a ```.env``` file

```cp local.env .env```

### Testing database setup
Install docker and run the following command from the root folder of the application.

```docker run -v $(pwd)/webapp_db/tests/test_db_init:/docker-entrypoint-initdb.d -e POSTGRES_PASSWORD=password -d -p 5433:5432 postgres```