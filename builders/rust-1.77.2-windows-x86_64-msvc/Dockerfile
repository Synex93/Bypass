FROM debian:bookworm-slim

ENV DEBIAN_FRONTEND=noninteractive
ENV RUST_VERSION=1.77.2
ENV RUST_TARGET=x86_64-pc-windows-msvc
ENV RUSTUP_HOME=/opt/rustup
ENV CARGO_HOME=/opt/cargo
ENV PATH=/opt/cargo/bin:$PATH

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    git \
    clang \
    lld \
    llvm \
    build-essential \
    pkg-config \
    python3 \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y \
        --profile minimal \
        --default-toolchain ${RUST_VERSION} \
        --target ${RUST_TARGET}

RUN rustup default ${RUST_VERSION} \
    && rustup target add ${RUST_TARGET}

RUN cargo install cargo-xwin --version 0.18.6 --locked

WORKDIR /workspace

COPY build.sh  /usr/local/bin/build
RUN chmod +x /usr/local/bin/build

ENTRYPOINT ["/usr/local/bin/build"]
