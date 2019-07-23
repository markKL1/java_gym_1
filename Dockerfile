
FROM rust:1.36.0

ENV RUST_BACKTRACE=1
ENV http_proxy=proxy1.keylane.local:3128
ENV shttp_proxy=proxy1.keylane.local:3128
ENV ftp_proxy=proxy1.keylane.local:3128

RUN mkdir "/root/.cargo" && \
    printf '[http]\nproxy = "http://localhost:3128"\nsslVerify = "false"\n\n[https]\nproxy = "https://localhost:3128"\nsslVerify = "false"\n\n' > "/root/.cargo/config"

WORKDIR /code/gym/

COPY test_data.csv .
COPY Cargo.toml .

RUN mkdir "src" && echo "fn main() {}" > "src/main.rs"

RUN cargo build --release

COPY src/ ./src/

RUN cargo build --release

CMD ["./target/release/rust_gym_1"]
