FROM rust as builder
WORKDIR /app

RUN apt-get update
RUN apt-get upgrade

RUN cargo install cargo-watch

RUN apt-get install -y protobuf-compiler

EXPOSE 50052

CMD ["sh", "-c", "cargo run --package api --release"]