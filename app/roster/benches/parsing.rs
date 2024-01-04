use std::io::Cursor;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use roster::application::server::frame::Frame;
use roster::application::{self};

fn parse_frame(buf: &[u8]) -> Result<(), application::server::frame::Error> {
    let mut cursor = Cursor::new(buf);
    Frame::check(&mut cursor)
}

fn criterion_benchmark(c: &mut Criterion) {
    let test = r"*1\r\n\$4\r\nPING\r\n";

    c.bench_function("parse_frame", |b| {
        b.iter(|| parse_frame(black_box(test.as_bytes())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
