## A simple Rust library about expressions, mathematics, matrices, etc.
- [`rexl`](https://github.com/jrrwll/rexl/tree/master/rexl)
- [`rexl-math`](https://github.com/jrrwll/rexl/tree/master/rexl-math)
- [`rexl-matrix`](https://github.com/jrrwll/rexl/tree/master/rexl-matrix)

## Development

```shell
cargo package -p rexl_macros --list
cargo package -p rexl_macros

# https://crates.io/settings/tokens
cargo login

cargo publish -p rexl_macros --dry-run
cargo publish -p rexl_macros
```
