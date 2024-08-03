# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
FROM rust:alpine3.20 AS chef
WORKDIR /app
RUN apk upgrade --available
RUN apk add --no-cache bash postgresql-dev curl  openssl
#libressl-dev
#ENV RUSTFLAGS="-Ctarget-feature=-crt-static"
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-chef -y

    #apk add --virtual build-deps gcc python-dev musl-dev && \
    #apk add 
#RUN apt-get update && apt-get install -yq libssl-dev

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json


# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM alpine:3.20 AS runtime
WORKDIR /app
#RUN apt-get update && apt-get install -yq libpq-dev libc6 glibc-source
RUN apk add --no-cache postgresql-dev 
#libressl-dev
COPY --from=builder /app/target/release/backend /app
ENTRYPOINT ["/app/backend"]