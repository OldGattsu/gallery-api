# 1. Build stage
FROM rust:1.82 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# 2. Runtime stage (минимальный)
FROM debian:bookworm-slпm
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/gallery /app/gallery
ENV RUST_LOG=info
EXPOSE 3000
CMD ["/app/gallery"]