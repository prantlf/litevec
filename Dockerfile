FROM lukemathwalker/cargo-chef:latest-rust-bookworm AS chef
WORKDIR /litevec

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /litevec/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin litevec

FROM debian:bookworm-slim as runtime
WORKDIR /litevec
COPY --from=builder /litevec/target/release/litevec /usr/local/bin

EXPOSE 8000
VOLUME ["/storage"]
ENTRYPOINT ["/usr/local/bin/litevec"]

HEALTHCHECK --interval=5m \
    CMD curl -f http://localhost:8000/ || exit 1
