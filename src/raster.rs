// raster.rs    A 2D raster image.
//
// Copyright (c) 2017-2019  Douglas P Lau
//
use std::marker::PhantomData;
use crate::alpha8::Alpha8;
use crate::pixel::PixFmt;

/// A raster image with owned pixel data.
/// If the pixel data must be owned elsewhere, consider using
/// [RasterB](struct.RasterB.html).
///
/// # Example
/// ```
/// use pix::{Raster, Alpha8, Rgba8};
/// let mut raster: Raster<Rgba8> = Raster::new(10, 10);
/// let mut matte: Raster<Alpha8> = Raster::new(10, 10);
/// matte.set_pixel(2, 4, Alpha8::new(255));
/// matte.set_pixel(2, 5, Alpha8::new(128));
/// raster.mask_over(&matte, 0, 0, Rgba8::new(128, 208, 208, 200));
/// let p = raster.as_u8_slice();
/// // work with pixel data...
/// ```
pub struct Raster<F: PixFmt> {
    width  : u32,
    height : u32,
    pixels : Vec<F>,
}

impl<F: PixFmt> Raster<F> {
    /// Create a new raster image.
    ///
    /// * `F` [Pixel format](trait.PixFmt.html).
    /// * `width` Width in pixels.
    /// * `height` Height in pixels.
    pub fn new(width: u32, height: u32) -> Raster<F> {
        let len = width * height;
        let mut pixels = Vec::with_capacity(capacity(len));
        for _ in 0..len {
            pixels.push(F::default());
        }
        Raster { width, height, pixels }
    }
    /// Get raster width.
    pub fn width(&self) -> u32 {
        self.width
    }
    /// Get raster height.
    pub fn height(&self) -> u32 {
        self.height
    }
    /// Get the length.
    fn len(&self) -> usize {
        (self.width * self.height) as usize
    }
    /// Get one pixel value.
    pub fn pixel(&self, x: u32, y: u32) -> F {
        let i = (self.width * y + x) as usize;
        self.pixels[i]
    }
    /// Set one pixel value.
    pub fn set_pixel(&mut self, x: u32, y: u32, p: F) {
        let i = (self.width * y + x) as usize;
        self.pixels[i] = p;
    }
    /// Get the pixels as a slice.
    pub fn as_slice(&self) -> &[F] {
        &self.pixels
    }
    /// Get the pixels as a mutable slice.
    pub fn as_slice_mut(&mut self) -> &mut [F] {
        &mut self.pixels
    }
    /// Get a row of pixels as a mutable slice.
    fn row_slice_mut(&mut self, x: u32, y: u32) -> &mut [F] {
        debug_assert!(x < self.width && y < self.height);
        let s = (y * self.width + x) as usize;
        let t = s + (self.width - x) as usize;
        &mut self.pixels[s..t]
    }
    /// Get a row of pixels as a u8 slice.
    fn row_slice_u8(&self, x: u32, y: u32) -> &[u8] {
        debug_assert!(x < self.width && y < self.height);
        let s = (y * self.width + x) as usize;
        let t = s + (self.width - x) as usize;
        F::as_u8_slice(&self.pixels[s..t])
    }
    /// Get the pixels as a u8 slice.
    pub fn as_u8_slice(&self) -> &[u8] {
        F::as_u8_slice(&self.pixels)
    }
    /// Get the pixels as a mutable u8 slice.
    pub fn as_u8_slice_mut(&mut self) -> &mut [u8] {
        F::as_u8_slice_mut(&mut self.pixels)
    }
    /// Clear all pixels.
    pub fn clear(&mut self) {
        debug_assert_eq!(self.len(), self.pixels.len());
        for p in self.pixels.iter_mut() {
            *p = F::default();
        }
    }
    /// Blend pixels with an alpha mask.
    ///
    /// * `mask` Alpha mask for compositing.
    /// * `x` Left position of alpha mask.
    /// * `y` Top position of alpha mask.
    /// * `clr` Color to composite.
    pub fn mask_over(&mut self, mask: &Raster<Alpha8>, x: i32, y: i32, clr: F) {
        if x == 0 && self.width() == mask.width() &&
           y == 0 && self.height() == mask.height()
        {
            F::mask_over(&mut self.pixels, mask.as_u8_slice(), clr);
            return;
        }
        if x + (mask.width() as i32) < 0 || x >= self.width() as i32 {
            return; // positioned off left or right edge
        }
        if y + (mask.height() as i32) < 0 || y >= self.height() as i32 {
            return; // positioned off top or bottom edge
        }
        let mx = 0.max(-x) as u32;
        let my = 0.max(-y) as u32;
        let dx = 0.max(x) as u32;
        let dy = 0.max(y) as u32;
        let h = (self.height() - dy).min(mask.height() - my);
        for yi in 0..h {
            let mut row = self.row_slice_mut(dx, dy + yi);
            let m = mask.row_slice_u8(mx, my + yi);
            F::mask_over(&mut row, m, clr);
        }
    }
}

/// Get the required capacity of the pixel vector.
fn capacity(len: u32) -> usize {
    // Capacity must be 8-element multiple (for SIMD)
    (((len + 7) >> 3) << 3) as usize
}

/// A raster image with borrowed pixel data.
/// This is more tricky to use than [Raster](struct.Raster.html),
/// so it should only be used when pixel data must be owned elsewhere.
///
/// # Example
/// ```
/// use pix::{PixFmt, RasterB, Rgba8};
/// let mut r = RasterB::<Rgba8>::new(10, 10);
/// let len = (r.width() * r.height()) as usize;
/// // NOTE: typically the pixels would be borrowed from some other source
/// let mut pixels = vec!(0; len * std::mem::size_of::<Rgba8>());
/// let mut pix = Rgba8::as_slice_mut(&mut pixels);
/// ```
pub struct RasterB<F: PixFmt> {
    width  : u32,
    height : u32,
    pixels : PhantomData<F>,
}

impl<F: PixFmt> RasterB<F> {
    /// Create a new raster image for borrowed pixel data.
    ///
    /// * `F` [Pixel format](trait.PixFmt.html).
    /// * `width` Width in pixels.
    /// * `height` Height in pixels.
    pub fn new(width: u32, height: u32) -> RasterB<F> {
        let pixels = PhantomData;
        RasterB { width, height, pixels }
    }
    /// Get raster width.
    pub fn width(&self) -> u32 {
        self.width
    }
    /// Get raster height.
    pub fn height(&self) -> u32 {
        self.height
    }
    /// Get the length.
    fn len(&self) -> usize {
        (self.width * self.height) as usize
    }
    /// Clear all pixels.
    pub fn clear(&self, pixels: &mut [F]) {
        assert_eq!(self.len(), pixels.len());
        for p in pixels.iter_mut() {
            *p = F::default();
        }
    }
    /// Blend pixels with an alpha mask.
    ///
    /// * `mask` Alpha mask for compositing.
    /// * `clr` Color to composite.
    /// * `pixels` Borrowed pixel data.
    pub fn mask_over(&self, mask: &[u8], clr: F, mut pixels: &mut [F]) {
        assert_eq!(self.len(), pixels.len());
        F::mask_over(&mut pixels, mask, clr);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::alpha8::*;
    use super::super::rgba8::*;
    #[test]
    fn raster_alpha() {
        let m = Raster::<Alpha8>::new(10, 10);
        assert!(m.width == 10);
        assert!(m.height == 10);
        assert!(m.pixels.len() == 100);
    }
    #[test]
    fn raster_mask() {
        let mut r = Raster::<Rgba8>::new(3, 3);
        let mut m = Raster::<Alpha8>::new(3, 3);
        m.set_pixel(0, 0, Alpha8::new(0xFF));
        m.set_pixel(1, 1, Alpha8::new(0x80));
        m.set_pixel(2, 2, Alpha8::new(0x40));
        r.mask_over(&m, 0, 0, Rgba8::new(0xFF,0x80,0x40,0xFF));
        let v = vec![
            0xFF,0x80,0x40,0xFF, 0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00, 0x80,0x40,0x20,0x80, 0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00, 0x40,0x20,0x10,0x40,
        ];
        let left = r.as_u8_slice();
        // NOTE: fallback version     SIMD version
        assert!(left[0] == 0xFF || left[0] == 0xFE);
        assert!(left[1] == 0x80 || left[1] == 0x7F);
        assert!(left[2] == 0x40 || left[2] == 0x3F);
        assert!(left[3] == 0xFF || left[3] == 0xFE);
        assert_eq!(&left[4..], &v[4..]);
    }
    #[test]
    fn smaller_mask() {
        let mut r = Raster::<Rgba8>::new(3, 3);
        let mut m = Raster::<Alpha8>::new(2, 2);
        m.set_pixel(0, 0, Alpha8::new(0xFF));
        m.set_pixel(1, 0, Alpha8::new(0x80));
        m.set_pixel(0, 1, Alpha8::new(0x40));
        m.set_pixel(1, 1, Alpha8::new(0x20));
        r.mask_over(&m, 1, 1, Rgba8::new(0x40,0xFF,0x80,0x80));
        let v = vec![
            0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00, 0x40,0xFF,0x80,0x80, 0x20,0x80,0x40,0x40,
            0x00,0x00,0x00,0x00, 0x10,0x40,0x20,0x20, 0x08,0x20,0x10,0x10,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn top_left() {
        let mut r = Raster::<Rgba8>::new(3, 3);
        let mut m = Raster::<Alpha8>::new(2, 2);
        m.set_pixel(0, 0, Alpha8::new(0xFF));
        m.set_pixel(1, 0, Alpha8::new(0xFF));
        m.set_pixel(0, 1, Alpha8::new(0xFF));
        m.set_pixel(1, 1, Alpha8::new(0xFF));
        r.mask_over(&m, -1, -1, Rgba8::new(0x20,0x40,0x80,0xFF));
        let v = vec![
            0x20,0x40,0x80,0xFF, 0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn bottom_right() {
        let mut r = Raster::<Rgba8>::new(3, 3);
        let mut m = Raster::<Alpha8>::new(2, 2);
        m.set_pixel(0, 0, Alpha8::new(0xFF));
        m.set_pixel(1, 0, Alpha8::new(0xFF));
        m.set_pixel(0, 1, Alpha8::new(0xFF));
        m.set_pixel(1, 1, Alpha8::new(0xFF));
        r.mask_over(&m, 2, 2, Rgba8::new(0x20,0x40,0x80,0xFF));
        let v = vec![
            0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00, 0x00,0x00,0x00,0x00, 0x20,0x40,0x80,0xFF,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
}
