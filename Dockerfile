
FROM rust:1.36.0

ENV RUST_BACKTRACE=1

WORKDIR /code/gym/

COPY test_data.csv .
COPY Cargo.toml .

RUN mkdir "src" && echo "fn main() {}" > "src/main.rs"

RUN cargo build --release

COPY src/ ./src/

RUN cargo build --release

CMD ["./target/release/rust_gym_1"]
