use std::io::Cursor;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use roster::application::server::frame::Frame;
use roster::application::{self};

fn check_frame(buf: &[u8]) -> Result<(), application::server::frame::Error> {
    let mut cursor = Cursor::new(buf);
    Frame::check(&mut cursor)
}

fn check(c: &mut Criterion) {
    let test = r"*1\r\n\$4\r\nPING\r\n";

    c.bench_function("check_frame", |b| {
        b.iter(|| check_frame(black_box(test.as_bytes())))
    });

    let test = r"*2\r\n$3\r\nGET\r\n$5\r\nhello\r\n";

    c.bench_function("check_frame_get", |b| {
        b.iter(|| check_frame(black_box(test.as_bytes())))
    });
}

fn parse_frame(buf: &[u8]) -> Result<Frame, application::server::frame::Error> {
    let mut cursor = Cursor::new(buf);
    Frame::parse(&mut cursor)
}

fn parse(c: &mut Criterion) {
    let test = r"*1\r\n\$4\r\nPING\r\n";

    c.bench_function("parse_frame", |b| {
        b.iter(|| parse_frame(black_box(test.as_bytes())))
    });

    let test = r"*2\r\n$3\r\nGET\r\n$5\r\nhello\r\n";

    c.bench_function("parse_frame_get", |b| {
        b.iter(|| parse_frame(black_box(test.as_bytes())))
    });
}

criterion_group!(benches, check, parse);
criterion_main!(benches);
