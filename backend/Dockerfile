# ---- Build stage ----
FROM rust:1.88.0-alpine as builder
WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev build-base

# Cache dependencies
COPY Cargo.toml Cargo.toml
COPY crates/engine/Cargo.toml crates/engine/Cargo.toml
COPY crates/parser/Cargo.toml crates/parser/Cargo.toml
COPY crates/search/Cargo.toml crates/search/Cargo.toml
COPY crates/storage/Cargo.toml crates/storage/Cargo.toml
COPY backend/Cargo.toml backend/Cargo.toml
    
# ---- Build the full app ----
COPY . .
RUN cargo build --release --manifest-path backend/Cargo.toml

# ---- Runtime stage ----
FROM debian:bookworm-slim
WORKDIR /app

# Create a non-root user and group
RUN addgroup --system app && adduser --system --ingroup app app

COPY --from=builder /app/target/release/backend /app/backend
RUN chown app:app /app/backend

USER app

EXPOSE 8080
CMD ["./backend"]