# Setup database

We use postgresql as our database. we will use the slq cli create the databse and run migrations. We will need to install it before we use it.

## How to install

run this command to install the cli

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

## Initialization

This commands will create the database and then run the migrations that are stored in the migrations folder

```bash
sqlx database create
sqlx migrate run
```

## Create new migration

we use revertable migration. Sqlx cannot mix revertable and not revertable migration. They have the same name but they have a `.up.sql` or `.down.sql` extension

```bash
sqlx migrate add -r "migration name"
```

## Revert migration

the reason we use revertable migration are to be able to take back migration. You can do this with the command below. If you don't give the migration name the it will revert all migrations applied

```bash
sqlx migrate revert "migration name"
```
