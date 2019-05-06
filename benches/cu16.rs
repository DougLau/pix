#[macro_use]
extern crate criterion;
extern crate pix;

use pix::*;
use criterion::Criterion;

fn cu16_mul(c: &mut Criterion) {
    c.bench_function(&"cu16_mul", move |b| {
        let n = Cu16::new(32768);
        let d = Cu16::new(16384);
        b.iter(|| n * d)
    });
}

fn cu16_div(c: &mut Criterion) {
    c.bench_function(&"cu16_div", move |b| {
        let n = Cu16::new(32768);
        let d = Cu16::new(16384);
        b.iter(|| n / d)
    });
}

criterion_group!(benches, cu16_mul, cu16_div);

criterion_main!(benches);
