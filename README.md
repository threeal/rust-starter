# Rust Starter

A minimal template for building a [Rust](https://www.rust-lang.org/) library with an optional CLI. Ships pre-configured with formatting, linting, 100% test coverage enforcement, pre-commit hooks, and CI.

## Getting Started

Create a new repository from this template on GitHub using [this link](https://github.com/new?template_name=rust-starter&template_owner=threeal), or clone it locally and point it at your own remote.

## Setup

Install [rustup](https://rustup.rs/), then install the toolchain pinned in `rust-toolchain.toml`:

```sh
rustup install
```

Install [cargo-tarpaulin](https://github.com/xd009642/tarpaulin) to run the test suite with coverage:

```sh
cargo install cargo-tarpaulin
```

Install [Lefthook](https://lefthook.dev/) and [dprint](https://dprint.dev/), then register the pre-commit hook:

```sh
lefthook install
```

## Customizing

Replace or extend the template files to fit your project:

- **`src/lib.rs`** — Replace with your own library logic.
- **`src/main.rs`** — Replace or remove the placeholder CLI. Remove this file and the `clap` dependency if your project doesn't need a CLI.
- **`Cargo.toml`** — Update the package name, description, version, authors, and other metadata.
- **`LICENSE`** — Replace with your preferred license, or keep the [Unlicense](https://unlicense.org/).
- **`README.md`** — Replace with a description of your project.

## Development

Write code in `src/`. Unit tests live in a `tests` module alongside the code they cover. Run the test suite with:

```sh
cargo tarpaulin
```

The project enforces 100% code coverage on every run.

Before committing, run the pre-commit hook to type-check and fix formatting and lint:

```sh
lefthook run pre-commit --all-files
```

If any file changes during the run, re-stage the changed files and retry. The hook also runs automatically on each `git commit` — if it fails, fix the reported issues, re-stage, and commit again.

After committing, push to `main` or open a pull request from another branch — CI will run the pre-commit hook across all files, the full test suite, and `cargo package`.

## Releasing

Update the version in `Cargo.toml`, push a version tag, and create a GitHub Release. To publish to [crates.io](https://crates.io/), run:

```sh
cargo publish
```
