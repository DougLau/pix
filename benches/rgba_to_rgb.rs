#[macro_use]
extern crate criterion;

use criterion::Criterion;
use pix::Raster;
use pix::rgb::{Rgb8, Rgba8p};

fn rgba_to_rgb(c: &mut Criterion, sz: u32) {
    let s = format!("rgba_to_rgb_{}", sz);
    c.bench_function(&s, move |b| {
        let r = Raster::<Rgba8p>::with_clear(sz, sz);
        b.iter(|| Raster::<Rgb8>::with_raster(&r))
    });
}

fn rgba_to_rgb_16(c: &mut Criterion) {
    rgba_to_rgb(c, 16);
}

fn rgba_to_rgb_256(c: &mut Criterion) {
    rgba_to_rgb(c, 256);
}

criterion_group!(benches, rgba_to_rgb_16, rgba_to_rgb_256,);

criterion_main!(benches);
