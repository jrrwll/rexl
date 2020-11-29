### Get start
```sh
cargo new foobar --lib
cargo check

cargo build
cargo build --release
cargo run

# build docs and 
cargo doc --open
```

### Doc comment
```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```


### Publishing on crates.io

- ##### Before your first publish
```sh
# save to ~/.cargo/credentials
cargo login abcdefghijklmnopqrstuvwxyz012345
```

- ##### Packaging a crate
```sh
cargo package --list
cargo package
```

- ##### Uploading the crate
```sh
cargo publish
```