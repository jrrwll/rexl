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