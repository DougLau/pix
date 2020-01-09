// raster.rs    Raster images.
//
// Copyright (c) 2017-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::{Ch16, Ch8, Channel, Format};
use std::convert::TryFrom;
use std::marker::PhantomData;

/// Builder for [Raster](struct.Raster.html) images.
///
/// After creating a `RasterBuilder`, the [AlphaMode](enum.AlphaMode.html) and
/// [GammaMode](enum.GammaMode.html) can be configured.  To finish building a
/// `Raster`, use one of the *with_* methods:
/// * [with_clear](struct.RasterBuilder.html#method.with_clear)
/// * [with_color](struct.RasterBuilder.html#method.with_color)
/// * [with_raster](struct.RasterBuilder.html#method.with_raster)
/// * [with_pixels](struct.RasterBuilder.html#method.with_pixels)
/// * [with_u8_buffer](struct.RasterBuilder.html#method.with_u8_buffer)
/// * [with_u16_buffer](struct.RasterBuilder.html#method.with_u16_buffer)
///
/// ### Create a `Raster`
/// ```
/// # use pix::*;
/// let r = RasterBuilder::<SepSRgb8>::new().with_clear(100, 100);
/// ```
pub struct RasterBuilder<F: Format> {
    _format: PhantomData<F>,
}

/// `Raster` image representing a two-dimensional array of pixels.
///
/// ### Create a `Raster` with a solid color rectangle
/// ```
/// # use pix::*;
/// let mut raster = RasterBuilder::<SepSRgb8>::new().with_clear(10, 10);
/// raster.set_region((2, 4, 3, 3), SepSRgb8::new(0xFF, 0xFF, 0x00));
/// ```
pub struct Raster<F: Format> {
    width: u32,
    height: u32,
    pixels: Box<[F]>,
}

/// `Iterator` for pixels within a [Raster](struct.Raster.html).
///
/// Use `Raster`::[region_iter](struct.Raster.html#method.region_iter) to
/// create.
///
/// ### All pixels in a `Raster`
/// ```
/// # use pix::*;
/// let mut mask = RasterBuilder::<Mask8>::new().with_clear(32, 32);
/// // ... set mask data
/// let it = mask.region_iter(mask.region());
/// ```
///
/// ### `Iterator` of `Region` within a `Raster`
/// ```
/// # use pix::*;
/// let mut gray = RasterBuilder::<SepSGrayAlpha16>::new().with_clear(40, 40);
/// // ... load raster data
/// let region = gray.region().intersection((20, 20, 10, 10));
/// let it = gray.region_iter(region);
/// ```
pub struct RasterIter<'a, F: Format> {
    raster: &'a Raster<F>,
    left: u32,
    right: u32,
    bottom: u32,
    x: u32,
    y: u32,
}

/// Location / dimensions of pixels relative to a [Raster](struct.Raster.html).
///
/// ### Create directly
/// ```
/// # use pix::*;
/// let r0 = Region::new(80, 20, 120, 280);
/// let r1 = r0.intersection((50, 40, 360, 240));
/// ```
/// ### Create from Raster
/// ```
/// # use pix::*;
/// let r = RasterBuilder::<SepSRgb8>::new().with_clear(100, 100);
/// let reg = r.region(); // (0, 0, 100, 100)
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Region {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
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

impl<F: Format> Default for RasterBuilder<F> {
    fn default() -> Self {
        Self::new()
    }
}

impl<F: Format> RasterBuilder<F> {
    /// Create a new raster builder.
    ///
    /// * `F` Pixel [Format](trait.Format.html).
    pub fn new() -> Self {
        let _format = PhantomData;
        RasterBuilder { _format }
    }
    /// Build a `Raster` with all pixels clear.
    ///
    /// ## Examples
    /// ```
    /// # use pix::*;
    /// let r1 = RasterBuilder::<SepSGray8>::new().with_clear(20, 20);
    /// let r2 = RasterBuilder::<Mask8>::new().with_clear(64, 64);
    /// let r3 = RasterBuilder::<SepSRgb16>::new().with_clear(10, 10);
    /// let r4 = RasterBuilder::<SepSGrayAlpha32>::new().with_clear(100, 250);
    /// ```
    pub fn with_clear(self, width: u32, height: u32) -> Raster<F> {
        self.with_color(width, height, F::default())
    }
    /// Build a `Raster` with all pixels set to one color.
    ///
    /// ## Example
    /// ```
    /// # use pix::*;
    /// let clr = SepSRgb8::new(0x40, 0xAA, 0xBB);
    /// let r = RasterBuilder::<SepSRgb8>::new().with_clear(15, 15);
    /// ```
    pub fn with_color(self, width: u32, height: u32, clr: F) -> Raster<F> {
        let len = (width * height) as usize;
        let pixels = vec![clr; len].into_boxed_slice();
        Raster {
            width,
            height,
            pixels,
        }
    }
    /// Build a `Raster` by copying another `Raster`.
    ///
    /// * `C` Destination `Channel`.
    /// * `H` Source `Channel`.
    /// * `P` Source `Format`.
    ///
    /// ### Convert from Rgb8 to Rgba16
    /// ```
    /// # use pix::*;
    /// let mut r0 = RasterBuilder::<SepSRgb8>::new().with_clear(50, 50);
    /// // load pixels into raster
    /// let r1 = RasterBuilder::<SepSRgba16>::new().with_raster(&r0);
    /// ```
    pub fn with_raster<C, H, P>(self, o: &Raster<P>) -> Raster<F>
    where
        C: Channel + From<H>,
        H: Channel,
        F: Format<Chan = C>,
        P: Format<Chan = H>,
    {
        let mut r = RasterBuilder::new().with_clear(o.width(), o.height());
        let reg = o.region();
        r.set_region(reg, o.region_iter(reg));
        r
    }
    /// Build a `Raster` with owned pixel data.  You can get ownership of the
    /// pixel data back from the `Raster` as either a `Vec<F>` or a `Box<[F]>`
    /// by calling `into()`.
    ///
    /// * `B` Owned pixed type (`Vec` or boxed slice).
    /// * `width` Width of `Raster`.
    /// * `height` Height of `Raster`.
    /// * `pixels` Pixel data.
    ///
    /// # Panics
    ///
    /// Panics if `pixels` length is not equal to `width` * `height`.
    ///
    /// ## Example
    /// ```
    /// # use pix::*;
    /// let p = vec![SepSRgb8::new(255, 0, 255); 16]; // vec of magenta pix
    /// let mut r = RasterBuilder::new()          // convert to raster
    ///     .with_pixels(4, 4, p);
    /// let clr = SepSRgb8::new(0x00, 0xFF, 0x00);    // green
    /// r.set_region((2, 0, 1, 3), clr);          // make stripe
    /// let p2 = Into::<Vec<SepSRgb8>>::into(r);      // convert back to vec
    /// ```
    pub fn with_pixels<B>(self, width: u32, height: u32, pixels: B) -> Raster<F>
    where
        B: Into<Box<[F]>>,
    {
        let len = (width * height) as usize;
        let pixels = pixels.into();
        assert_eq!(len, pixels.len());
        Raster {
            width,
            height,
            pixels,
        }
    }
    /// Build a `Raster` from a `u8` buffer.
    ///
    /// * `B` Owned pixed type (`Vec` or boxed slice).
    /// * `width` Width of `Raster`.
    /// * `height` Height of `Raster`.
    /// * `buffer` Buffer of pixel data.
    ///
    /// # Panics
    ///
    /// Panics if `buffer` length is not equal to `width` * `height` *
    /// `std::mem::size_of::<F>()`.
    pub fn with_u8_buffer<B>(
        self,
        width: u32,
        height: u32,
        buffer: B,
    ) -> Raster<F>
    where
        B: Into<Box<[u8]>>,
        F: Format<Chan = Ch8>,
    {
        let len = (width * height) as usize;
        let buffer: Box<[u8]> = buffer.into();
        let capacity = buffer.len();
        assert_eq!(len * std::mem::size_of::<F>(), capacity);
        let slice = std::boxed::Box::<[u8]>::into_raw(buffer);
        let pixels: Box<[F]> = unsafe {
            let ptr = (*slice).as_mut_ptr() as *mut F;
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            Box::from_raw(slice)
        };
        Raster {
            width,
            height,
            pixels,
        }
    }
    /// Build a `Raster` from a `u16` buffer.
    ///
    /// * `B` Owned pixed type (`Vec` or boxed slice).
    /// * `width` Width of `Raster`.
    /// * `height` Height of `Raster`.
    /// * `buffer` Buffer of pixel data (in native-endian byte order).
    ///
    /// # Panics
    ///
    /// Panics if `buffer` length is not equal to `width` * `height` *
    /// `std::mem::size_of::<F>()`.
    pub fn with_u16_buffer<B>(
        self,
        width: u32,
        height: u32,
        buffer: B,
    ) -> Raster<F>
    where
        B: Into<Box<[u16]>>,
        F: Format<Chan = Ch16>,
    {
        let len = (width * height) as usize;
        let buffer: Box<[u16]> = buffer.into();
        let capacity = buffer.len();
        assert_eq!(
            len * std::mem::size_of::<F>(),
            capacity * std::mem::size_of::<u16>()
        );
        let slice = std::boxed::Box::<[u16]>::into_raw(buffer);
        let pixels: Box<[F]> = unsafe {
            let ptr = (*slice).as_mut_ptr() as *mut F;
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            Box::from_raw(slice)
        };
        Raster {
            width,
            height,
            pixels,
        }
    }
}

impl<F: Format> Raster<F> {
    /// Get width of `Raster`.
    pub fn width(&self) -> u32 {
        self.width
    }
    /// Get height of `Raster`.
    pub fn height(&self) -> u32 {
        self.height
    }
    /// Get one pixel value.
    pub fn pixel(&self, x: u32, y: u32) -> F {
        let row = &self.as_slice_row(y);
        row[x as usize]
    }
    /// Set one pixel value.
    pub fn set_pixel<P>(&mut self, x: u32, y: u32, p: P)
    where
        F: From<P>,
    {
        let row = &mut self.as_slice_row_mut(y);
        row[x as usize] = p.into();
    }
    /// Clear all pixels to [Format](trait.Format.html) default.
    pub fn clear(&mut self) {
        for p in self.pixels.iter_mut() {
            *p = F::default();
        }
    }
    /// Get `Region` of entire `Raster`.
    pub fn region(&self) -> Region {
        Region::new(0, 0, self.width(), self.height())
    }
    /// Get an `Iterator` of pixels within a `Region`.
    ///
    /// * `reg` Region within `Raster`.
    pub fn region_iter<R>(&self, reg: R) -> RasterIter<F>
    where
        R: Into<Region>,
    {
        RasterIter::new(self, reg.into())
    }
    /// Set a `Region` using a pixel `Iterator`.
    ///
    /// * `reg` Region within `Raster`.
    /// * `it` `Iterator` of pixels in `Region`.
    ///
    /// ### Set entire raster to one color
    /// ```
    /// # use pix::*;
    /// let mut r = RasterBuilder::<SepSRgb32>::new().with_clear(360, 240);
    /// r.set_region(r.region(), SepSRgb32::new(0.5, 0.2, 0.8));
    /// ```
    /// ### Set rectangle to solid color
    /// ```
    /// # use pix::*;
    /// let mut raster = RasterBuilder::<SepSRgb8>::new().with_clear(100, 100);
    /// raster.set_region((20, 40, 25, 50), SepSRgb8::new(0xDD, 0x96, 0x70));
    /// ```
    /// ### Copy part of one `Raster` to another, converting pixel format
    /// ```
    /// # use pix::*;
    /// let mut rgb = RasterBuilder::<SepSRgb8>::new().with_clear(100, 100);
    /// let mut gray = RasterBuilder::<SepSGray16>::new().with_clear(50, 50);
    /// // ... load image data
    /// let src = gray.region().intersection((20, 10, 25, 25));
    /// let dst = rgb.region().intersection((40, 40, 25, 25));
    /// // Regions must have the same shape!
    /// rgb.set_region(dst, gray.region_iter(src));
    /// ```
    pub fn set_region<C, R, I, P, H>(&mut self, reg: R, mut it: I)
    where
        F: Format<Chan = C>,
        C: Channel + From<H>,
        H: Channel,
        P: Format<Chan = H>,
        R: Into<Region>,
        I: Iterator<Item = P>,
    {
        let reg = reg.into();
        let x0 = if reg.x >= 0 {
            reg.x as u32
        } else {
            self.width()
        };
        let x1 = self.width().min(x0 + reg.width);
        let (x0, x1) = (x0 as usize, x1 as usize);
        let y0 = if reg.y >= 0 {
            reg.y as u32
        } else {
            self.height()
        };
        let y1 = self.height().min(y0 + reg.height);
        if y0 < y1 && x0 < x1 {
            for yi in y0..y1 {
                let row = self.as_slice_row_mut(yi);
                for x in x0..x1 {
                    if let Some(p) = it.next() {
                        row[x] = p.convert();
                    }
                }
            }
        }
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
    /// Get view of a row of pixels as a `u8` slice.
    pub fn as_u8_slice_row(&self, y: u32) -> &[u8] {
        debug_assert!(y < self.height);
        let s = (y * self.width) as usize;
        let t = s + self.width as usize;
        Self::u8_slice(&self.pixels[s..t])
    }
    /// Get view of a pixel slice as a `u8` slice.
    fn u8_slice(pix: &[F]) -> &[u8] {
        unsafe { pix.align_to::<u8>().1 }
    }
    /// Get view of pixels as a `u8` slice.
    pub fn as_u8_slice(&self) -> &[u8] {
        Self::u8_slice(&self.pixels)
    }
    /// Get view of pixels as a mutable `u8` slice.
    pub fn as_u8_slice_mut(&mut self) -> &mut [u8] {
        Self::u8_slice_mut(&mut self.pixels)
    }
    /// Get view of a pixel slice as a mutable `u8` slice.
    fn u8_slice_mut(pix: &mut [F]) -> &mut [u8] {
        unsafe { pix.align_to_mut::<u8>().1 }
    }
}

impl<'a, F: Format> RasterIter<'a, F> {
    /// Create a new `Raster` pixel `Iterator`.
    ///
    /// * `region` Region of pixels to iterate.
    fn new(raster: &'a Raster<F>, region: Region) -> Self {
        let y = u32::try_from(region.y).unwrap_or(0);
        let bottom = u32::try_from(region.bottom()).unwrap_or(0);
        let x = u32::try_from(region.x).unwrap_or(0);
        let right = u32::try_from(region.right()).unwrap_or(0);
        let left = x;
        RasterIter {
            raster,
            left,
            right,
            bottom,
            x,
            y,
        }
    }
}

impl<'a, F: Format> Iterator for RasterIter<'a, F> {
    type Item = F;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.right {
            self.x = self.left;
            self.y += 1;
            if self.y >= self.bottom {
                return None;
            }
        }
        let p = self.raster.pixel(self.x, self.y);
        self.x += 1;
        Some(p)
    }
}

impl From<(i32, i32, u32, u32)> for Region {
    fn from(r: (i32, i32, u32, u32)) -> Self {
        Region::new(r.0, r.1, r.2, r.3)
    }
}

impl Region {
    /// Create a new `Region`
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Region {
            x,
            y,
            width,
            height,
        }
    }
    /// Get intersection with another `Region`
    pub fn intersection<R>(self, rhs: R) -> Self
    where
        R: Into<Self>,
    {
        let rhs = rhs.into();
        let x0 = self.x.max(rhs.x);
        let x1 = self.right().min(rhs.right());
        let w = (x1 - x0) as u32;
        let y0 = self.y.max(rhs.y);
        let y1 = self.bottom().min(rhs.bottom());
        let h = (y1 - y0) as u32;
        Region::new(x0, y0, w, h)
    }
    /// Get right side
    fn right(self) -> i32 {
        let x = i64::from(self.x) + i64::from(self.width);
        if x < std::i32::MAX.into() {
            x as i32
        } else {
            self.x
        }
    }
    /// Get bottom side
    fn bottom(self) -> i32 {
        let y = i64::from(self.y) + i64::from(self.height);
        if y < std::i32::MAX.into() {
            y as i32
        } else {
            self.y
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::*;
    use super::*;
    #[test]
    fn mask8() {
        let mut r = RasterBuilder::<Mask8>::new().with_clear(3, 3);
        r.set_pixel(0, 0, 0xFF);
        r.set_pixel(2, 0, 0x12);
        r.set_pixel(1, 1, 0x34);
        r.set_pixel(0, 2, 0x56);
        r.set_pixel(2, 2, 0x78);
        let v = vec![0xFF, 0x00, 0x12, 0x00, 0x34, 0x00, 0x56, 0x00, 0x78];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn mask16() {
        let mut r = RasterBuilder::<Mask16>::new().with_clear(3, 3);
        r.set_pixel(2, 0, 0x9ABC);
        r.set_pixel(1, 1, 0x5678);
        r.set_pixel(0, 2, 0x1234);
        r.set_pixel(0, 0, 0xFFFF);
        r.set_pixel(2, 2, 0x8080);
        let v = vec![
            0xFF, 0xFF, 0x00, 0x00, 0xBC, 0x9A, 0x00, 0x00, 0x78, 0x56, 0x00,
            0x00, 0x34, 0x12, 0x00, 0x00, 0x80, 0x80,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn mask32() {
        let p: Vec<_> = vec![
            0.25, 0.5, 0.75, 1.0, 0.5, 0.55, 0.7, 0.8, 0.75, 0.65, 0.6, 0.4,
            1.0, 0.75, 0.5, 0.25,
        ]
        .iter()
        .map(|p| Mask::new(Ch32::new(*p)))
        .collect();
        let mut r = RasterBuilder::<Mask32>::new().with_pixels(4, 4, p);
        let clr = Mask32::new(Ch32::new(0.05));
        r.set_region((1, 1, 2, 2), clr);
        let v: Vec<_> = vec![
            0.25, 0.5, 0.75, 1.0, 0.5, 0.05, 0.05, 0.8, 0.75, 0.05, 0.05, 0.4,
            1.0, 0.75, 0.5, 0.25,
        ]
        .iter()
        .map(|p| Mask::new(Ch32::new(*p)))
        .collect();
        let r2 = RasterBuilder::<Mask32>::new().with_pixels(4, 4, v);
        assert_eq!(r.as_slice(), r2.as_slice());
    }
    #[test]
    fn rgb8() {
        let mut r = RasterBuilder::<SepSRgb8>::new().with_clear(4, 4);
        let rgb = SepSRgb8::new(0xCC, 0xAA, 0xBB);
        r.set_region((1, 1, 2, 2), rgb);
        let v = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xCC, 0xAA, 0xBB, 0xCC, 0xAA, 0xBB, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0xCC, 0xAA, 0xBB, 0xCC, 0xAA, 0xBB,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn gray8() {
        let mut r = RasterBuilder::<SepSGray8>::new().with_clear(4, 4);
        r.set_region((0, 0, 1, 1), SepSGray8::from(0x23));
        r.set_region((10, 10, 1, 1), SepSGray8::from(0x45));
        r.set_region((2, 2, 10, 10), SepSGray8::from(0xBB));
        let v = vec![
            0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xBB,
            0xBB, 0x00, 0x00, 0xBB, 0xBB,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn rgb8_buffer() {
        let b = vec![
            0xAA, 0x00, 0x00, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x00, 0xBB,
            0x00, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0x00, 0x00, 0xCC, 0xCC,
            0xDD, 0xEE, 0xFF, 0x00, 0x11,
        ];
        let mut r = RasterBuilder::<SepSRgb8>::new().with_u8_buffer(3, 3, b);
        let rgb = SepSRgb8::new(0x12, 0x34, 0x56);
        r.set_region((0, 1, 2, 1), rgb);
        let v = vec![
            0xAA, 0x00, 0x00, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x12, 0x34,
            0x56, 0x12, 0x34, 0x56, 0x99, 0xAA, 0xBB, 0x00, 0x00, 0xCC, 0xCC,
            0xDD, 0xEE, 0xFF, 0x00, 0x11,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn grayalpha16_buffer() {
        let b = vec![
            0x1001, 0x5005, 0x1000, 0x3002, 0x5004, 0x7006, 0x2002, 0x6006,
            0x9008, 0xB00A, 0xD00C, 0xF00E, 0x3003, 0x7007, 0xE00F, 0xC00D,
            0xA00B, 0x8009,
        ];
        let mut r =
            RasterBuilder::<SepSGrayAlpha16>::new().with_u16_buffer(3, 3, b);
        r.set_region((1, 0, 2, 2), SepSGrayAlpha16::new(0x4444));
        let v = vec![
            0x01, 0x10, 0x05, 0x50, 0x44, 0x44, 0xFF, 0xFF, 0x44, 0x44, 0xFF,
            0xFF, 0x02, 0x20, 0x06, 0x60, 0x44, 0x44, 0xFF, 0xFF, 0x44, 0x44,
            0xFF, 0xFF, 0x03, 0x30, 0x07, 0x70, 0x0F, 0xE0, 0x0D, 0xC0, 0x0B,
            0xA0, 0x09, 0x80,
        ];
        // FIXME: this will fail on big-endian archs
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn gray_to_rgb() {
        let mut r = RasterBuilder::<SepSGray8>::new().with_clear(3, 3);
        r.set_region((2, 0, 4, 2), SepSGray8::new(0x45));
        r.set_region((0, 2, 2, 10), SepSGray8::new(0xDA));
        let r = RasterBuilder::<SepSRgb8>::new().with_raster(&r);
        let v = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x45, 0x45, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x45, 0x45, 0x45, 0xDA, 0xDA, 0xDA, 0xDA,
            0xDA, 0xDA, 0x00, 0x00, 0x00,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn rgb_to_gray() {
        let mut r = RasterBuilder::<SepSRgb16>::new().with_clear(3, 3);
        r.set_region((1, 0, 4, 2), SepSRgb16::new(0x4321, 0x9085, 0x5543));
        r.set_region((0, 1, 1, 10), SepSRgb16::new(0x5768, 0x4091, 0x5000));
        let r = RasterBuilder::<SepSGray8>::new().with_raster(&r);
        let v = vec![0x00, 0x90, 0x90, 0x57, 0x90, 0x90, 0x57, 0x00, 0x00];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn gray_to_mask() {
        let mut r = RasterBuilder::<SepSGrayAlpha8>::new().with_clear(3, 3);
        r.set_region((0, 1, 2, 8), SepSGrayAlpha8::with_alpha(0x67, 0x94));
        r.set_region((2, 0, 1, 10), SepSGrayAlpha8::with_alpha(0xBA, 0xA2));
        let r = RasterBuilder::<Mask16>::new().with_raster(&r);
        let v = vec![
            0x00, 0x00, 0x00, 0x00, 0xA2, 0xA2, 0x94, 0x94, 0x94, 0x94, 0xA2,
            0xA2, 0x94, 0x94, 0x94, 0x94, 0xA2, 0xA2,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn mask_to_gray() {
        let mut r = RasterBuilder::<Mask16>::new().with_clear(3, 3);
        r.set_region((0, 1, 3, 8), Mask16::new(0xABCD));
        r.set_region((2, 0, 1, 3), Mask16::new(0x9876));
        let r = RasterBuilder::<SepSGrayAlpha8>::new().with_raster(&r);
        let v = vec![
            0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x98, 0xFF, 0xAB, 0xFF, 0xAB, 0xFF,
            0x98, 0xFF, 0xAB, 0xFF, 0xAB, 0xFF, 0x98,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn copy_region_gray() {
        let mut g0 = RasterBuilder::<SepSGray16>::new().with_clear(3, 3);
        let mut g1 = RasterBuilder::<SepLGray16>::new().with_clear(3, 3);
        g0.set_region((0, 2, 2, 5), SepSGray16::new(0x4455));
        g0.set_region((2, 0, 3, 2), SepSGray8::new(0x33));
        g1.set_region(g1.region(), g0.region_iter(g0.region()));
        let v = vec![
            0x00, 0x00, 0x00, 0x00, 0x7A, 0x08, 0x00, 0x00, 0x00, 0x00, 0x7A,
            0x08, 0xD4, 0x0E, 0xD4, 0x0E, 0x00, 0x00,
        ];
        assert_eq!(g1.as_u8_slice(), &v[..]);
    }
    #[test]
    fn from_rgb8() {
        let r = RasterBuilder::<SepSRgb8>::new().with_clear(50, 50);
        let _ = RasterBuilder::<SepSRgb16>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSRgb32>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSRgba8>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSRgba16>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSRgba32>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSGray8>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSGray16>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSGray32>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSGrayAlpha8>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSGrayAlpha16>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSGrayAlpha32>::new().with_raster(&r);
        let _ = RasterBuilder::<Mask8>::new().with_raster(&r);
        let _ = RasterBuilder::<Mask16>::new().with_raster(&r);
        let _ = RasterBuilder::<Mask32>::new().with_raster(&r);
    }
    #[test]
    fn from_mask8() {
        let r = RasterBuilder::<Mask8>::new().with_clear(50, 50);
        let _ = RasterBuilder::<SepSRgb8>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSRgb16>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSRgb32>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSRgba8>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSRgba16>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSRgba32>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSGray8>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSGray16>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSGray32>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSGrayAlpha8>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSGrayAlpha16>::new().with_raster(&r);
        let _ = RasterBuilder::<SepSGrayAlpha32>::new().with_raster(&r);
        let _ = RasterBuilder::<Mask8>::new().with_raster(&r);
        let _ = RasterBuilder::<Mask16>::new().with_raster(&r);
        let _ = RasterBuilder::<Mask32>::new().with_raster(&r);
    }
    #[test]
    fn region_size() {
        assert_eq!(std::mem::size_of::<Region>(), 16);
    }
    #[test]
    fn intersect() -> Result<(), ()> {
        let r = Region::new(0, 0, 5, 5);
        assert_eq!(r, Region::new(0, 0, 5, 5));
        assert_eq!(r, r.intersection(Region::new(0, 0, 10, 10)));
        assert_eq!(r, r.intersection(Region::new(-5, -5, 10, 10)));
        assert_eq!(
            Region::new(0, 0, 4, 4),
            r.intersection(Region::new(-1, -1, 5, 5))
        );
        assert_eq!(
            Region::new(1, 2, 1, 3),
            r.intersection(Region::new(1, 2, 1, 100))
        );
        assert_eq!(
            Region::new(2, 1, 3, 1),
            r.intersection(Region::new(2, 1, 100, 1))
        );
        Ok(())
    }
}
