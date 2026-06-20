
### making-executables-smaller
```sh
RUSTFLAGS='-C link-args=-s' cargo build --release

rustc -O -C prefer-dynamic main.rs

# remove symbol information
/usr/bin/strip target/release/some_bin

# compress
upx --best --lzma target/release/some_bin
```
