# wasmio
S3 Backed Server for Wasmer

```
 ▄     ▄ ▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄   ▄▄ ▄▄▄ ▄▄▄▄▄▄▄ 
█ █ ▄ █ █      █       █  █▄█  █   █       █
█ ██ ██ █  ▄   █  ▄▄▄▄▄█       █   █   ▄   █
█       █ █▄█  █ █▄▄▄▄▄█       █   █  █ █  █
█       █      █▄▄▄▄▄  █       █   █  █▄█  █
█   ▄   █  ▄   █▄▄▄▄▄█ █ ██▄██ █   █       █
█▄▄█ █▄▄█▄█ █▄▄█▄▄▄▄▄▄▄█▄█   █▄█▄▄▄█▄▄▄▄▄▄▄█
```

[![release](https://github.com/Miaxos/wasmio/actions/workflows/release.yml/badge.svg)](https://github.com/Miaxos/wasmio/actions/workflows/release.yml)
[![Crates.io version](https://img.shields.io/crates/v/wasmio.svg)](https://crates.io/crates/wasmio)
[![dependency status](https://deps.rs/repo/github/miaxos/wasmio/status.svg)](https://deps.rs/repo/github/miaxos/wasmio)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/miaxos/wasmio/compare)


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

### Standard

```bash
RUST_LOG=info CONFIG_FILE_LOCATION=./public/config.local.toml cargo run --package wasmio RUST_LOG=info
```

### Wasmer

```bash
cargo wasix build --package wasmio
wasmer run . \
  --net \
  --enable-threads \
  --env CONFIG_FILE_LOCATION=/public/config.local.toml \
  --env RUST_LOG=info \
  --mapdir /public:$(pwd)/public
```

## Performance

Right now, we are using a simple JSON to store the data, which is not efficient
for files based storage. This is just an experiment on the whole storage and
wasmer.

To have better performance, the storage layer should be reworked to better use
Files APIs (with Seek for instance, a better format to store data, we don't need
JSON).

## TODO

- [ ] API Endpoings for creating, listing, deleting objects
- [ ] Creation date / hash available
- [ ] Deploy
- [ ] e2e tests

## Benchmarks

(Integrate bencher here.)

## E2E Tests

(Integrate bencher here.)
