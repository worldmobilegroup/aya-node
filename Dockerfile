# Use a specific nightly version to ensure consistency
FROM rustlang/rust:nightly

WORKDIR /aya

# Install system dependencies
RUN apt-get update -y && \
    apt-get install -y cmake pkg-config libssl-dev git gcc build-essential clang libclang-dev protobuf-compiler

# Install Rust targets and components
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rust-src

# Clone the repository
RUN git clone https://github.com/worldmobilegroup/aya-node /aya
WORKDIR /aya
RUN git submodule update --init

# ARG should be declared before its usage
ARG PROFILE=release
RUN cargo build --${PROFILE}

# Expose necessary ports
EXPOSE 30333 9933 9944 9615

# Set environment variables
ENV RUST_LOG=info,cargo=warn

# Command to run the node with additional flags for broader RPC access and more detailed logging
CMD ["cargo", "run", "--bin", "aya-node", "--", "--dev", "--rpc-methods=Unsafe", "--rpc-external", "--unsafe-rpc-external", "--log", "rpc", "--ws-external", "--rpc-cors", "all"]
