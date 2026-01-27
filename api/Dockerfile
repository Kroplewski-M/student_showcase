# ===== DEV =====
FROM rust:1.90-slim-bookworm AS dev

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /api
COPY . .
CMD ["cargo", "run"]


# ===== Build =====
FROM rust:1.90-slim-bookworm AS build

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /api

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /api
COPY --from=build /api/target/release/api .

EXPOSE 8080
CMD ["./api"]
