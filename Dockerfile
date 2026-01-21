# Builder
FROM rust:latest AS builder
WORKDIR /app

COPY . .

RUN cargo build --release

# Runtime
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && \
    apt-get install -y iputils-ping && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/shamash /usr/local/bin/
RUN mkdir -p /app/shamash-logs
CMD ["shamash"]
