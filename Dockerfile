
FROM rust:1.36.0

WORKDIR /code/gym/

COPY test_data.csv .
COPY Cargo.toml .
COPY src/ ./src/

RUN cargo build --release

RUN find .

CMD ["./target/release/rust_gym_1"]
