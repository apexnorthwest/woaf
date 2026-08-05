FROM docker.io/library/rust:1.97.1 as builder
RUN apt update && apt install libclang-dev libarchive-dev cmake -y
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN CARGO_BUILD_WARNINGS=deny cargo build --release --target x86_64-unknown-linux-gnu

FROM gcr.io/distroless/cc-debian13:latest AS release
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/woaf /woaf
CMD ["/woaf"]