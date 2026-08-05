# FROM rust:1.97.1
FROM docker.io/library/rust@sha256:74182bc004bb9be325def279d21c72808917d6e4fb7d8651ecdb58d3205d88d2 as builder
RUN apt update && apt install libclang-dev libarchive-dev cmake -y
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN CARGO_BUILD_WARNINGS=deny cargo build --release --target x86_64-unknown-linux-gnu

# FROM gcr.io/distroless/cc-debian13:latest
FROM gcr.io/distroless/cc-debian13@sha256:ed7c407fd64eb0af9dddb9456b94cee188a40a7f53cf38c9836e1e9ae14fca02 AS release
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/woaf /woaf
CMD ["/woaf"]