
FROM rust:1.36.0

RUN export RUST_BACKTRACE=1 && \
    export http_proxy=proxy1.keylane.local:3128 && \
    export shttp_proxy=proxy1.keylane.local:3128 && \
    export ftp_proxy=proxy1.keylane.local:3128 && \
    export HTTP_PROXY=proxy1.keylane.local:3128 && \
    export SHTTP_PROXY=proxy1.keylane.local:3128 && \
    export FTP_PROXY=proxy1.keylane.local:3128 && \
    mkdir ".cargo" && \
    printf "[http]\nproxy = \"http://localhost:3128\"\nsslVerify = false\ncheck-revoke = false\n\n" > ".cargo/config" && \
    printf "[https]\nproxy = \"https://localhost:3128\"\nsslVerify = false\ncheck-revoke = false\n\n" >> ".cargo/config" && \
    git config --global http.proxy proxy1.keylane.local:3128 && \
    git config --global https.proxy proxy1.keylane.local:3128

WORKDIR /code/gym/

COPY test_data.csv .
COPY Cargo.toml .

RUN mkdir "src" && echo "fn main() {}" > "src/main.rs"

RUN cargo build --release

COPY src/ ./src/

RUN cargo build --release

#CMD ["./target/release/rust_gym_1"]
CMD ["cargo", "run", "--release"]
