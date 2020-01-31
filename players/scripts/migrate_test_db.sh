#! /bin/sh
DATABASE_URL=postgres://postgres:password@postgres:5432/players_test diesel migration run
