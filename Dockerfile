# Stage 1: Build the Rust project
FROM rustlang/rust:nightly as builder

ARG PROFILE=release
WORKDIR /aya

# Update core parts and install necessary packages
RUN apt-get update -y && \
    apt-get install -y cmake pkg-config libssl-dev git gcc build-essential clang libclang-dev protobuf-compiler

# Install rust wasm. Needed for substrate wasm engine
RUN rustup target add wasm32-unknown-unknown
# Install rust-src for the standard library sources
RUN rustup component add rust-src --toolchain nightly

# Download Frontier repo
RUN git clone https://github.com/worldmobilegroup/aya-node /aya
RUN cd /aya && git submodule init && git submodule update

# Download rust dependencies and build the rust binary
RUN cargo build "--$PROFILE"

# Stage 2: Create a minimal image with the built binary
FROM rustlang/rust:nightly

WORKDIR /aya
COPY --from=builder /aya/target/release/aya-node /usr/local/bin/aya-node

# 30333 for p2p traffic
# 9933 for RPC call
# 9944 for Websocket
# 9615 for Prometheus (metrics)
EXPOSE 30333 9933 9944 9615

ENV PROFILE ${PROFILE}

# The execution will run the compiled binary
CMD aya-node --dev
