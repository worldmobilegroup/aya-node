FROM rustlang/rust:nightly as builder

ARG PROFILE=release
ARG VERSION=devnet-v.0.2.0
WORKDIR /aya

RUN apt-get update -y && \
        apt-get install -y cmake pkg-config libssl-dev git gcc build-essential clang libclang-dev protobuf-compiler

# Install rust wasm. Needed for substrate wasm engine
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN rustup target add wasm32-unknown-unknown --toolchain nightly-x86_64-unknown-linux-gnu
# Download aya-node repo
RUN git clone https://github.com/worldmobilegroup/aya-node /aya
RUN cd /aya && git submodule init && git submodule update

# Download chain info into /aya folder
RUN wget -O chainspec.json https://github.com/worldmobilegroup/aya-node/releases/download/${VERSION}/wm-devnet-chainspec.json

RUN cargo build "--$PROFILE"

# (Optional) Remove debug symbols
RUN strip ./target/release/aya-node

FROM cgr.dev/chainguard/wolfi-base

RUN apk update && \
    apk add libstdc++

COPY --from=builder aya/target/release/aya-node /target/release/aya-node
COPY --from=builder aya/chainspec.json .

# 30333 for p2p traffic
# 9933 for RPC call
# 9944 for Websocket
# 9615 for Prometheus (metrics)
EXPOSE 30333 9933 9944 9615

ENV PROFILE ${PROFILE}
CMD ["target/release/aya-node"]
