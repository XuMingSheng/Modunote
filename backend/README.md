# Backend Development

Run commands from `backend/`. `COMPOSE_PROJECT_NAME` defaults to `backend`.

## Migrations

### SQLite

- `just migrate-sqlite-dev`

### Postgres

- `just postgres-dev-up`
- `just migrate-postgres-dev`
- `just postgres-dev-reset`

## Build

- `just build` to run `sqlx prepare` for both DB crates (uses the dev DB URLs) and compile.

## Tests

### SQLite

- `just test-sqlite` to create a temporary sqlite test DB, run migrations, execute sqlite contract tests, then clean up the DB file.

### Postgres

- `just test-postgres` to spin up the test container, run postgres contract tests, then tear it down.
- `just postgres-test-up` to start the postgres test container when you want it running across multiple test runs.
- `just postgres-test-down` to stop and remove the postgres test container.
- `just postgres-test-reset` to remove the postgres test container and its test volume when you need a clean slate.
