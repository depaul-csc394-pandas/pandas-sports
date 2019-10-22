# Pandas Sports Service

### Setup

- Install PostgreSQL
  ```
  # apt update
  # apt install postgresql postgresql-contrib
  ```
- [Install rustup](https://rustup.rs/)
- Install Diesel CLI
  ```
  $ cargo install diesel\_cli --no-default-features --features postgres
  ```
- Copy `.env.example` to `.env` and edit the DATABASE_URL variable with your
  Postgres username and password. Make sure to use a Postgres superuser role.
- Set up the database
  ```
  $ diesel database setup
  ```
- Build and run with
  ```
  $ cargo run --release
  ```
