#[macro_use]
extern crate criterion;
extern crate pix;

use pix::*;
use criterion::Criterion;

fn cu8_mul(c: &mut Criterion) {
    c.bench_function(&"cu8_mul", move |b| {
        let n = Ch8::new(128);
        let d = Ch8::new(64);
        b.iter(|| n * d)
    });
}

fn cu8_div(c: &mut Criterion) {
    c.bench_function(&"cu8_div", move |b| {
        let n = Ch8::new(128);
        let d = Ch8::new(64);
        b.iter(|| n / d)
    });
}

criterion_group!(benches, cu8_mul, cu8_div);

criterion_main!(benches);
