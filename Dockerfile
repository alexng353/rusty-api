FROM rust:1-alpine3.19 

WORKDIR /app

RUN apk add --no-cache musl-dev openssl-dev pkgconfig

COPY . .

RUN cargo build --release
