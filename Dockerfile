
FROM rust:1.36.0

RUN export RUST_BACKTRACE=1 && \
    export http_proxy=http://proxy2.keylane.local:3128 && \
    export shttp_proxy=http://proxy2.keylane.local:3128 && \
    export ftp_proxy=ftp://proxy2.keylane.local:3128 && \
    export socks_proxy=socks5h://proxy2.keylane.local:3128 && \
    export HTTP_PROXY=http://proxy2.keylane.local:3128 && \
    export SHTTP_PROXY=http://proxy2.keylane.local:3128 && \
    export FTP_PROXY=ftp://proxy2.keylane.local:3128 && \
    export SOCKS_PROXY=socks5h://proxy2.keylane.local:3128 && \
    mkdir ".cargo" && \
    mkdir ".git" && \
    printf "[http]\nproxy = \"http://proxy2.keylane.local:3128\"\nsslVerify = false\ncheck-revoke = false\n\n" > ".cargo/config" && \
    printf "[https]\nproxy = \"http://proxy2.keylane.local:3128\"\nsslVerify = false\ncheck-revoke = false\n\n" >> ".cargo/config" && \
    printf "[http]\nproxy = \"http://proxy2.keylane.local:3128\"\nsslVerify = false\ncheck-revoke = false\n\n" > ".git/config" && \
    printf "[https]\nproxy = \"http://proxy2.keylane.local:3128\"\nsslVerify = false\ncheck-revoke = false\n\n" >> ".git/config" && \
    git config --global http.proxy proxy2.keylane.local:3128 && \
    git config --global https.proxy proxy2.keylane.local:3128

WORKDIR /code/gym/

COPY test_data.csv .
COPY Cargo.toml .

RUN mkdir "src" && echo "fn main() {}" > "src/main.rs"

RUN cargo build --release

COPY src/ ./src/

RUN cargo build --release

#CMD ["./target/release/rust_gym_1"]
CMD ["cargo", "run", "--release"]
