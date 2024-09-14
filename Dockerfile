FROM rust:latest

RUN apt-get update && \
    apt-get install -y \
    gcc-multilib \
    libc6-dev-i386

RUN rustup target add x86_64-unknown-linux-gnu

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release --target=x86_64-unknown-linux-gnu

CMD ["cargo", "run"]
