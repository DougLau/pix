// raster.rs    A 2D raster image.
//
// Copyright (c) 2017-2019  Douglas P Lau
//
use crate::Format;

/// Raster image representing a two-dimensional array of pixels.
///
/// # Example
/// ```
/// use pix::{Raster, Rgb, Rgb8};
/// let clr: Rgb8 = Rgb::new(0xFF, 0x88, 0x00);
/// let mut raster: Raster<Rgb8> = Raster::new(10, 10);
/// raster.set_rect(2, 4, 3, 3, clr);
/// ```
#[derive(Clone, Debug)]
pub struct Raster<F: Format> {
    width  : u32,
    height : u32,
    pixels : Box<[F]>,
}

impl<F: Format> Into<Box<[F]>> for Raster<F> {
    /// Get internal pixel data as boxed slice.
    fn into(self) -> Box<[F]> {
        self.pixels
    }
}

impl<F: Format> Into<Vec<F>> for Raster<F> {
    /// Get internal pixel data as `Vec` of pixels.
    fn into(self) -> Vec<F> {
        self.pixels.into()
    }
}

impl<F: Format> Raster<F> {
    /// Create a new raster image.
    ///
    /// * `F` Pixel [Format](trait.Format.html).
    /// * `width` Width in pixels.
    /// * `height` Height in pixels.
    pub fn new(width: u32, height: u32) -> Raster<F> {
        let len = (width * height) as usize;
        let pixels = vec![F::default(); len].into_boxed_slice();
        Raster { width, height, pixels }
    }
    /// Create a new raster image with owned pixel data.  You can get ownership
    /// of the pixel data back from the `Raster` as either a `Vec<F>` or a
    /// `Box<[F]>` by calling `into()`.
    ///
    /// * `F` Pixel [Format](trait.Format.html).
    /// * `width` Width in pixels.
    /// * `height` Height in pixels.
    /// * `pixels` Pixel data.
    ///
    /// # Panics
    ///
    /// Panics if `pixels` length is not equal to `width` * `height`.
    pub fn with_pixels<B>(width: u32, height: u32, pixels: B) -> Raster<F>
        where B: Into<Box<[F]>>
    {
        let len = (width * height) as usize;
        let pixels = pixels.into();
        assert_eq!(len, pixels.len());
        Raster { width, height, pixels }
    }
    /// Create a new raster image from a buffer of bytes.
    ///
    /// * `F` Pixel [Format](trait.Format.html).
    /// * `width` Width in pixels.
    /// * `height` Height in pixels.
    /// * `buffer` Byte buffer of pixel data.
    ///
    /// This is unsafe because the byte buffer is transmuted into `F`.
    ///
    /// # Panics
    ///
    /// Panics if `buffer` length is not equal to `width` * `height` *
    /// `std::mem::size_of::<F>()`.
    pub unsafe fn with_buffer<B>(width: u32, height: u32, buffer: B)
        -> Raster<F> where B: Into<Box<[u8]>>
    {
        let len = (width * height) as usize;
        let buffer: Box<[u8]> = buffer.into();
        let capacity = buffer.len();
        assert_eq!(len * std::mem::size_of::<F>(), capacity);
        let slice = std::boxed::Box::<[u8]>::into_raw(buffer);
        let ptr = (*slice).as_mut_ptr();
        let ptr = std::mem::transmute::<*mut u8, *mut F>(ptr);
        let slice = std::slice::from_raw_parts_mut(ptr, len);
        let pixels: Box<[F]> = Box::from_raw(slice);
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
    /// Get one pixel value.
    pub fn pixel(&self, x: u32, y: u32) -> F {
        let row = &self.as_slice_row(y);
        row[x as usize]
    }
    /// Set one pixel value.
    pub fn set_pixel<P>(&mut self, x: u32, y: u32, p: P) where F: From<P> {
        let row = &mut self.as_slice_row_mut(y);
        row[x as usize] = p.into();
    }
    /// Get view of pixels as a slice.
    pub fn as_slice(&self) -> &[F] {
        &self.pixels
    }
    /// Get view of pixels as a mutable slice.
    pub fn as_slice_mut(&mut self) -> &mut [F] {
        &mut self.pixels
    }
    /// Get view of a row of pixels as a slice.
    pub fn as_slice_row(&self, y: u32) -> &[F] {
        debug_assert!(y < self.height);
        let s = (y * self.width) as usize;
        let t = s + self.width as usize;
        &self.pixels[s..t]
    }
    /// Get view of a row of pixels as a mutable slice.
    pub fn as_slice_row_mut(&mut self, y: u32) -> &mut [F] {
        debug_assert!(y < self.height);
        let s = (y * self.width) as usize;
        let t = s + self.width as usize;
        &mut self.pixels[s..t]
    }
    /// Get view of a row of pixels as a u8 slice.
    pub fn as_u8_slice_row(&self, y: u32) -> &[u8] {
        debug_assert!(y < self.height);
        let s = (y * self.width) as usize;
        let t = s + self.width as usize;
        Self::as_u8_slice_range(&self.pixels[s..t])
    }
    /// Get view of a pixel slice as a u8 slice.
    fn as_u8_slice_range(pix: &[F]) -> &[u8] {
        unsafe { pix.align_to::<u8>().1 }
    }
    /// Get view of pixels as a u8 slice.
    pub fn as_u8_slice(&self) -> &[u8] {
        Self::as_u8_slice_range(&self.pixels)
    }
    /// Get view of pixels as a mutable u8 slice.
    pub fn as_u8_slice_mut(&mut self) -> &mut [u8] {
        Self::as_u8_slice_range_mut(&mut self.pixels)
    }
    /// Get view of a pixel slice as a mutable u8 slice.
    fn as_u8_slice_range_mut(pix: &mut [F]) -> &mut [u8] {
        unsafe { pix.align_to_mut::<u8>().1 }
    }
    /// Clear all pixels.
    pub fn clear(&mut self) {
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
    pub fn set_rect<P>(&mut self, x: u32, y: u32, w: u32, h: u32, clr: P)
        where F: From<P>
    {
        let clr = clr.into();
        if y < self.height() && x < self.width() {
            let xm = self.width.min(x + w);
            let ym = self.height.min(y + h);
            let xrange = (x as usize)..(xm as usize);
            for yi in y..ym {
                self.as_slice_row_mut(yi)[xrange.clone()]
                    .iter_mut()
                    .for_each(|p| *p = clr);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::*;
    #[test]
    fn mask8() {
        let mut r = Raster::<Mask8>::new(3, 3);
        r.set_pixel(0, 0, 1.0);
        r.set_pixel(2, 0, 0x12);
        r.set_pixel(1, 1, 0x34);
        r.set_pixel(0, 2, 0x56);
        r.set_pixel(2, 2, 0x78);
        let v = vec![
            0xFF, 0x00, 0x12,
            0x00, 0x34, 0x00,
            0x56, 0x00, 0x78,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn mask16() {
        let mut r = Raster::<Mask16>::new(3, 3);
        r.set_pixel(2, 0, 0x9ABC);
        r.set_pixel(1, 1, 0x5678);
        r.set_pixel(0, 2, 0x1234);
        r.set_pixel(0, 0, 1.0);
        r.set_pixel(2, 2, 0x80u8);
        let v = vec![
            0xFF,0xFF, 0x00,0x00, 0xBC,0x9A,
            0x00,0x00, 0x78,0x56, 0x00,0x00,
            0x34,0x12, 0x00,0x00, 0x80,0x80,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn mask32() {
        let p: Vec<_> = vec![
            0.25, 0.5, 0.75, 1.0,
            0.5,  0.55, 0.7, 0.8,
            0.75, 0.65, 0.6, 0.4,
            1.0,  0.75, 0.5, 0.25,
        ].iter().map(|p| Mask::new(Ch32::new(*p))).collect();
        let mut r = Raster::<Mask32>::with_pixels(4, 4, p);
        let clr = Mask::new(Ch32::new(0.05));
        r.set_rect(1, 1, 2, 2, clr);
        let v: Vec<_> = vec![
            0.25, 0.5, 0.75, 1.0,
            0.5,  0.05, 0.05, 0.8,
            0.75, 0.05, 0.05, 0.4,
            1.0,  0.75, 0.5, 0.25,
        ].iter().map(|p| Mask::new(Ch32::new(*p))).collect();
        let r2 = Raster::<Mask32>::with_pixels(4, 4, v);
        assert_eq!(r.as_slice(), r2.as_slice());
    }
    #[test]
    fn rgb8() {
        let mut r = Raster::<Rgb8>::new(4, 4);
        let rgb: Rgb8 = Rgb::new(0xCC, 0xAA, 0xBB);
        r.set_rect(1, 1, 2, 2, rgb);
        let v = vec![
            0x00,0x00,0x00, 0x00,0x00,0x00, 0x00,0x00,0x00, 0x00,0x00,0x00,
            0x00,0x00,0x00, 0xCC,0xAA,0xBB, 0xCC,0xAA,0xBB, 0x00,0x00,0x00,
            0x00,0x00,0x00, 0xCC,0xAA,0xBB, 0xCC,0xAA,0xBB, 0x00,0x00,0x00,
            0x00,0x00,0x00, 0x00,0x00,0x00, 0x00,0x00,0x00, 0x00,0x00,0x00,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn gray8() {
        let mut r = Raster::<Gray8>::new(4, 4);
        r.set_rect(0, 0, 1, 1, Gray::from(0x23));
        r.set_rect(10, 10, 1, 1, Gray::from(0x45));
        r.set_rect(2, 2, 10, 10, 0xBB);
        let v = vec![
            0x23,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,
            0x00,0x00,0xBB,0xBB,
            0x00,0x00,0xBB,0xBB,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn rgb8_buffer() {
        let b = vec![
            0xAA,0x00,0x00, 0x00,0x11,0x22, 0x33,0x44,0x55,
            0x00,0xBB,0x00, 0x66,0x77,0x88, 0x99,0xAA,0xBB,
            0x00,0x00,0xCC, 0xCC,0xDD,0xEE, 0xFF,0x00,0x11,
        ];
        let mut r = unsafe { Raster::<Rgb8>::with_buffer(3, 3, b) };
        let rgb: Rgb8 = Rgb::new(0x12, 0x34, 0x56);
        r.set_rect(0, 1, 2, 1, rgb);
        let v = vec![
            0xAA,0x00,0x00, 0x00,0x11,0x22, 0x33,0x44,0x55,
            0x12,0x34,0x56, 0x12,0x34,0x56, 0x99,0xAA,0xBB,
            0x00,0x00,0xCC, 0xCC,0xDD,0xEE, 0xFF,0x00,0x11,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn grayalpha16_buffer() {
        let b = vec![
            0x01,0x10,0x05,0x50, 0x00,0x10,0x02,0x30, 0x04,0x50,0x06,0x70,
            0x02,0x20,0x06,0x60, 0x08,0x90,0x0A,0xB0, 0x0C,0xD0,0x0E,0xF0,
            0x03,0x30,0x07,0x70, 0x0F,0xE0,0x0D,0xC0, 0x0B,0xA0,0x09,0x80,
        ];
        let mut r = unsafe { Raster::<GrayAlpha16>::with_buffer(3, 3, b) };
        let c = Gray::new(0x4444);
        r.set_rect(1, 0, 2, 2, c);
        let v = vec![
            0x01,0x10,0x05,0x50, 0x44,0x44,0xFF,0xFF, 0x44,0x44,0xFF,0xFF,
            0x02,0x20,0x06,0x60, 0x44,0x44,0xFF,0xFF, 0x44,0x44,0xFF,0xFF,
            0x03,0x30,0x07,0x70, 0x0F,0xE0,0x0D,0xC0, 0x0B,0xA0,0x09,0x80,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
}
