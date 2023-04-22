FROM rust as builder
WORKDIR /app

RUN apt-get update
RUN apt-get upgrade

RUN cargo install trunk --locked

RUN apt-get install -y protobuf-compiler

RUN rustup target add wasm32-unknown-unknown

EXPOSE 80

CMD ["sh", "-c", "trunk build ./csr/index.html --release && cargo run --package ssr --release"]