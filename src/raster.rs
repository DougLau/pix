// raster.rs    A 2D raster image.
//
// Copyright (c) 2017-2019  Douglas P Lau
//
use std::marker::PhantomData;
use mask::Mask;
use pixel::PixFmt;

/// A raster image with owned pixel data.
/// If the pixel data must be owned elsewhere, consider using
/// [RasterB](struct.RasterB.html).
///
/// # Example
/// ```
/// use pix::{Raster, Rgba8};
/// let mut r: Raster<Rgba8> = Raster::new(100, 100);
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
    /// * `mask` Alpha mask for compositing.  It is cleared before returning.
    /// * `clr` Color to composite.
    /// * `pixels` Borrowed pixel data.
    pub fn over(&mut self, mask: &mut Mask, clr: F) {
        debug_assert_eq!(self.len(), self.pixels.len());
        F::over(&mut self.pixels, mask.pixels(), clr);
        mask.clear();
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
/// let mut r = RasterB::<Rgba8>::new(100, 100);
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
    /// * `mask` Alpha mask for compositing.  It is cleared before returning.
    /// * `clr` Color to composite.
    /// * `pixels` Borrowed pixel data.
    pub fn over(&self, mask: &mut Mask, clr: F, mut pixels: &mut [F]) {
        assert_eq!(self.len(), pixels.len());
        F::over(&mut pixels, mask.pixels(), clr);
        mask.clear();
    }
}
