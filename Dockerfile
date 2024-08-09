# Build stage
FROM rust:bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Final run stage
FROM debian:bookworm-slim AS runner

WORKDIR /app
RUN apt update
RUN apt install -y libpq-dev
COPY --from=builder /app/target/release/genesis /app/genesis
COPY --from=builder /app/Rocket.toml /app
COPY --from=builder /app/diesel.toml /app
EXPOSE 8888
CMD ["/app/genesis"]
