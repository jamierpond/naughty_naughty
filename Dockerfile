# 1. This tells docker to use the Rust official image
FROM rust:1.49

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# This is required otherwise rustup will use loads of ram when
# pulling down dependencies and fail mysteriously
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# add x86_64-unknown-linux-musl target
RUN rustup target add x86_64-unknown-linux-musl

# Install musl-tools
RUN apt-get update && apt-get install -y musl-tools

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/naughty_naughty"]
