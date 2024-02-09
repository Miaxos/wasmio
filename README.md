# wasmio
S3 Backed Server for Wasmer

## Idea

Wasmer allow file storage with mounted volumes, with this kind of backend we'll
be able to implement applications based on a Storage.

We'll implement a S3 Server backed by a Volume from Wasmer.

## Development

You'll need to install [wasmer](https://wasmer.io) and also [cargo-wasix](https://github.com/wasix-org/cargo-wasix).

> Side note if you install `wasmer` on a Mac OS you'll need to add this to the
`.bash_profile` as it seems `wasmer` add it in the `.bashrc` instead.

``` bash
export WASMER_DIR="/Users/wizard/.wasmer"
[ -s "$WASMER_DIR/wasmer.sh" ] && source "$WASMER_DIR/wasmer.sh"
```

To run the solution locally, you can use this:

```bash
cargo wasix build --package wasmio
wasmer run . \
  --net \
  --enable-threads \
  --env CONFIG_FILE_LOCATION=/public/config.local.toml \
  --env RUST_LOG=info \
  --mapdir /public:$(pwd)/public
```

## Features

- [ ] API Endpoings for creating, listing, deleting objects
- [ ] Creation date / hash available
- [ ] Tests & e2e tests

## Benchmarks

(Integrate bencher here.)

## E2E Tests

(Integrate bencher here.)
