# Players API

## Overview
Provides an API to CRUD players and teams.

## Running Locally
Populate the values in `.env` from `.env.sample`.
Install the diesel cli:
```
$ cargo install diesel_cli --no-default-features --features "postgres"
```
Then setup the database:
```
$ ./scripts/setup_postgres.sh
$ diesel database setup
```
Then run
```
$ cargo run --bin main
```

## Running Seeds
```
cargo run --bin seed
```

## Running Tests
It is recommended to _not_ run tests in the Docker container
because it will take forever to wait for the application to
compile then run the tests. So if you are using a Docker environment,
make sure to port forward the postgres instance and set `DATABASE_URL_TEST` 
to the local URL for that database.
If running for the first time:
```
$ ./scripts/setup_postgres_test.sh
```
then
```
$ ./scripts/test.sh
```
