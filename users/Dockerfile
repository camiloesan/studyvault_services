FROM rust:latest AS builder

EXPOSE 8083

RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev openssl libssl-dev

WORKDIR /usr/src/studyvault_services

COPY Cargo.toml Cargo.lock ./
COPY auth/Cargo.toml auth/
COPY channels/Cargo.toml channels/
COPY comments/Cargo.toml comments/
COPY data_access/Cargo.toml data_access/
COPY posts/Cargo.toml posts/
COPY subscriptions/Cargo.toml subscriptions/
COPY users/Cargo.toml users/
COPY users/.env users/.env

COPY . .

RUN cargo build -p users --release

FROM ubuntu:22.04

RUN apt-get update && apt-get install -y ca-certificates

COPY --from=builder /usr/src/studyvault_services/target/release/users /usr/local/bin/users

CMD ["users"]
