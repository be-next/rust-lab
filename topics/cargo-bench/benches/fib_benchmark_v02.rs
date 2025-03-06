use criterion::{BenchmarkId, Criterion, PlotConfiguration, criterion_group, criterion_main};

fn recursice_naive_fibonacci(term: u32) -> u32 {
    if term == 0 {
        0
    } else if term == 1 {
        1
    } else {
        recursice_naive_fibonacci(term - 1) + recursice_naive_fibonacci(term - 2)
    }
}

fn recusive_idiomatic_fibonacci(term: u32) -> u32 {
    match term {
        0 | 1 => term,
        _ => recusive_idiomatic_fibonacci(term - 1) + recusive_idiomatic_fibonacci(term - 2),
    }
}

fn functionnal_fibonacci(term: u32) -> u32 {
    match term {
        0 | 1 => term,
        _ => {
            let fib = (1..term).fold((0, 1), |(previous_value, current_value), _| {
                (current_value, previous_value + current_value)
            });
            fib.1
        }
    }
}

fn imperative_fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn binet_fibonacci(term: u32) -> u64 {
    // Compute the square root of 5.
    let sqrt5 = 5_f64.sqrt();

    // Define phi and psi, the two roots of x^2 = x + 1.
    let phi = (1.0 + sqrt5) / 2.0;
    let psi = (1.0 - sqrt5) / 2.0;

    // Binet's formula:
    //   F(n) = (phi^n - psi^n) / sqrt(5)
    (((phi.powf(term as f64) - psi.powf(term as f64)) / sqrt5).round()) as u64
}

// static TERM: u32 = 30;

fn criterion_benchmark(c: &mut Criterion) {
    // Create a group to compare implementations
    let mut group = c.benchmark_group("Fibonacci");

    // Configure the chart style
    group
        .plot_config(PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic)); // Échelle logarithmique pour mieux visualiser les différences

    group.sampling_mode(criterion::SamplingMode::Flat); // For more precise measurements

    // You can also add a reference comparison
    group.bench_function("baseline", |b| b.iter(|| {})); // To measure overhead

    // Finally, you can configure the group with specific parameters
    group.sample_size(100).significance_level(0.01);

    // Benchmark each function for the same value
    for term in [30].iter() {
        group.bench_with_input(
            BenchmarkId::new("Naive recursive", term),
            term,
            |b, &term| b.iter(|| recursice_naive_fibonacci(term)),
        );

        group.bench_with_input(
            BenchmarkId::new("Idiomatic recursive", term),
            term,
            |b, &term| b.iter(|| recusive_idiomatic_fibonacci(term)),
        );

        group.bench_with_input(BenchmarkId::new("Functional", term), term, |b, &term| {
            b.iter(|| functionnal_fibonacci(term))
        });

        group.bench_with_input(BenchmarkId::new("Imperative", term), term, |b, &term| {
            b.iter(|| imperative_fibonacci(term as u64))
        });

        group.bench_with_input(BenchmarkId::new("Binet formula", term), term, |b, &term| {
            b.iter(|| binet_fibonacci(term))
        });
    }

    // // Finish the group
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
