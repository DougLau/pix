// ycc.rs       YCbCr color model.
//
// Copyright (c) 2019-2020  Jeron Aldaron Lau
// Copyright (c) 2020  Douglas P Lau
//
use crate::alpha::{Premultiplied, Straight};
use crate::channel::{Ch16, Ch32, Ch8};
use crate::el::{Pix3, Pix4, PixRgba, Pixel};
use crate::gamma::Linear;
use crate::model::ColorModel;
use std::ops::Range;

/// [YCbCr] [color model] used in JPEG format.
///
/// The components are *[y]*, *[cb]*, *[cr]* and optional *alpha*.
///
/// [cb]: model/struct.YCbCr.html#method.cb
/// [cr]: model/struct.YCbCr.html#method.cr
/// [channel]: trait.Channel.html
/// [color model]: trait.ColorModel.html
/// [y]: model/struct.YCbCr.html#method.y
/// [ycbcr]: https://en.wikipedia.org/wiki/YCbCr
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct YCbCr {}

impl YCbCr {
    /// Get the *y* component.
    ///
    /// This is *luma* when gamma-encoded, or *luminance* with linear gamma.
    ///
    /// # Example: YCbCr Y
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch32;
    /// # use pix::model::YCbCr;
    /// let p = YCbCr32::new(0.25, 0.5, 1.0);
    /// assert_eq!(YCbCr::y(p), Ch32::new(0.25));
    /// ```
    pub fn y<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }

    /// Get the *Cb* component.
    ///
    /// This the blue-difference chroma.
    ///
    /// # Example: YCbCr Cb
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch16;
    /// # use pix::model::YCbCr;
    /// let p = YCbCr16::new(0x2000, 0x1234, 0x8000);
    /// assert_eq!(YCbCr::cb(p), Ch16::new(0x1234));
    /// ```
    pub fn cb<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two()
    }

    /// Get the *Cr* component.
    ///
    /// This the red-difference chroma.
    ///
    /// # Example: YCbCr Cr
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch8;
    /// # use pix::model::YCbCr;
    /// let p = YCbCr8::new(0x93, 0x80, 0xA0);
    /// assert_eq!(YCbCr::cr(p), Ch8::new(0xA0));
    /// ```
    pub fn cr<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }

    /// Get the *alpha* component.
    ///
    /// # Example: YCbCr Alpha
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch8;
    /// # use pix::model::YCbCr;
    /// let p = YCbCra8::new(0x50, 0xA0, 0x80, 0xB0);
    /// assert_eq!(YCbCr::alpha(p), Ch8::new(0xB0));
    /// ```
    pub fn alpha<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.four()
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
        let y = Self::y(p).into();
        let cb = Self::cb(p).into();
        let cr = Self::cr(p).into();

        let r = y + (cr - 0.5) * 1.402;
        let g = y - (cb - 0.5) * 0.344136 - (cr - 0.5) * 0.714136;
        let b = y + (cb - 0.5) * 1.772;
        PixRgba::<P>::new(r.into(), g.into(), b.into(), Self::alpha(p).into())
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P>(rgba: PixRgba<P>) -> P
    where
        P: Pixel<Model = Self>,
    {
        let channels = rgba.channels();
        let red = channels[0].into();
        let green = channels[1].into();
        let blue = channels[2].into();
        let alpha = channels[3];

        let y = (0.299 * red) + (0.587 * green) + (0.114 * blue);
        let cb = 0.5 - (0.168736 * red) - (0.331264 * green) + (0.5 * blue);
        let cr = 0.5 + (0.5 * red) - (0.418688 * green) - (0.081312 * blue);

        P::from_channels(&[y.into(), cb.into(), cr.into(), alpha])
    }
}

/// [YCbCr](model/struct.YCbCr.html) 8-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCr8 = Pix3<Ch8, YCbCr, Straight, Linear>;
/// [YCbCr](model/struct.YCbCr.html) 16-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCr16 = Pix3<Ch16, YCbCr, Straight, Linear>;
/// [YCbCr](model/struct.YCbCr.html) 32-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCr32 = Pix3<Ch32, YCbCr, Straight, Linear>;

/// [YCbCr](model/struct.YCbCr.html) 8-bit
/// [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCra8 = Pix4<Ch8, YCbCr, Straight, Linear>;
/// [YCbCr](model/struct.YCbCr.html) 16-bit
/// [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCra16 = Pix4<Ch16, YCbCr, Straight, Linear>;
/// [YCbCr](model/struct.YCbCr.html) 32-bit
/// [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCra32 = Pix4<Ch32, YCbCr, Straight, Linear>;

/// [YCbCr](model/struct.YCbCr.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCra8p = Pix4<Ch8, YCbCr, Premultiplied, Linear>;
/// [YCbCr](model/struct.YCbCr.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCra16p = Pix4<Ch16, YCbCr, Premultiplied, Linear>;
/// [YCbCr](model/struct.YCbCr.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCra32p = Pix4<Ch32, YCbCr, Premultiplied, Linear>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<YCbCr8>(), 3);
        assert_eq!(std::mem::size_of::<YCbCr16>(), 6);
        assert_eq!(std::mem::size_of::<YCbCr32>(), 12);
        assert_eq!(std::mem::size_of::<YCbCra8>(), 4);
        assert_eq!(std::mem::size_of::<YCbCra16>(), 8);
        assert_eq!(std::mem::size_of::<YCbCra32>(), 16);
    }
}
