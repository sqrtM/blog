# syntax=docker/dockerfile:1

# Stage 1: Build the Rust application
FROM rust:latest AS chef
RUN cargo install cargo-chef
WORKDIR app

# Stage 2: Prepare the application dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build the application dependencies
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin blog

# Stage 4: Create the final image for your web application
FROM fedora:latest AS web-app
COPY --from=builder /app/target/release/blog /usr/local/bin

# Specify the command to run your web application
CMD ["/usr/local/bin/blog"]
