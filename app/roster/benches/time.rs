use std::time::SystemTime;

use criterion::{criterion_group, criterion_main, Criterion};
use monoio::time::Instant;

fn time_b(c: &mut Criterion) {
    c.bench_function("monotonic_time", |b| {
        b.iter(|| Instant::now());
    });

    c.bench_function("system_time", |b| {
        b.iter(|| SystemTime::now());
    });

    c.bench_function("coarsetime_time_instant", |b| {
        b.iter(|| coarsetime::Instant::now());
    });

    c.bench_function("coarsetime_time_recent", |b| {
        b.iter(|| coarsetime::Instant::recent());
    });
}

criterion_group!(times, time_b);
criterion_main!(times);
