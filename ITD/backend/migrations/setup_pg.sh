#!/bin/bash

if [ $PG_PASSWORD ] && [ $PG_USER ] && [ $PG_DB ]; then
    createdb ${PG_DB}
    psql ${PG_DB} -c "create user ${PG_USER} password '${PG_PASSWORD}'"
    psql ${PG_DB} -c "grant all privileges on database ${PG_DB} to ${PG_USER}"
    # psql ${PG_DB} -f init.sql
else
    echo "Please set the PG_DB, PG_USER, PG_PASSWORD environment variable"
fi