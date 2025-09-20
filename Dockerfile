# Build web server
FROM rust:1.89 AS rust-builder
WORKDIR /server

COPY Cargo.toml Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

COPY src src
COPY migrations migrations
RUN cargo build --release --locked

# Build static site
FROM node:20 AS node-builder
WORKDIR /frontend

COPY frontend/package*.json .
RUN npm install

COPY frontend/ .
RUN npm run build

# Runtime
FROM debian:bookworm-slim
WORKDIR /konsfekt
RUN apt-get update && apt-get install -y libssl3 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=rust-builder /server/target/release/konsfekt /usr/local/bin/konsfekt
COPY --from=node-builder /frontend/build ./frontend/build
COPY migrations /konsfekt/migrations
CMD ["konsfekt"]
