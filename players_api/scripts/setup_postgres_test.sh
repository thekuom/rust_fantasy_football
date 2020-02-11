#! /bin/bash
if [ -f .env ]; then
  set -a
  source .env
  set +a
fi

psql -h $DB_HOST_TEST -p $DB_PORT_TEST -U postgres -c "CREATE USER $DB_USER_TEST WITH PASSWORD '$DB_PASSWORD_TEST'" || true;

psql -h $DB_HOST_TEST -p $DB_PORT_TEST -U postgres -c "ALTER USER $DB_USER_TEST WITH SUPERUSER" || true;

psql -h $DB_HOST_TEST -p $DB_PORT_TEST -U postgres -c "CREATE DATABASE $DB_NAME_TEST WITH OWNER $DB_USER_TEST" || true;
