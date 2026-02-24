# ===== DEV =====
FROM rust:1.90-slim-trixie AS dev

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    g++ \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install sqlx-cli \
    --no-default-features \
    --features postgres

WORKDIR /api
COPY . .
CMD ["cargo", "run"]

# ===== Build =====
FROM rust:1.90-slim-trixie AS build

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    g++ \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /api
COPY . .
RUN cargo build --release

# ===== Runtime =====
FROM debian:trixie-slim
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /api
COPY --from=build /api/target/release/api .
COPY --from=build /api/templates ./templates

EXPOSE 8080
CMD ["./api"]
