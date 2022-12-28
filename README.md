# CRustS - Transpiling Unsafe C code to Safe Rust

## Installation

```bash
# install c2rust
if [ $(uname -s) == "Darwin" ]; then
   git clone https://github.com/immunant/c2rust 
   cd c2rust
   scripts/provision_mac.sh
   cargo build --release
   cp target/release/c2rust $HOME/.cargo/bin
   cp target/release/c2rust-transpile $HOME/.cargo/bin
   cp target/release/c2rust-analyze $HOME/.cargo/bin
   cp target/release/c2rust-instrument $HOME/.cargo/bin
   brew install bear
   export PATH=$HOME/.local/bin:$HOME/.cargo/bin:$PATH
   cargo install --path .
elif [ $(uname -s) == "Linux" ]; then
   cargo install c2rust
   apt-get install bear
   export PATH=$HOME/.local/bin:$HOME/.cargo/bin:$PATH
   cargo install --path .
else
   docker pull yijun/crusts
fi
```

## Usage:

Run `crusts` in the folder where there is a `Makefile`.

```bash
crusts
```
or 
```bash
docker run -v $(pwd):/mnt -it yijun/crusts
```

As a result, Rust code will be generated from the C code:
```
src/*.rs -- contains the refactored Rust code from the C code;
build.rs lib.rs -- contains the builder Rust code;
```

## Update

- [x] integrate with TXL
- [x] integrate with docker
