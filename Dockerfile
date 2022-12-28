FROM rust:1.66 AS build
RUN apt update
RUN apt install llvm -y
RUN apt install cmake -y
RUN apt install clang -y
RUN apt install libclang-dev -y
ENV LLVM_LIB_DIR /usr/lib/llvm-11/lib
RUN cargo install c2rust
RUN apt install bear -y
RUN rustup component add rustfmt --toolchain nightly-2021-11-22-x86_64-unknown-linux-gnu
RUN git clone https://github.com/yijunyu/crusts \
 && cd crusts \
 && cargo install --path .
WORKDIR /mnt
CMD ["crusts"]
