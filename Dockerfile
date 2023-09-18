# syntax=docker/dockerfile:1

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

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

FROM gcr.io/distroless/cc as final
WORKDIR /app
COPY --from=builder /app/target/release/blog /usr/local/bin

# Configure rocket to listen on all interfaces.
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_SECRET_KEY=f76mdAwq36kBh6eW//rsioZ3QbvB9iy3atef8tXDFdc=
ENV POSTGRES_PASSWORD=password
ENV POSTGRES_USER=postgres

CMD ["/usr/local/bin"]
