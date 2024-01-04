use std::io::Cursor;

use bytes::{Buf, Bytes, BytesMut};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use roster::application::server::frame::Frame;
use roster::application::{self};

fn check_frame(buf: &[u8]) -> Result<(), application::server::frame::Error> {
    let mut cursor = Cursor::new(buf);
    Frame::check(&mut cursor)
}

fn check(c: &mut Criterion) {
    let ping_test = b"*1\r\n$4\r\nPING\r\n";

    c.bench_function("check_frame", |b| {
        b.iter_batched(
            || ping_test.as_slice(),
            |bytes| check_frame(black_box(bytes)).unwrap(),
            criterion::BatchSize::PerIteration,
        )
    });

    let get_test = b"*2\r\n$3\r\nGET\r\n$5\r\nhello\r\n";

    c.bench_function("check_frame_get", |b| {
        b.iter_batched(
            || get_test.as_slice(),
            |bytes| check_frame(black_box(bytes)).unwrap(),
            criterion::BatchSize::PerIteration,
        )
    });
}

fn parse_frame(
    buf: &mut Cursor<&[u8]>,
) -> Result<Frame, application::server::frame::Error> {
    Frame::parse(buf)
}

fn parse(c: &mut Criterion) {
    let ping_test = b"*1\r\n$4\r\nPING\r\n";

    c.bench_function("parse_frame", |b| {
        b.iter_batched(
            || Cursor::new(ping_test.as_slice()),
            |mut cursor| parse_frame(black_box(&mut cursor)).unwrap(),
            criterion::BatchSize::PerIteration,
        )
    });

    let get_test = b"*2\r\n$3\r\nGET\r\n$5\r\nhello\r\n";

    c.bench_function("parse_frame_get", |b| {
        b.iter_batched(
            || Cursor::new(get_test.as_slice()),
            |mut cursor| parse_frame(black_box(&mut cursor)).unwrap(),
            criterion::BatchSize::PerIteration,
        )
    });
}

fn compose_check_parse(
    mut buffer: BytesMut,
) -> Result<Option<Frame>, application::server::frame::Error> {
    let mut buf = Cursor::new(&buffer[..]);
    Frame::check(&mut buf).unwrap();
    let len = buf.position() as usize;
    buf.set_position(0);
    let frame = Frame::parse(&mut buf)?;
    buffer.advance(len);
    buffer.reserve(4 * 1024);
    Ok(Some(frame))
}

fn compose(c: &mut Criterion) {
    let get_test = b"*2\r\n$3\r\nGET\r\n$5\r\nhello\r\n";

    c.bench_function("compose_get", |b| {
        b.iter_batched(
            || BytesMut::from(get_test.as_slice()),
            |buf| compose_check_parse(black_box(buf)).unwrap(),
            criterion::BatchSize::PerIteration,
        )
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = check, parse, compose
}
criterion_main!(benches);
