# wasmio
S3 Backed Server for Wasmer

## Idea

Wasmer allow file storage with mounted volumes, with this kind of backend we'll
be able to implement the S3 APIs.

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

## TODO

- Error management, quite crappy right now, abstract ressource & request_id and
impl proper From to have easy way to propagate errors.
