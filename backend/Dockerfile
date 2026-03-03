# 1. Start with the chef image
FROM lukemathwalker/cargo-chef:latest-rust-1.80-bookworm AS chef
WORKDIR /app
RUN apt-get update && apt-get install -y lld clang pkg-config libssl-dev

# 2. Planner: Prepare the "recipe"
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 3. Builder: Compile dependencies only
FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# This layer is cached until you change a dependency in Cargo.toml
RUN cargo chef cook --release --recipe-path recipe.json

# Now copy the actual source code and build the app
COPY . .
RUN cargo build --release -p api

# 4. Runtime: Keep it tiny
# Note: Using alpine for both stages makes life easier with musl/libc compatibility
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -ms /bin/bash app
USER app

# Copy binary and configs
COPY --from=builder --chown=app:app /app/target/release/api /app/api
COPY --chown=app:app ./configs /app/configs

EXPOSE 8080
CMD ["./api"]
