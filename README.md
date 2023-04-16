# Ordinal Type

This crate provides a type `Ordinal<T>`, that formats an integer as an ordinal number.

## Install

Command:

```sh
cargo add ordinal-type
```

Cargo.toml:

```toml
[dependencies]
ordinal-type = "*"
```

## Usage

```rust
use ordinal_type::Ordinal;

let ordinal = Ordinal(1);
assert_eq!(ordinal.to_string(), "1st");
```
