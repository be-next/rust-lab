# How to use cargo bench

`cargo bench` is a command in Rust’s package manager, **Cargo**, that runs performance benchmarks on Rust code. It relies on the `criterion` library to provide precise measurements.

## ⚙️ Prerequisites: Enabling Benchmarking in Rust

To use `cargo bench`, you need to enable the `benches` feature in your `Cargo.toml` file. This feature is disabled by default. To enable it, add the following line to your `Cargo.toml` file:

```toml
[dev-dependencies]
criterion = "0.5" # or a more recent version

[[bench]]
name = "my_benchmark"
harness = false  # Disables the default test harness to use Criterion
```

## 🚀 Running Benchmark

Inside the `benches/` folder, create a file [`my_benchmark.rs`](benches/my_benchmark.rs) with the following content:

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(20)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

where :

- `c.bench_function(name, closure)`: Declares a benchmark with an identifiable name.
- `b.iter(|| function_to_measure())`: Runs the function multiple times to get an average time.
- `black_box(x)` (not used here but recommended): Prevents the compiler from optimizing out the benchmark by treating the variable as unknown.

🏃‍♂️ Running the Benchmarks

To run the benchmark, execute the following command:

```bash
cargo bench
```

This command will compile the benchmark code and run the benchmark. The output will show the average time taken to run the benchmark.

## 📊 Interpreting the Results

The output of `cargo bench` will show the average time taken to run the benchmark. The `criterion` library provides a detailed report with statistics and plots to help you analyze the performance of your code.

The output will look like this:

```bash
fib 20                  time:   [267.72 ps 268.17 ps 268.69 ps]
```

This output shows the average time taken to run the benchmark. The range `[267.72 ps 268.17 ps 268.69 ps]` indicates the minimum, maximum, and median time taken to run the benchmark.

## 🔄 Comparing with Previous Versions

You can compare the performance of your code with previous versions by using the `cargo benchcmp` command. This command compares the benchmark results of two different commits.

To compare the benchmark results of the current version with the previous version, run the following command:

```bash
cargo benchcmp <commit1> <commit2>
```

This command will show the difference in performance between the two commits.

## 🧪 Writing Custom Benchmarks

You can write custom benchmarks to measure the performance of your code. Use the `criterion` library to write benchmarks that measure the performance of specific functions or code blocks.

For more information on writing custom benchmarks, refer to the [Criterion documentation](https://bheisler.github.io/criterion.rs/book/index.html).
