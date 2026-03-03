FROM rust:1.92-bookworm AS builder
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli --locked
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY client/ client/
COPY src/ src/
RUN cargo build --release
RUN cargo run --release -- --build-only

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/stratego /app/stratego
COPY --from=builder /app/client/dist/ /app/static/
EXPOSE 8080
VOLUME /app/data
ENTRYPOINT ["/app/stratego", "--static-dir", "/app/static", "--data-dir", "/app/data"]
