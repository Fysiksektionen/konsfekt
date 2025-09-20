# Build
FROM rust:1.89 AS builder
WORKDIR /konsfekt

COPY Cargo.toml Cargo.lock .

RUN mkdir src && touch src/lib.rs \
    && cargo build --release \
    && rm -rf src

COPY . .

RUN cargo build --release

WORKDIR /konsfekt/frontend
RUN npm run build

# Runtime
FROM debian:bookworm-slim
WORKDIR /konsfekt
COPY --from=builder /target/release/konsfekt /usr/local/bin/konsfekt
CMD ["konsfekt"]
