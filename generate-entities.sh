#!/bin/sh

SQLITE_PATH=/tmp/migration.db
export DATABASE_URL=sqlite://$SQLITE_PATH

touch $SQLITE_PATH

sea-orm-cli migrate

sea-orm-cli generate entity \
    --output-dir ./entity/src/ \
    --with-serde both \
    --lib

rm $SQLITE_PATH
