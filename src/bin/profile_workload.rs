use broken_app::{algo, normalize};
use std::hint::black_box;
use std::time::{Duration, Instant};

struct Measurement {
    name: &'static str,
    calls: usize,
    elapsed: Duration,
}

fn measure(name: &'static str, calls: usize, mut workload: impl FnMut()) -> Measurement {
    let started = Instant::now();
    for _ in 0..calls {
        workload();
    }
    Measurement {
        name,
        calls,
        elapsed: started.elapsed(),
    }
}

fn main() {
    let text = "Hello Rust World ".repeat(3_000);
    let dedup_data: Vec<u64> = (0..1_000).flat_map(|n| [n, n]).rev().collect();

    let measurements = [
        measure("normalize_51k", 2_000, || {
            black_box(normalize(black_box(&text)));
        }),
        measure("fib_32", 100, || {
            black_box(algo::slow_fib(black_box(32)));
        }),
        measure("dedup_2k", 200, || {
            black_box(algo::slow_dedup(black_box(&dedup_data)));
        }),
    ];

    let total: Duration = measurements.iter().map(|item| item.elapsed).sum();
    println!("workload,total_ms,per_call_us,share_percent");
    for item in measurements {
        let total_ms = item.elapsed.as_secs_f64() * 1_000.0;
        let per_call_us = item.elapsed.as_secs_f64() * 1_000_000.0 / item.calls as f64;
        let share = item.elapsed.as_secs_f64() / total.as_secs_f64() * 100.0;
        println!("{},{total_ms:.3},{per_call_us:.3},{share:.2}", item.name);
    }
}
