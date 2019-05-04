#[macro_use]
extern crate criterion;
extern crate pix;

use pix::*;
use criterion::Criterion;

fn gray_over(c: &mut Criterion, sz: u32) {
    let s = format!("gray_over_{}", sz);
    c.bench_function(&s, move |b| {
        let mut r = Raster::<Gray<Cu8>>::new(sz, sz);
        let mut m = Raster::<Alpha<Cu8>>::new(sz, sz);
        m.set_pixel(0, 0, Alpha::<Cu8>::new(255.into()));
        m.set_pixel(sz - 1, sz - 1, Alpha::<Cu8>::new(128.into()));
        b.iter(|| r.mask_over(&m, 0, 0, Gray::<Cu8>::new(100.into())))
    });
}

fn gray_over_16(c: &mut Criterion) {
    gray_over(c, 16);
}

fn gray_over_256(c: &mut Criterion) {
    gray_over(c, 256);
}

fn gray_over_512(c: &mut Criterion) {
    gray_over(c, 512);
}

fn rgba_over(c: &mut Criterion, sz: u32) {
    let s = format!("rgba_over_{}", sz);
    c.bench_function(&s, move |b| {
        let mut r = Raster::<Rgba<Cu8>>::new(sz, sz);
        let mut m = Raster::<Alpha<Cu8>>::new(sz, sz);
        let rgba = Rgba::new(100.into(), 50.into(), 150.into(), 255.into());
        m.set_pixel(0, 0, Alpha::<Cu8>::new(255.into()));
        m.set_pixel(sz - 1, sz - 1, Alpha::<Cu8>::new(128.into()));
        b.iter(|| r.mask_over(&m, 0, 0, rgba))
    });
}

fn rgba_over_16(c: &mut Criterion) {
    rgba_over(c, 16);
}

fn rgba_over_256(c: &mut Criterion) {
    rgba_over(c, 256);
}

fn rgba_over_512(c: &mut Criterion) {
    rgba_over(c, 512);
}

criterion_group!(benches, gray_over_16, gray_over_256, gray_over_512,
                          rgba_over_16, rgba_over_256, rgba_over_512);

criterion_main!(benches);
