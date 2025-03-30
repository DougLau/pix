// ycc.rs       YCbCr color model.
//
// Copyright (c) 2019-2020  Jeron Aldaron Lau
// Copyright (c) 2020-2024  Douglas P Lau
//
//! [YCbCr] color model and types.
//!
//! [ycbcr]: https://en.wikipedia.org/wiki/YCbCr
use crate::ColorModel;
use crate::chan::{Ch8, Ch16, Ch32, Channel, Linear, Premultiplied, Straight};
use crate::el::{Pix, PixRgba, Pixel};
use std::ops::Range;

/// [YCbCr] [color model] (used in JPEG and other formats).
///
/// The components are *[y]*, *[cb]*, *[cr]* and optional *[alpha]*.
///
/// [alpha]: ../el/trait.Pixel.html#method.alpha
/// [cb]: #method.cb
/// [cr]: #method.cr
/// [color model]: ../trait.ColorModel.html
/// [y]: #method.y
/// [ycbcr]: https://en.wikipedia.org/wiki/YCbCr
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct YCbCr {}

impl YCbCr {
    /// Get the *y* component.
    ///
    /// This is *luma* when gamma-encoded, or *luminance* with linear gamma.
    ///
    /// # Example: YCbCr Y
    /// ```
    /// use pix::chan::Ch32;
    /// use pix::ycc::{YCbCr, YCbCr32};
    ///
    /// let p = YCbCr32::new(0.25, 0.5, 1.0);
    /// assert_eq!(YCbCr::y(p), Ch32::new(0.25));
    /// ```
    pub fn y<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.get::<0>()
    }

    /// Get a mutable reference to the *y* component.
    ///
    /// # Example: Modify YCbCr Y
    /// ```
    /// use pix::chan::Ch32;
    /// use pix::ycc::{YCbCr, YCbCr32};
    ///
    /// let mut p = YCbCr32::new(0.25, 0.5, 1.0);
    /// *YCbCr::y_mut(&mut p) = Ch32::new(0.75);
    /// assert_eq!(YCbCr::y(p), Ch32::new(0.75));
    /// ```
    pub fn y_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.get_mut::<0>()
    }

    /// Get the *Cb* component.
    ///
    /// This the blue-difference chroma.
    ///
    /// # Example: YCbCr Cb
    /// ```
    /// use pix::chan::Ch16;
    /// use pix::ycc::{YCbCr, YCbCr16};
    ///
    /// let p = YCbCr16::new(0x2000, 0x1234, 0x8000);
    /// assert_eq!(YCbCr::cb(p), Ch16::new(0x1234));
    /// ```
    pub fn cb<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.get::<1>()
    }

    /// Get a mutable reference to the *Cb* component.
    ///
    /// # Example: Modify YCbCr Cr
    /// ```
    /// use pix::chan::Ch16;
    /// use pix::ycc::{YCbCr, YCbCr16};
    ///
    /// let mut p = YCbCr16::new(0x2000, 0x1234, 0x8000);
    /// *YCbCr::cb_mut(&mut p) = 0x4321.into();
    /// assert_eq!(YCbCr::cb(p), Ch16::new(0x4321));
    /// ```
    pub fn cb_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.get_mut::<1>()
    }

    /// Get the *Cr* component.
    ///
    /// This the red-difference chroma.
    ///
    /// # Example: YCbCr Cr
    /// ```
    /// use pix::chan::Ch8;
    /// use pix::ycc::{YCbCr, YCbCr8};
    ///
    /// let p = YCbCr8::new(0x93, 0x80, 0xA0);
    /// assert_eq!(YCbCr::cr(p), Ch8::new(0xA0));
    /// ```
    pub fn cr<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.get::<2>()
    }

    /// Get a mutable reference to the *Cr* component.
    ///
    /// # Example: Modify YCbCr Cr
    /// ```
    /// use pix::chan::Ch8;
    /// use pix::ycc::{YCbCr, YCbCr8};
    ///
    /// let mut p = YCbCr8::new(0x88, 0x77, 0x66);
    /// *YCbCr::cr_mut(&mut p) = 0x55.into();
    /// assert_eq!(YCbCr::cr(p), Ch8::new(0x55));
    /// ```
    pub fn cr_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.get_mut::<2>()
    }
}

impl ColorModel for YCbCr {
    const CIRCULAR: Range<usize> = 0..0;
    const LINEAR: Range<usize> = 0..3;
    const ALPHA: usize = 3;

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> PixRgba<P>
    where
        P: Pixel<Model = Self>,
    {
        let y = Self::y(p).to_f32();
        let cb = Self::cb(p).to_f32();
        let cr = Self::cr(p).to_f32();

        let red = y + (cr - 0.5) * 1.402;
        let green = y - (cb - 0.5) * 0.344_136 - (cr - 0.5) * 0.714_136;
        let blue = y + (cb - 0.5) * 1.772;
        PixRgba::<P>::new(red, green, blue, p.alpha().to_f32())
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P>(rgba: PixRgba<P>) -> P
    where
        P: Pixel<Model = Self>,
    {
        let chan = rgba.channels();
        let red = chan[0].to_f32();
        let green = chan[1].to_f32();
        let blue = chan[2].to_f32();
        let alpha = chan[3];

        let y = (0.299 * red) + (0.587 * green) + (0.114 * blue);
        let cb = 0.5 - (0.168_736 * red) - (0.331_264 * green) + (0.5 * blue);
        let cr = 0.5 + (0.5 * red) - (0.418_688 * green) - (0.081_312 * blue);

        P::from_channels(&[y.into(), cb.into(), cr.into(), alpha])
    }
}

/// [YCbCr](struct.YCbCr.html) 8-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type YCbCr8 = Pix<3, Ch8, YCbCr, Straight, Linear>;

/// [YCbCr](struct.YCbCr.html) 16-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type YCbCr16 = Pix<3, Ch16, YCbCr, Straight, Linear>;

/// [YCbCr](struct.YCbCr.html) 32-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type YCbCr32 = Pix<3, Ch32, YCbCr, Straight, Linear>;

/// [YCbCr](struct.YCbCr.html) 8-bit
/// [straight](../chan/struct.Straight.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type YCbCra8 = Pix<4, Ch8, YCbCr, Straight, Linear>;

/// [YCbCr](struct.YCbCr.html) 16-bit
/// [straight](../chan/struct.Straight.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type YCbCra16 = Pix<4, Ch16, YCbCr, Straight, Linear>;

/// [YCbCr](struct.YCbCr.html) 32-bit
/// [straight](../chan/struct.Straight.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type YCbCra32 = Pix<4, Ch32, YCbCr, Straight, Linear>;

/// [YCbCr](struct.YCbCr.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type YCbCra8p = Pix<4, Ch8, YCbCr, Premultiplied, Linear>;

/// [YCbCr](struct.YCbCr.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type YCbCra16p = Pix<4, Ch16, YCbCr, Premultiplied, Linear>;

/// [YCbCr](struct.YCbCr.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type YCbCra32p = Pix<4, Ch32, YCbCr, Premultiplied, Linear>;
