[![dependency status](https://deps.rs/repo/github/be-next/rust-lab/status.svg?path=%2F)](https://deps.rs/repo/github/be-next/rust-lab?path=%2F)

# 🔬 rust-lab

Experiments, tests, and insights on Rust and its ecosystem.

All experimentations are documented in the `topics` directory, organized by topic (see below).

## Topics

### Basics

- [Ownership, References and Borrowing in Rust](topics/rust-ownership-references-borrowing/README.md): the core concepts of Rust's memory management system.
- [Error Handling in Rust](topics/rust-error-handling/README.md): different ways to handle errors in Rust. From native error handling to custom error types.
- [Smart Pointers in Rust](topics/rust-smart-pointers/README.md): a taxonomy of smart pointers in Rust and examples of their usage.
- [Concurrent programming in Rust](topics/rust-concurrent-programming/README.md): some examples and exploration of concurrent programming in Rust (huge topic!).

### Crates

- [axum and opentelemetry](topics/crate-axum-opentelemetry/README.md): an example of how to use `axum` and `opentelemetry` together. Traces are visualized in Jaeger.

## Utilities

### Bacon

[`bacon`](https://crates.io/crates/bacon) is an automatic monitoring tool for Rust (but not only), which detects changes in your source code and automatically restarts builds, tests or other commands. It is particularly useful for code experimentation and TDD development.

To install `bacon`, run the following command:

```bash
cargo install bacon
```

To run `bacon` with a specific example file, run the following command:

```bash
bacon run -- -q --example <rust-example-file>
```

### Cargo outdated

[`cargo outdated`](https://crates.io/crates/cargo-outdated) is a cargo subcommand for displaying when Rust dependencies are out of date.

```bash
cargo outdated
```

### Cargo upgrade

[`cargo upgrade`](https://crates.io/crates/cargo-edit) is a tool to help you keep your Rust projects up-to-date. It can automatically update your `Cargo.toml` file with the latest versions of your dependencies.

To install `cargo-edit`, run the following command:

```bash
cargo install cargo-edit
```

To update the dependencies in `Cargo.toml` file, run the following command:

```bash
cargo upgrade
```

> [!IMPORTANT]
> To force update the dependencies (while major update or breaking change), run the following command:
>
>```bash
>cargo upgrade --incompatible
>```
