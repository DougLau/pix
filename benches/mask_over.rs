#[macro_use]
extern crate criterion;

use criterion::Criterion;
use pix::*;

fn mask_over_gray(c: &mut Criterion, sz: u32) {
    let s = format!("mask_over_gray_{}", sz);
    c.bench_function(&s, move |b| {
        let mut r = Raster::<Graya8>::with_clear(sz, sz);
        let mut m = Raster::<Matte8>::with_clear(sz, sz);
        let c = Graya8::new(100, 255);
        let sz1 = (sz - 1) as i32;
        *m.pixel_mut(0, 0) = Matte8::new(255);
        *m.pixel_mut(sz1, sz1) = Matte8::new(128);
        b.iter(|| r.composite_matte((), &m, (), c, ops::SrcOver))
    });
}

fn mask_over_gray_16(c: &mut Criterion) {
    mask_over_gray(c, 16);
}

fn mask_over_gray_256(c: &mut Criterion) {
    mask_over_gray(c, 256);
}

fn mask_over_rgba(c: &mut Criterion, sz: u32) {
    let s = format!("mask_over_rgba_{}", sz);
    c.bench_function(&s, move |b| {
        let mut r = Raster::<Rgba8>::with_clear(sz, sz);
        let mut m = Raster::<Matte8>::with_clear(sz, sz);
        let rgba = Rgba8::new(100, 50, 150, 255);
        let sz1 = (sz - 1) as i32;
        *m.pixel_mut(0, 0) = Matte8::new(255);
        *m.pixel_mut(sz1, sz1) = Matte8::new(128);
        b.iter(|| r.composite_matte((), &m, (), rgba, ops::SrcOver))
    });
}

fn mask_over_rgba_16(c: &mut Criterion) {
    mask_over_rgba(c, 16);
}

fn mask_over_rgba_256(c: &mut Criterion) {
    mask_over_rgba(c, 256);
}

criterion_group!(
    benches,
    mask_over_gray_16,
    mask_over_gray_256,
    mask_over_rgba_16,
    mask_over_rgba_256,
);

criterion_main!(benches);
