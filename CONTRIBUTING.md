## Setting up the environment

This repository contains a Rust crate generated from an OpenAPI specification by [Stainless](https://stainless.com).

Most of the code in this repository is generated. The `lib/` and `examples/` directories are never modified by the generator, so they are safe places for hand-written code.

### Prerequisites

Install a recent Rust toolchain (1.75 or later), for example with [rustup](https://rustup.rs):

```sh
rustup toolchain install stable
```

Or install the pinned tools with [Homebrew](https://brew.sh):

```sh
brew bundle
```

You can run `./scripts/bootstrap` to verify your toolchain and fetch dependencies.

## Modifying/adding code

Most of the SDK is generated, so direct modifications to generated files will be overwritten on the next generation. Add hand-written code under `lib/` or `examples/`.

## Adding and running examples

All files in the `examples/` directory are standalone runnable programs. Add a file such as `examples/my_example.rs`, register it in `Cargo.toml` under `[[example]]`, and run it with:

```sh
cargo run --example my_example
```

## Building and linting

```sh
./scripts/format   # cargo fmt
./scripts/lint     # cargo fmt --check && cargo clippy --all-targets -- -D warnings
./scripts/build    # cargo build --all-targets
```

## Running tests

The integration tests under `tests/` are smoke tests that run against a mock server seeded from the OpenAPI specification. They skip automatically when `TEST_API_BASE_URL` is not set, so `cargo test` is always safe to run.

To run them against a mock server:

```sh
# Start a mock server (defaults to http://localhost:4010)
./scripts/mock

# In another terminal, point the tests at it
TEST_API_BASE_URL=http://localhost:4010 ./scripts/test
```

## Releases

Releases are managed with [release-please](https://github.com/googleapis/release-please). Version bumps and changelog entries are derived from [Conventional Commits](https://www.conventionalcommits.org), so please format your commit messages accordingly.
