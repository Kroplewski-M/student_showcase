# ===== DEV =====
FROM rust:1.90-slim-bullseye AS dev
WORKDIR /api
COPY . .
CMD ["cargo", "run"]


# ===== Build =====
FROM rust:1.90-slim-bullseye AS build

WORKDIR /api

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc

WORKDIR /api

COPY --from=build /api/target/release/api .

EXPOSE 8080

CMD ["./api"]

