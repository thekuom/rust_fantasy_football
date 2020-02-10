#! /bin/bash
set -a
source .env
set +a

DATABASE_URL=$DATABASE_URL_TEST diesel migration run
