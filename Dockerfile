# syntax=docker/dockerfile:1

FROM rust:latest AS chef
RUN cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin blog

FROM debian:bookworm-slim AS final
WORKDIR app
COPY --from=builder /app/target/release/blog /usr/local/bin
ENTRYPOINT ["/usr/local/bin/blog"]
