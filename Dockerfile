FROM rust:1.83-slim-bookworm as chef

RUN cargo install cargo-chef

WORKDIR /app
COPY Cargo.* .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

WORKDIR /app
COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE true

ARG BIN
RUN if [ -z "$BIN" ]; then echo "BIN argument is not set"; exit 1; fi

RUN cargo build --release --bin $BIN

FROM debian:bookworm-slim AS runner

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/$BIN $BIN
COPY configuration configuration

ENTRYPOINT ["./$BIN"]
