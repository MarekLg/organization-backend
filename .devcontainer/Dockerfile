FROM rust:1-bookworm

RUN apt upgrade && apt update -y

# add rustfmt
RUN rustup component add rustfmt

# sqlx CLI
RUN cargo install sqlx-cli