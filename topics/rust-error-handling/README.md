# Error handling in Rust

I won’t elaborate further, but error handling is one of the most critical aspects of software development — because when things go wrong (and they will), how you handle failure defines the reliability of your software.

Rust provides a robust error management system that combines the strictness of the type system with structured, explicit handling, ensuring that failures are accounted for rather than left to chance.

Instead of relying on traditional exceptions, Rust enforces clear error propagation through `Result` and `Option` types, making error handling explicit and predictable. For truly unrecoverable situations, Rust provides panics, ensuring that failures are either properly managed or immediately halted.

## Understanding Rust's error handling

- [Error handling in official Rust documentation](https://doc.rust-lang.org/book/ch09-00-error-handling.html): because everything starts by RTFM!
- [Andrew Gallant's blog post on Error handling in Rust](https://blog.burntsushi.net/rust-error-handling/): there is a lot to learn to master error handling in Rust, this blog post is a good journey to visit Rust fondamentals trough the lens of error handling.
- [Rust Error Handling - Best Practices by Jeremy Chone](https://rust10x.com/best-practices/error-handling): agumented and experienced view on error handling in Rust brilliantly presented (youtube video bellow).

<p align="center">
  <a href="https://www.youtube.com/watch?v=j-VQCYP7wyw">
    <img src="https://img.youtube.com/vi/j-VQCYP7wyw/0.jpg" width="300"/>
  </a>
</p>

## Crates for error handling in Rust

There are several crates available to help with error handling in Rust. Here are some of the most popular ones:

- [anyhow crate](https://crates.io/crates/anyhow): Used when the precise error type is not important. It simplifies error handling by encapsulating errors into a single type (`anyhow::Error`), while allowing for proper display and error chaining.

- [thiserror crate](https://crates.io/crates/thiserror): Designed for idiomatic and ergonomic custom error creation, leveraging derive macros to avoid boilerplate code.

- [eyre crate](https://crates.io/crates/eyre): A fork of the `anyhow` crate, with additional features and a more ergonomic API (e.g., global hooks, enriched formatting).

- [miette crate](https://crates.io/crates/miette): A modern alternative to eyre, designed for detailed and well-formatted error messages (useful for CLI applications or developer tools).

- [snafu crate](https://crates.io/crates/snafu): A powerful error handling library that allows for detailed error context and chaining, with a focus on ergonomics and flexibility.
