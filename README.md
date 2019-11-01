# Pandas Sports Service

[![Build Status](https://travis-ci.org/depaul-csc394-pandas/pandas-sports.svg?branch=master)](https://travis-ci.org/depaul-csc394-pandas/pandas-sports)

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

### API Routes

- `/api/matches`
    - `GET`: returns list of matches
    - `POST`: creates a new match (see `data-format.md`)
- `/api/matches/{id}`
    - `GET`: get a match by id
    - `DELETE`: delete a match
- `/api/teams`
    - `GET`: returns list of teams
    - `POST`: creates a new team (see `data-format.md`)
- `/api/teams/{id}`
    - `GET`: get a team by id
    - `DELETE`: delete a team (must have no matches)
