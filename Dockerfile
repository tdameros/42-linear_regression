FROM rust:1.78-bookworm as builder

COPY . /app/
WORKDIR /app/
RUN cargo clean && cargo build --bins --release

FROM debian:bookworm
RUN apt-get update && apt-get install -y \
    libfontconfig \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/train /app/target/release/predict /usr/local/bin/
WORKDIR /app/assets/

