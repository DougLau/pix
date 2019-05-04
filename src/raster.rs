// raster.rs    A 2D raster image.
//
// Copyright (c) 2017-2019  Douglas P Lau
//
use crate::alpha8::Alpha8;
use crate::pixel::PixFmt;

/// A raster image with owned pixel data.
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
#[derive(Clone, Debug)]
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
        let row = &self.row_slice(y);
        row[x as usize]
    }
    /// Set one pixel value.
    pub fn set_pixel(&mut self, x: u32, y: u32, p: F) {
        let row = &mut self.row_slice_mut(y);
        row[x as usize] = p;
    }
    /// Get the pixels as a slice.
    pub fn as_slice(&self) -> &[F] {
        &self.pixels
    }
    /// Get the pixels as a mutable slice.
    pub fn as_slice_mut(&mut self) -> &mut [F] {
        &mut self.pixels
    }
    /// Get a row of pixels as a slice.
    fn row_slice(&self, y: u32) -> &[F] {
        debug_assert!(y < self.height);
        let s = (y * self.width) as usize;
        let t = s + self.width as usize;
        &self.pixels[s..t]
    }
    /// Get a row of pixels as a mutable slice.
    fn row_slice_mut(&mut self, y: u32) -> &mut [F] {
        debug_assert!(y < self.height);
        let s = (y * self.width) as usize;
        let t = s + self.width as usize;
        &mut self.pixels[s..t]
    }
    /// Get a row of pixels as a u8 slice.
    fn row_slice_u8(&self, y: u32) -> &[u8] {
        debug_assert!(y < self.height);
        let s = (y * self.width) as usize;
        let t = s + self.width as usize;
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
    /// Set a rectangle to specified color.
    ///
    /// * `x` Left position of rectangle.
    /// * `y` Top position of rectangle.
    /// * `w` Width of rectangle.
    /// * `h` Height of rectangle.
    /// * `clr` Color to set.
    pub fn set_rect(&mut self, x: u32, y: u32, w: u32, h: u32, clr: F) {
        if y < self.height() && x < self.width() {
            let xm = self.width.min(x + w);
            let ym = self.height.min(y + h);
            let xrange = (x as usize)..(xm as usize);
            for yi in y..ym {
                self.row_slice_mut(yi)[xrange.clone()]
                    .iter_mut()
                    .for_each(|p| *p = clr);
            }
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
        let mx = 0.max(-x) as usize;
        let my = 0.max(-y) as u32;
        let mw = mask.width as usize;
        let dx = 0.max(x) as usize;
        let dy = 0.max(y) as u32;
        let dw = self.width as usize;
        let h = (self.height - dy).min(mask.height - my);
        for yi in 0..h {
            let mut row = &mut self.row_slice_mut(dy + yi)[dx..dw];
            let m = &mask.row_slice_u8(my + yi)[mx..mw];
            F::mask_over(&mut row, m, clr);
        }
    }
}

/// Get the required capacity of the pixel vector.
fn capacity(len: u32) -> usize {
    // Capacity must be 8-element multiple (for SIMD)
    (((len + 7) >> 3) << 3) as usize
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::alpha8::*;
    use super::super::gray8::*;
    use super::super::rgb8::*;
    use super::super::rgba8::*;
    #[test]
    fn raster_alpha() {
        let m = Raster::<Alpha8>::new(10, 10);
        assert!(m.width == 10);
        assert!(m.height == 10);
        assert!(m.pixels.len() == 100);
    }
    #[test]
    fn rectangle_rgb() {
        let mut r = Raster::<Rgb8>::new(4, 4);
        r.set_rect(1, 1, 2, 2, Rgb8::new(0xCC, 0xAA, 0xBB));
        let v = vec![
            0x00,0x00,0x00, 0x00,0x00,0x00, 0x00,0x00,0x00, 0x00,0x00,0x00,
            0x00,0x00,0x00, 0xCC,0xAA,0xBB, 0xCC,0xAA,0xBB, 0x00,0x00,0x00,
            0x00,0x00,0x00, 0xCC,0xAA,0xBB, 0xCC,0xAA,0xBB, 0x00,0x00,0x00,
            0x00,0x00,0x00, 0x00,0x00,0x00, 0x00,0x00,0x00, 0x00,0x00,0x00,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn rectangle_gray() {
        let mut r = Raster::<Gray8>::new(4, 4);
        r.set_rect(0, 0, 1, 1, Gray8::new(0x23));
        r.set_rect(10, 10, 1, 1, Gray8::new(0x45));
        r.set_rect(2, 2, 10, 10, Gray8::new(0xBB));
        let v = vec![
            0x23,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,
            0x00,0x00,0xBB,0xBB,
            0x00,0x00,0xBB,0xBB,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
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
