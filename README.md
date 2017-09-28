
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
