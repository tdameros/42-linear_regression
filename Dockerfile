FROM rust:1.78-bookworm

COPY . /linear_regression/
WORKDIR /linear_regression

RUN cargo clean && cargo build --bins --release
RUN cp target/release/train target/release/predict .
