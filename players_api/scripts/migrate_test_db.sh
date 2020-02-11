#! /bin/bash
if [ -f .env ]; then
  set -a
  source .env
  set +a
fi

DATABASE_URL=$DATABASE_URL_TEST diesel migration run
