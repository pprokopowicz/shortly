# Shortly

Simple url shortener written in Rust, with [axum](https://github.com/tokio-rs/axum) and [sqlx](https://github.com/launchbadge/sqlx). Postgres database used for data persistence.

## Before running

Before running, create a `.env` file with key `DATABASE_URL`.

```console
foo@bar:~$ echo "DATABASE_URL=<YOUR_DATABASE_URL>" > .env
foo@bar:~$ cargo run
```

## Usage

Services exposes 4 different routes.

1. get `/api/entries` - returns all entries from the database.
2. post `/api/entries` - creates a new entry.
3. get `/api/entries/{external_id}` - fetches one entry by external id.
4. get `/{external_id}` - redirects to a url based on external id.