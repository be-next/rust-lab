[![dependency status](https://deps.rs/repo/github/be-next/rust-lab/status.svg?path=%2F)](https://deps.rs/repo/github/be-next/rust-lab?path=%2F)

# 🔬 rust-lab

Experiments, tests, and insights on Rust and its ecosystem.

All experimentations are documented in the `topics` directory, organized by topic (see below).

## Topics

- [Error Handling in Rust](topics/rust-error-handling/README.md): different ways to handle errors in Rust. From native error handling to custom error types.
- [Smart Pointers in Rust](topics/rust-smart-pointers/README.md): a taxonomy of smart pointers in Rust and examples of their usage.
- [Concurrent programming in Rust](topics/rust-concurrent-programming/README.md): some examples and exploration of concurrent programming in Rust (huge topic!).

## Utilities

### Bacon

[`bacon`](https://crates.io/crates/bacon) is an automatic monitoring tool for Rust (but not only), which detects changes in your source code and automatically restarts builds, tests or other commands. It is particularly useful for code experimentation and TDD development.

```bash
bacon run -- -q --example <rust-example-file>
```
