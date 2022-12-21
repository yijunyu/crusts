# CRustS - Transpiling Unsafe C code to Safe Rust

## Installation

```bash
cargo install c2rust
pip install scan-build
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
