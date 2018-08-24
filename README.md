# quickcheck_derive

<!-- badges -->
[![Travis (.org)](https://img.shields.io/travis/panicbit/quickcheck_derive.svg)](https://travis-ci.org/panicbit/quickcheck_derive)
[![Crates.io](https://img.shields.io/crates/v/quickcheck_derive.svg)](https://crates.io/crates/quickcheck_derive)

This crate adds a derive for the `Arbitrary` trait from the `quickcheck` crate.

# Usage

```rust
#[macro_use]
extern crate quickcheck_derive;
extern crate quickcheck;

#[derive(Arbitrary,Clone)]
struct Data {
    foo: i32,
    bar: String,
}
```
