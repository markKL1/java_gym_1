FROM rust:1.36.0
WORKDIR /code/gym/
COPY Cargo.toml .
RUN mkdir "src" && echo "fn main() {}" > "src/main.rs"
RUN cargo build --release
COPY src/ ./src/
RUN cargo build --release
CMD ["cargo", "run", "--release"]
