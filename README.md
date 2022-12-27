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
else
   cargo install c2rust
fi
pip install scan-build
cd -
export PATH=$HOME/.local/bin:$HOME/.cargo/bin:$PATH
cargo install --path .
```

## Usage:

Run `crusts` in the folder where there is a `Makefile`.

```bash
crusts
```

As a result, two subfolders will be created:
```
c2rust -- contains the transpiled Rust code from the C code;
crusts -- contains the refactored Rust code from the transpiled Rust code;
```

## Update

- [x] integrate with TXL
