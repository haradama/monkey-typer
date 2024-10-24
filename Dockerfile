FROM rust:latest

RUN apt-get update && apt-get install -y \
    libxtst-dev \
    libx11-dev \
    libxi-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/monkey-typer

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN cargo build --release
