FROM rust:1.87.0 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/simple-rust-server /usr/local/bin/server
EXPOSE 8300
CMD ["/usr/local/bin/server"]