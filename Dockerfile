FROM lukemathwalker/cargo-chef:latest-rust-bookworm AS chef

WORKDIR /usr/src
RUN apt-get update -y && apt-get upgrade -y && \
    apt-get clean && rm -rf /var/cache/apt/archives/* && rm -rf /var/lib/apt/lists/*

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /usr/src/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin litevec

FROM prantlf/healthchk as healthchk

FROM debian:bookworm-slim as runtime
LABEL maintainer="Ferdinand Prantl <prantlf@gmail.com>"

COPY --from=builder /usr/src/target/release/litevec /litevec
COPY --from=healthchk /healthchk /

WORKDIR /
EXPOSE 8000
ENTRYPOINT ["/litevec"]

ARG RUST_LOG=debug
ENV LITEVEC_AUTOSAVE_INTERVAL=10
ENV LITEVEC_COMPRESSION_LIMIT=1024
ENV LITEVEC_CORS_MAXAGE=86400
ENV LITEVEC_PAYLOAD_LIMIT=1073741824
ENV LITEVEC_PORT=8000
ENV LITEVEC_TIMEOUT=30
ENV RUST_LOG=${RUST_LOG}

# HEALTHCHECK --interval=5m \
#     CMD /healthchk -m HEAD http://localhost:8000/ping || exit 1
