FROM lukemathwalker/cargo-chef:latest-rust-alpine3.24 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN cargo build --release

FROM dhi.io/static:20260909-alpine AS runtime
COPY --from=builder /app/target/release/modulaga-config-api /modulaga-config-api

EXPOSE 8080

ENTRYPOINT ["/modulaga-config-api"]