# ---- Build stage ----
FROM rust:1.88.0-alpine as builder
WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev build-base

# Cache dependencies
COPY Cargo.toml Cargo.toml
COPY crates/api/Cargo.toml crates/api/Cargo.toml
COPY crates/domain/Cargo.toml crates/domain/Cargo.toml
COPY crates/storage/Cargo.toml crates/storage/Cargo.toml
COPY crates/storage_sqlite/Cargo.toml crates/storage_sqlite/Cargo.toml
COPY crates/storage_postgres/Cargo.toml crates/storage_postgres/Cargo.toml
    
# ---- Build the full app ----
COPY . .
RUN cargo build --release -p api

# ---- Runtime stage ----
FROM debian:bookworm-slim
WORKDIR /app

# Create a non-root user and group
RUN addgroup --system app && adduser --system --ingroup app app

COPY --from=builder /app/target/release/api /app/api
RUN chown app:app /app/api

USER app

EXPOSE 8080
CMD ["./api"]
