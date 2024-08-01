# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
FROM clux/muslrust:stable AS chef
WORKDIR /app
#ENV RUSTFLAGS="-Ctarget-feature=-crt-static"
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-chef -y
#RUN apt-get update && apt-get install -yq libssl-dev

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json


# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -yq libpq-dev libc6 glibc-source
COPY --from=builder /app/target/release/backend /app
ENTRYPOINT ["/app/backend"]