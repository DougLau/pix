#[macro_use]
extern crate criterion;
extern crate pix;

use pix::*;
use criterion::Criterion;

fn ch8_mul(c: &mut Criterion) {
    c.bench_function(&"ch8_mul", move |b| {
        let n = Ch8::new(128);
        let d = Ch8::new(64);
        b.iter(|| n * d)
    });
}

fn ch8_div(c: &mut Criterion) {
    c.bench_function(&"ch8_div", move |b| {
        let n = Ch8::new(128);
        let d = Ch8::new(64);
        b.iter(|| n / d)
    });
}

criterion_group!(benches, ch8_mul, ch8_div);

criterion_main!(benches);
