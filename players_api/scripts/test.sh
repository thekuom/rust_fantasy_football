#! /bin/bash

if [ -f .env ]; then
  set -a
  source .env
  set +a
fi

# Reset database
DATABASE_URL=$DATABASE_URL_TEST diesel migration revert
while [ $? -eq 0 ]
do
  DATABASE_URL=$DATABASE_URL_TEST diesel migration revert
done

DATABASE_URL=$DATABASE_URL_TEST diesel migration run

cargo test -- --test-threads=5;
