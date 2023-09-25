Running database migrations locally:

```sh
    SKIP_DOCKER=true ./scripts/init_db.sh
```

Running database migrations on Digital Ocean (temporarily disable Trusted Sources to proceed):

```sh
    DATABASE_URL=<YOUR-DIGITAL-OCEAN-DB-CONNECTION-STRING> sqlx migrate run
```