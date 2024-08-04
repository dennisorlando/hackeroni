# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
FROM rust:alpine AS chef
WORKDIR /app
#RUN apt-get upgrade --available
#RUN apt-get install --no-cache bash postgresql-dev curl  openssl
#libressl-dev
#ENV RUSTFLAGS="-Ctarget-feature=-crt-static"
RUN apk add --no-cache curl postgresql-dev openssl-dev openssl-libs-static

#openssl openssl-dev alpine-sdk
#libressl-dev 
#openssl
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | ash
RUN cargo binstall cargo-chef -y

    
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
FROM alpine:latest AS runtime
WORKDIR /app
#RUN apk add --no-cache postgresql-dev
#openssl-libs-static
#RUN apt update &&  apt-get install libssl-dev libpq-dev ca-certificates curl -y && apt clean
#COPY /usr/local/share/ca-certificates/* > /usr/local/share/ca-certificates/
#RUN update-ca-certificates
COPY --from=builder /app/target/release/backend /app

#libressl-dev

ENTRYPOINT ["/app/backend"]

