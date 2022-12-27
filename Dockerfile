FROM rust
WORKDIR /app
COPY . .
RUN cargo build
ENTRYPOINT ["cargo","run"]