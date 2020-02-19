#! /bin/bash
if [ -f .env ]; then
  set -a
  source .env
  set +a
fi

psql -h $DB_HOST -p $DB_PORT -U postgres -c "CREATE USER $DB_USER WITH PASSWORD '$DB_PASSWORD'" || true;

psql -h $DB_HOST -p $DB_PORT -U postgres -c "ALTER USER $DB_USER WITH SUPERUSER" || true;

psql -h $DB_HOST -p $DB_PORT -U postgres -c "CREATE DATABASE $DB_NAME WITH OWNER $DB_USER" || true;
