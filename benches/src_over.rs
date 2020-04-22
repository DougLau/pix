#[macro_use]
extern crate criterion;

use criterion::Criterion;
use pix::chan::Premultiplied;
use pix::el::Pixel;
use pix::gray::Graya8p;
use pix::matte::Matte8;
use pix::ops::SrcOver;
use pix::rgb::Rgba8p;
use pix::Raster;

fn matte_over_gray(c: &mut Criterion, sz: u32) {
    let s = format!("matte_over_gray_{}", sz);
    c.bench_function(&s, move |b| {
        let mut r = Raster::with_clear(sz, sz);
        let mut m = Raster::<Matte8>::with_clear(sz, sz);
        let c = Graya8p::new(100, 255);
        let sz1 = (sz - 1) as i32;
        *m.pixel_mut(0, 0) = Matte8::new(255);
        *m.pixel_mut(sz1, sz1) = Matte8::new(128);
        b.iter(|| r.composite_matte((), &m, (), c, SrcOver))
    });
}

fn matte_over_gray_16(c: &mut Criterion) {
    matte_over_gray(c, 16);
}

fn matte_over_gray_256(c: &mut Criterion) {
    matte_over_gray(c, 256);
}

fn matte_over_rgba(c: &mut Criterion, sz: u32) {
    let s = format!("matte_over_rgba_{}", sz);
    c.bench_function(&s, move |b| {
        let mut r = Raster::with_clear(sz, sz);
        let mut m = Raster::<Matte8>::with_clear(sz, sz);
        let rgba = Rgba8p::new(100, 50, 150, 255);
        let sz1 = (sz - 1) as i32;
        *m.pixel_mut(0, 0) = Matte8::new(255);
        *m.pixel_mut(sz1, sz1) = Matte8::new(128);
        b.iter(|| r.composite_matte((), &m, (), rgba, SrcOver))
    });
}

fn matte_over_rgba_16(c: &mut Criterion) {
    matte_over_rgba(c, 16);
}

fn matte_over_rgba_256(c: &mut Criterion) {
    matte_over_rgba(c, 256);
}

fn raster_over<P>(c: &mut Criterion, tp: &str, sz: u32)
where
    P: Pixel<Alpha = Premultiplied>,
{
    let s = format!("raster_over_{}_{}", tp, sz);
    c.bench_function(&s, move |b| {
        let mut r0 = Raster::<P>::with_clear(sz, sz);
        let r1 = Raster::with_clear(sz, sz);
        b.iter(|| r0.composite_raster((), &r1, (), SrcOver))
    });
}

fn raster_over_gray_16(c: &mut Criterion) {
    raster_over::<Graya8p>(c, "gray", 16);
}

fn raster_over_gray_256(c: &mut Criterion) {
    raster_over::<Graya8p>(c, "gray", 256);
}

fn raster_over_rgba_16(c: &mut Criterion) {
    raster_over::<Rgba8p>(c, "rgba", 16);
}

fn raster_over_rgba_256(c: &mut Criterion) {
    raster_over::<Rgba8p>(c, "rgba", 256);
}

criterion_group!(
    benches,
    matte_over_gray_16,
    matte_over_gray_256,
    matte_over_rgba_16,
    matte_over_rgba_256,
    raster_over_gray_16,
    raster_over_gray_256,
    raster_over_rgba_16,
    raster_over_rgba_256,
);

criterion_main!(benches);
