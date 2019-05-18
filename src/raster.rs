// raster.rs    Raster images.
//
// Copyright (c) 2017-2019  Douglas P Lau
//
use crate::{AlphaMode, Ch8, Ch16, Channel, Format, GammaMode, PixModes};
use std::convert::TryFrom;
use std::marker::PhantomData;

/// Builder for [Raster](struct.Raster.html) images.
///
/// ### Create with [Raster::new](struct.Raster.html#method.new)
/// ```
/// # use pix::*;
/// let rb = Raster::<Rgb8>::new(100, 100);
/// ```
///
/// To build a `Raster`, use one of the *with_* methods:
/// * [with_empty](struct.RasterBuilder.html#method.with_empty)
/// * [with_pixels](struct.RasterBuilder.html#method.with_pixels)
/// * [with_u8_buffer](struct.RasterBuilder.html#method.with_u8_buffer)
/// * [with_u16_buffer](struct.RasterBuilder.html#method.with_u16_buffer)
pub struct RasterBuilder<F: Format> {
    width     : u32,
    height    : u32,
    alpha_mode: AlphaMode,
    gamma_mode: GammaMode,
    _format   : PhantomData<F>,
}

/// `Raster` image representing a two-dimensional array of pixels.
///
/// ```
/// # use pix::*;
/// let clr = Rgb8::new(0xFF, 0x88, 0x00);
/// let mut raster = Raster::<Rgb8>::new(10, 10).with_empty();
/// raster.set_region((2, 4, 3, 3), clr);
/// ```
pub struct Raster<F: Format> {
    width     : u32,
    height    : u32,
    alpha_mode: AlphaMode,
    gamma_mode: GammaMode,
    pixels    : Box<[F]>,
}

/// `Iterator` for pixels within a [Raster](struct.Raster.html).
///
/// Use Raster::[region_iter](struct.Raster.html#method.region_iter) to
/// create.
///
/// ### All pixels in a `Raster`
/// ```
/// # use pix::*;
/// let mut mask = Raster::<Mask8>::new(32, 32).with_empty();
/// // ... set mask data
/// let it = mask.region_iter(mask.region());
/// ```
///
/// ### `Iterator` of `Region` within a `Raster`
/// ```
/// # use pix::*;
/// let mut gray = Raster::<GrayAlpha16>::new(40, 40).with_empty();
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
/// let r = Raster::<Rgb8>::new(100, 100).with_empty();
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

impl<F: Format> RasterBuilder<F> {
    /// Create a new raster builder.
    fn new(width: u32, height: u32) -> Self {
        let alpha_mode = AlphaMode::Separated;
        let gamma_mode = GammaMode::Linear;
        let _format = PhantomData;
        RasterBuilder { width, height, alpha_mode, gamma_mode, _format }
    }
    /// Set the alpha mode.  The default value is
    /// [Separated](enum.AlphaMode.html#variant.Separated).
    pub fn alpha_mode(mut self, alpha_mode: AlphaMode) -> Self {
        self.alpha_mode = alpha_mode;
        self
    }
    /// Set the gamma mode.  The default value is
    /// [Linear](enum.GammaMode.html#variant.Linear).
    pub fn gamma_mode(mut self, gamma_mode: GammaMode) -> Self {
        self.gamma_mode = gamma_mode;
        self
    }
    /// Build a [Raster](struct.Raster.html) with empty pixel data.
    pub fn with_empty(self) -> Raster<F> {
        let width = self.width;
        let height = self.height;
        let alpha_mode = self.alpha_mode;
        let gamma_mode = self.gamma_mode;
        let len = (width * height) as usize;
        let pixels = vec![F::default(); len].into_boxed_slice();
        Raster { width, height, alpha_mode, gamma_mode, pixels }
    }
    /// Build a `Raster` with owned pixel data.  You can get ownership of the
    /// pixel data back from the `Raster` as either a `Vec<F>` or a `Box<[F]>`
    /// by calling `into()`.
    ///
    /// * `F` Pixel [Format](trait.Format.html).
    /// * `pixels` Pixel data.
    ///
    /// # Panics
    ///
    /// Panics if `pixels` length is not equal to `width` * `height`.
    ///
    /// ## Example
    /// ```
    /// use pix::*;
    /// let p = vec![Rgb8::new(255, 0, 255); 16];     // vec of magenta pix
    /// let mut r = Raster::new(4, 4).with_pixels(p); // convert to raster
    /// let clr = Rgb8::new(0x00, 0xFF, 0x00);        // green
    /// r.set_region((2, 0, 1, 3), clr);              // make stripe
    /// let p2 = Into::<Vec<Rgb8>>::into(r);          // convert back to vec
    /// ```
    pub fn with_pixels<B>(self, pixels: B) -> Raster<F>
        where B: Into<Box<[F]>>
    {
        let width = self.width;
        let height = self.height;
        let alpha_mode = self.alpha_mode;
        let gamma_mode = self.gamma_mode;
        let len = (width * height) as usize;
        let pixels = pixels.into();
        assert_eq!(len, pixels.len());
        Raster { alpha_mode, gamma_mode, width, height, pixels }
    }
    /// Build a `Raster` from a u8 buffer.
    ///
    /// * `F` Pixel [Format](trait.Format.html).
    /// * `buffer` Buffer of pixel data.
    ///
    /// # Panics
    ///
    /// Panics if `buffer` length is not equal to `width` * `height` *
    /// `std::mem::size_of::<F>()`.
    pub fn with_u8_buffer<B>(self, buffer: B) -> Raster<F>
        where B: Into<Box<[u8]>>, F: Format<Chan=Ch8>
    {
        let width = self.width;
        let height = self.height;
        let alpha_mode = self.alpha_mode;
        let gamma_mode = self.gamma_mode;
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
        Raster { alpha_mode, gamma_mode, width, height, pixels }
    }
    /// Build a `Raster` from a u16 buffer.
    ///
    /// * `F` Pixel [Format](trait.Format.html).
    /// * `buffer` Buffer of pixel data (in native-endian byte order).
    ///
    /// # Panics
    ///
    /// Panics if `buffer` length is not equal to `width` * `height` *
    /// `std::mem::size_of::<F>()`.
    pub fn with_u16_buffer<B>(self, buffer: B) -> Raster<F>
        where B: Into<Box<[u16]>>, F: Format<Chan=Ch16>
    {
        let width = self.width;
        let height = self.height;
        let alpha_mode = self.alpha_mode;
        let gamma_mode = self.gamma_mode;
        let len = (width * height) as usize;
        let buffer: Box<[u16]> = buffer.into();
        let capacity = buffer.len();
        assert_eq!(len * std::mem::size_of::<F>(),
            capacity * std::mem::size_of::<u16>());
        let slice = std::boxed::Box::<[u16]>::into_raw(buffer);
        let pixels: Box<[F]> = unsafe {
            let ptr = (*slice).as_mut_ptr() as *mut F;
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            Box::from_raw(slice)
        };
        Raster { alpha_mode, gamma_mode, width, height, pixels }
    }
}

impl<F: Format> Raster<F> {
    /// Create a new raster builder.
    ///
    /// * `F` Pixel [Format](trait.Format.html).
    /// * `width` Width in pixels.
    /// * `height` Height in pixels.
    ///
    /// ## Examples
    /// ```
    /// use pix::*;
    /// let rb1 = Raster::<Gray8>::new(20, 20);
    /// let rb2 = Raster::<Mask8>::new(64, 64);
    /// let rb3 = Raster::<Rgb16>::new(10, 10);
    /// let rb4 = Raster::<GrayAlpha32>::new(100, 150);
    /// ```
    pub fn new(width: u32, height: u32) -> RasterBuilder<F> {
        RasterBuilder::new(width, height)
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
    pub fn set_pixel<P>(&mut self, x: u32, y: u32, p: P)
        where F: From<P>
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
    /// Get an iterator of pixels.
    ///
    /// * `reg` Region within `Raster`.
    pub fn region_iter<R>(&self, reg: R) -> RasterIter<F>
        where R: Into<Region>
    {
        RasterIter::new(self, reg.into())
    }
    /// Set a `Region` using a pixel iterator.
    ///
    /// * `reg` Region within raster.
    /// * `it` Iterator of pixels in region.
    ///
    /// ### Set rectangle to solid color
    /// ```
    /// # use pix::*;
    /// let mut raster = Raster::<Rgb8>::new(100, 100).with_empty();
    /// let clr = Rgb8::new(0xDD, 0x96, 0x70);
    /// raster.set_region((20, 40, 25, 50), clr);
    /// ```
    /// ### Copy part of one `Raster` to another, converting pixel format
    /// ```
    /// # use pix::*;
    /// let mut rgb = Raster::<Rgb8>::new(100, 100).with_empty();
    /// let mut gray = Raster::<Gray16>::new(50, 50).with_empty();
    /// // ... load image data
    /// let src = gray.region().intersection((20, 10, 25, 25));
    /// let dst = rgb.region().intersection((40, 40, 25, 25));
    /// rgb.set_region(dst, gray.region_iter(src));
    /// ```
    pub fn set_region<C, R, I, P, H>(&mut self, reg: R, mut it: I)
        where F: Format<Chan=C>, C: Channel + From<H>, H: Channel,
              P: Format<Chan=H>, R: Into<Region>, I: Iterator<Item=P> + PixModes
    {
        // FIXME: src/dst regions must have same shape!
        let reg = reg.into();
        let gamma_mode = self.gamma_mode;
        let alpha_mode = self.alpha_mode;
        let x0 = if reg.x >= 0 { reg.x as u32 } else { self.width() };
        let x1 = self.width().min(x0 + reg.width);
        let (x0, x1) = (x0 as usize, x1 as usize);
        let y0 = if reg.y >= 0 { reg.y as u32 } else { self.height() };
        let y1 = self.height().min(y0 + reg.height);
        if y0 < y1 && x0 < x1 {
            for yi in y0..y1 {
                let row = self.as_slice_row_mut(yi);
                for x in x0..x1 {
                    if let Some(p) = it.next() {
                        row[x] = Self::convert_pixel(p, &it, gamma_mode,
                            alpha_mode);
                    }
                }
            }
        }
    }
    /// Convert a pixel from one `Format` to another
    fn convert_pixel<C, P, H, M>(p: P, m: &M, gamma_mode: GammaMode,
        alpha_mode: AlphaMode) -> F
        where F: Format<Chan=C>, C: Channel + From<H>, H: Channel,
              P: Format<Chan=H>, M: PixModes
    {
        let rgba = p.rgba();
        // Decode gamma
        let rgba = match m.gamma_mode() {
            Some(m) => {
                [m.decode(rgba[0]),
                 m.decode(rgba[1]),
                 m.decode(rgba[2]),
                 rgba[3]]
            },
            None => rgba,
        };
        // Remove associated alpha
        let rgba = match m.alpha_mode() {
            Some(m) => {
                [m.decode(rgba[0], rgba[3]),
                 m.decode(rgba[1], rgba[3]),
                 m.decode(rgba[2], rgba[3]),
                 rgba[3]]
            },
            None => rgba,
        };
        // Convert bit depth
        let red = rgba[0].into();
        let green = rgba[1].into();
        let blue = rgba[2].into();
        let alpha = rgba[3].into();
        // Apply alpha
        let rgba = [
            alpha_mode.encode(red, alpha),
            alpha_mode.encode(green, alpha),
            alpha_mode.encode(blue, alpha),
            alpha
        ];
        // Encode gamma
        let rgba = [
            gamma_mode.encode(rgba[0]),
            gamma_mode.encode(rgba[1]),
            gamma_mode.encode(rgba[2]),
            rgba[3]
        ];
        F::with_rgba(rgba)
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
        Self::u8_slice(&self.pixels[s..t])
    }
    /// Get view of a pixel slice as a u8 slice.
    fn u8_slice(pix: &[F]) -> &[u8] {
        unsafe { pix.align_to::<u8>().1 }
    }
    /// Get view of pixels as a u8 slice.
    pub fn as_u8_slice(&self) -> &[u8] {
        Self::u8_slice(&self.pixels)
    }
    /// Get view of pixels as a mutable u8 slice.
    pub fn as_u8_slice_mut(&mut self) -> &mut [u8] {
        Self::u8_slice_mut(&mut self.pixels)
    }
    /// Get view of a pixel slice as a mutable u8 slice.
    fn u8_slice_mut(pix: &mut [F]) -> &mut [u8] {
        unsafe { pix.align_to_mut::<u8>().1 }
    }
    /// Make a copy with a specified pixel format.
    ///
    /// * `P` Pixel format of new Raster.
    ///
    /// ### Convert from Rgb8 to Rgba16
    /// ```
    /// # use pix::*;
    /// let mut r0 = Raster::<Rgb8>::new(50, 50).with_empty();
    /// // load pixels into raster
    /// let r1: Raster<Rgba16> = r0.to_raster();
    /// ```
    pub fn to_raster<C, H, P>(&self) -> Raster<P>
        where P: Format<Chan=C>, C: Channel + From<H>, H: Channel,
              F: Format<Chan=H>
    {
        let mut r = Raster::new(self.width(), self.height()).with_empty();
        let reg = self.region();
        r.set_region(reg, self.region_iter(reg));
        r
    }
}

impl<'a, F: Format> RasterIter<'a, F> {
    /// Create a new Raster pixel iterator
    ///
    /// * `region` Region of pixels to iterate.
    fn new(raster: &'a Raster<F>, region: Region) -> Self {
        let y = u32::try_from(region.y).unwrap_or(0);
        let bottom = u32::try_from(region.bottom()).unwrap_or(0);
        let x = u32::try_from(region.x).unwrap_or(0);
        let right = u32::try_from(region.right()).unwrap_or(0);
        let left = x;
        RasterIter { raster, left, right, bottom, x, y }
    }
}

impl<'a, F: Format> PixModes for RasterIter<'a, F> {

    /// Get the pixel format alpha mode
    fn alpha_mode(&self) -> Option<AlphaMode> {
        Some(self.raster.alpha_mode)
    }

    /// Get the pixel format gamma mode
    fn gamma_mode(&self) -> Option<GammaMode> {
        Some(self.raster.gamma_mode)
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
    /// Create a new Region
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Region { x, y, width, height }
    }
    /// Get intersection with another Region
    pub fn intersection<R>(self, rhs: R) -> Self
        where R: Into<Self>
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
    use super::*;
    use super::super::*;
    #[test]
    fn mask8() {
        let mut r = Raster::<Mask8>::new(3, 3).with_empty();
        r.set_pixel(0, 0, 0xFF);
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
        let mut r = Raster::<Mask16>::new(3, 3).with_empty();
        r.set_pixel(2, 0, 0x9ABC);
        r.set_pixel(1, 1, 0x5678);
        r.set_pixel(0, 2, 0x1234);
        r.set_pixel(0, 0, 0xFFFF);
        r.set_pixel(2, 2, 0x8080);
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
        let mut r = Raster::<Mask32>::new(4, 4).with_pixels(p);
        let clr = Mask32::new(Ch32::new(0.05));
        r.set_region((1, 1, 2, 2), clr);
        let v: Vec<_> = vec![
            0.25, 0.5, 0.75, 1.0,
            0.5,  0.05, 0.05, 0.8,
            0.75, 0.05, 0.05, 0.4,
            1.0,  0.75, 0.5, 0.25,
        ].iter().map(|p| Mask::new(Ch32::new(*p))).collect();
        let r2 = Raster::<Mask32>::new(4, 4).with_pixels(v);
        assert_eq!(r.as_slice(), r2.as_slice());
    }
    #[test]
    fn rgb8() {
        let mut r = Raster::<Rgb8>::new(4, 4).with_empty();
        let rgb = Rgb8::new(0xCC, 0xAA, 0xBB);
        r.set_region((1, 1, 2, 2), rgb);
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
        let mut r = Raster::<Gray8>::new(4, 4).with_empty();
        r.set_region((0, 0, 1, 1), Gray8::from(0x23));
        r.set_region((10, 10, 1, 1), Gray8::from(0x45));
        r.set_region((2, 2, 10, 10), Gray8::from(0xBB));
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
        let mut r = Raster::<Rgb8>::new(3, 3).with_u8_buffer(b);
        let rgb = Rgb8::new(0x12, 0x34, 0x56);
        r.set_region((0, 1, 2, 1), rgb);
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
            0x1001,0x5005, 0x1000,0x3002, 0x5004,0x7006,
            0x2002,0x6006, 0x9008,0xB00A, 0xD00C,0xF00E,
            0x3003,0x7007, 0xE00F,0xC00D, 0xA00B,0x8009,
        ];
        let mut r = Raster::<GrayAlpha16>::new(3, 3).with_u16_buffer(b);
        r.set_region((1, 0, 2, 2), GrayAlpha16::new(0x4444));
        let v = vec![
            0x01,0x10,0x05,0x50, 0x44,0x44,0xFF,0xFF, 0x44,0x44,0xFF,0xFF,
            0x02,0x20,0x06,0x60, 0x44,0x44,0xFF,0xFF, 0x44,0x44,0xFF,0xFF,
            0x03,0x30,0x07,0x70, 0x0F,0xE0,0x0D,0xC0, 0x0B,0xA0,0x09,0x80,
        ];
        // FIXME: this will fail on big-endian archs
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn gray_to_rgb() {
        let mut r = Raster::<Gray8>::new(3, 3).with_empty();
        r.set_region((2, 0, 4, 2), Gray8::new(0x45));
        r.set_region((0, 2, 2, 10), Gray8::new(0xDA));
        let r: Raster<Rgb8> = r.to_raster();
        let v = vec![
            0x00,0x00,0x00, 0x00,0x00,0x00, 0x45,0x45,0x45,
            0x00,0x00,0x00, 0x00,0x00,0x00, 0x45,0x45,0x45,
            0xDA,0xDA,0xDA, 0xDA,0xDA,0xDA, 0x00,0x00,0x00,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn rgb_to_gray() {
        let mut r = Raster::<Rgb16>::new(3, 3).with_empty();
        r.set_region((1, 0, 4, 2), Rgb16::new(0x4321, 0x9085, 0x5543));
        r.set_region((0, 1, 1, 10), Rgb16::new(0x5768, 0x4091, 0x5000));
        let r: Raster<Gray8> = r.to_raster();
        let v = vec![
            0x00, 0x90, 0x90,
            0x57, 0x90, 0x90,
            0x57, 0x00, 0x00,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn gray_to_mask() {
        let mut r = Raster::<GrayAlpha8>::new(3, 3).with_empty();
        r.set_region((0, 1, 2, 8), GrayAlpha8::with_alpha(0x67, 0x94));
        r.set_region((2, 0, 1, 10), GrayAlpha8::with_alpha(0xBA, 0xA2));
        let r: Raster<Mask16> = r.to_raster();
        let v = vec![
            0x00, 0x00, 0x00, 0x00, 0xA2, 0xA2,
            0x94, 0x94, 0x94, 0x94, 0xA2, 0xA2,
            0x94, 0x94, 0x94, 0x94, 0xA2, 0xA2,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn mask_to_gray() {
        let mut r = Raster::<Mask16>::new(3, 3).with_empty();
        r.set_region((0, 1, 3, 8), Mask16::new(0xABCD));
        r.set_region((2, 0, 1, 3), Mask16::new(0x9876));
        let r: Raster<GrayAlpha8> = r.to_raster();
        let v = vec![
            0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x98,
            0xFF, 0xAB, 0xFF, 0xAB, 0xFF, 0x98,
            0xFF, 0xAB, 0xFF, 0xAB, 0xFF, 0x98,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn copy_region_gray() {
        let mut r = Raster::<Gray16>::new(3, 3)
            .gamma_mode(GammaMode::Srgb)
            .with_empty();
        r.set_region((0, 2, 2, 5), Gray16::new(0x4455));
        r.set_region((2, 0, 3, 2), Gray8::new(0x33));
        let v = vec![
            0x00,0x00, 0x00,0x00, 0x0A,0x7C,
            0x00,0x00, 0x00,0x00, 0x0A,0x7C,
            0xB1,0x8D, 0xB1,0x8D, 0x00,0x00,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn from_rgb8() {
        let r = Raster::<Rgb8>::new(50, 50).with_empty();
        let _: Raster<Rgb16> = r.to_raster();
        let _: Raster<Rgb32> = r.to_raster();
        let _: Raster<Rgba8> = r.to_raster();
        let _: Raster<Rgba16> = r.to_raster();
        let _: Raster<Rgba32> = r.to_raster();
        let _: Raster<Gray8> = r.to_raster();
        let _: Raster<Gray16> = r.to_raster();
        let _: Raster<Gray32> = r.to_raster();
        let _: Raster<GrayAlpha8> = r.to_raster();
        let _: Raster<GrayAlpha16> = r.to_raster();
        let _: Raster<GrayAlpha32> = r.to_raster();
        let _: Raster<Mask8> = r.to_raster();
        let _: Raster<Mask16> = r.to_raster();
        let _: Raster<Mask32> = r.to_raster();
    }
    #[test]
    fn from_mask8() {
        let r = Raster::<Mask8>::new(50, 50).with_empty();
        let _: Raster<Rgb8> = r.to_raster();
        let _: Raster<Rgb16> = r.to_raster();
        let _: Raster<Rgb32> = r.to_raster();
        let _: Raster<Rgba8> = r.to_raster();
        let _: Raster<Rgba16> = r.to_raster();
        let _: Raster<Rgba32> = r.to_raster();
        let _: Raster<Gray8> = r.to_raster();
        let _: Raster<Gray16> = r.to_raster();
        let _: Raster<Gray32> = r.to_raster();
        let _: Raster<GrayAlpha8> = r.to_raster();
        let _: Raster<GrayAlpha16> = r.to_raster();
        let _: Raster<GrayAlpha32> = r.to_raster();
        let _: Raster<Mask8> = r.to_raster();
        let _: Raster<Mask16> = r.to_raster();
        let _: Raster<Mask32> = r.to_raster();
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
        assert_eq!(Region::new(0, 0, 4, 4), r.intersection(
            Region::new(-1, -1, 5, 5)));
        assert_eq!(Region::new(1, 2, 1, 3), r.intersection(
            Region::new(1, 2, 1, 100)));
        assert_eq!(Region::new(2, 1, 3, 1), r.intersection(
            Region::new(2, 1, 100, 1)));
        Ok(())
    }
}
