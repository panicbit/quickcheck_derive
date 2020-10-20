# NOTICE

**Before using this crate, please evaluate [quickcheck's official derive crate](https://crates.io/crates/quickcheck_macros).**

# quickcheck_derive

<!-- badges -->
[![Travis (.org)](https://img.shields.io/travis/panicbit/quickcheck_derive.svg)](https://travis-ci.org/panicbit/quickcheck_derive)
[![Crates.io](https://img.shields.io/crates/v/quickcheck_derive.svg)](https://crates.io/crates/quickcheck_derive)

This crate adds a derive for the `Arbitrary` trait from the `quickcheck` crate.

# Requirements
You need the following dependencies in your `Cargo.toml`:

- `quickcheck` >= 0.7
- `rand`

# Usage

```rust
#[macro_use]
extern crate quickcheck_derive;

#[derive(Arbitrary,Clone)]
struct Data {
    foo: i32,
    bar: String,
}
```
