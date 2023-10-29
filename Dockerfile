FROM rust:1.70

RUN apt update && apt -y install qemu-system-x86 qemu-utils ovmf && \
    rustup install nightly && \
    rustup default nightly && \
    rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu