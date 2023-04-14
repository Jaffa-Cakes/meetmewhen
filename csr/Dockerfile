FROM rust as builder
WORKDIR /app

RUN apt-get update
RUN apt-get upgrade

RUN cargo install trunk --locked

RUN apt-get install -y protobuf-compiler

RUN rustup target add wasm32-unknown-unknown

EXPOSE 8080

CMD ["sh", "-c", "trunk serve ./csr/index.html -w ./client/ --release --address 0.0.0.0"]