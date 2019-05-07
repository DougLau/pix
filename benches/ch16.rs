#[macro_use]
extern crate criterion;
extern crate pix;

use pix::*;
use criterion::Criterion;

fn ch16_mul(c: &mut Criterion) {
    c.bench_function(&"ch16_mul", move |b| {
        let n = Ch16::new(32768);
        let d = Ch16::new(16384);
        b.iter(|| n * d)
    });
}

fn ch16_div(c: &mut Criterion) {
    c.bench_function(&"ch16_div", move |b| {
        let n = Ch16::new(32768);
        let d = Ch16::new(16384);
        b.iter(|| n / d)
    });
}

criterion_group!(benches, ch16_mul, ch16_div);

criterion_main!(benches);
