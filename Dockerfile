FROM rust:1.70

RUN apt update && apt -y install qemu-system-x86 qemu-utils ovmf