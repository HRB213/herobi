# ------------------------------
# Stage 1. Build an app
# ------------------------------
FROM rust:1.96.0 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# ------------------------------
# Stage 2. Runtime
# ------------------------------
FROM debian:bookworm-slim

COPY --from=builder /app/target/release/herobi /app/herobi

WORKDIR /opt

ENTRYPOINT ["/app/herobi"]