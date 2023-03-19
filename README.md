# phoron_core

![github workflow](https://github.com/oyi-lang/phoron_asm/actions/workflows/rust.yml/badge.svg)
[<img alt="crates.io" src="https://img.shields.io/crates/v/phoron_core.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/phoron_core)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-phoron_core-b84432c?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/phoron_core)

This project provides the low-level functionality of interacting with the JVM, and thus it provides the following high-level features:

  - generating `class` files from the object rrpresentation, and
  - generating object representations from `class` files

## Build

  ```
    $ cargo build --release
    $ cargo test --release
  ```

## Usage

Refer to the tests.

## Planned Features

  - Pluggable support for custom Attributes.

## LICENCE

See [LICENSE](LICENSE).
