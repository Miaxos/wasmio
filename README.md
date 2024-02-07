# wasmio
S3 Backed Server for Wasmer

## Development

Need:
- wasmer
- cargo install cargo-wasix

```
cargo wasix run --package wasmio
wasmer run . --net --enable-threads --env CONFIG_FILE_LOCATION=/public/config.local.toml 
cargo wasix build --package wasmio && wasmer run . --net --enable-threads --env CONFIG_FILE_LOCATION=/public/config.local.toml --env RUST_LOG=info
```

## Features

- [ ] API Endpoings for creating, listing, deleting objects
- [ ] Creation date / hash available
- [ ] Tests & e2e tests
