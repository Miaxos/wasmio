# wasmio
S3 Backed Server for Wasmer

## Development

Need:
- wasmer
- cargo install cargo-wasix

```
cargo wasix run --package wasmio
wasmer run . --net --enable-threads --env CONFIG_FILE_LOCATION=/public/config.local.toml 
```

## Features

- [ ] API Endpoings for creating, listing, deleting objects
- [ ] Creation date / hash available
- [ ] Tests & e2e tests
