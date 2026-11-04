FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p server

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates sqlite3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server /app/generalbuffer

EXPOSE 3000

CMD ["./generalbuffer"]