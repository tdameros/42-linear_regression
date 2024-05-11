FROM rust:1.78-bookworm

COPY . /linear_regression/
WORKDIR /linear_regression

RUN cargo build --bin train --release
RUN cargo build --bin predict --release
RUN cp target/release/train target/release/predict .
