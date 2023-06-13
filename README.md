# Rust Cross Platform

This is an example for a cross-platform library in rust.
It is intended to be used for embedded and modern systems.

Every platform implements a wrapper of basic functions the
platform (usually kernel) provides. For example unix wraps libc.

These wrappers are the Platform Abstraction Layer - pal
the pal creates a uniform API for all platforms allowing the 
implementation of cross-platform high level objects.


# Usage

[`main.rs`](src/main.rs) includes an example testing checking the `pal::file` module.

## Execution

`cargo build --features <platform_name>`

or build + execute directly with:

`cargo run --features <platform_name>`.

Currently `unix` is a supported platform name.

> Note: dependencies are installed based on your platform using `cfg-if`

## Utilities

* `cargo clippy` # rust's clang-tidy
* `cargo fmt` # rust's clang-format
* `cargo doc` # generates documentation in target/doc

# Future (TODO)

* Check if possible to head forward declaration thing in order to have better
IDE completions for pal functions
* Implement file module for windows