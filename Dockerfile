FROM rust:1.66 AS build
RUN apt update
RUN apt install llvm -y
RUN apt install cmake -y
RUN apt install clang -y
RUN apt install libclang-dev -y
ENV LLVM_LIB_DIR /usr/lib/llvm-11/lib
RUN cargo install c2rust
RUN cargo install txl-rs \
 && cargo init --bin abc \
 && cd abc \
 && txl-rs src/main.rs \
 && cd txl10.8b.linux64 \
 && ./InstallTxl \
 && cd .. \
 && crusts \
 && cp txl10.8b.linux64/lib/Rust/* /usr/local/lib/txl/
RUN apt install bear -y
RUN rustup component add rustfmt
RUN git clone https://github.com/yijunyu/crusts \
 && cd crusts \
 && cargo install --path .
WORKDIR /mnt
CMD ["crusts"]
