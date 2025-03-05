use criterion::{Criterion, criterion_group, criterion_main};

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

static TERM: u32 = 30;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        format!("recursice_naive_fibonacci for term {}", TERM).as_str(),
        |b| b.iter(|| recursice_naive_fibonacci(TERM)),
    );
    c.bench_function(
        format!("recusive_idiomatic_fibonacci for term {}", TERM).as_str(),
        |b| b.iter(|| recusive_idiomatic_fibonacci(TERM)),
    );
    c.bench_function(
        format!("functionnal_fibonacci for term {}", TERM).as_str(),
        |b| b.iter(|| functionnal_fibonacci(TERM)),
    );
    c.bench_function(format!("binet_fibonacci for term {}", TERM).as_str(), |b| {
        b.iter(|| binet_fibonacci(TERM))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
