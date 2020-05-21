// cmy.rs       CMY color model.
//
// Copyright (c) 2020  Douglas P Lau
//
//! [CMY] color model and types.
//!
//! [cmy]: https://en.wikipedia.org/wiki/CMY_color_model
use crate::chan::{
    Ch16, Ch32, Ch8, Channel, Linear, Premultiplied, Srgb, Straight,
};
use crate::el::{Pix3, Pix4, PixRgba, Pixel};
use crate::ColorModel;
use std::ops::Range;

/// [CMY] subtractive [color model].
///
/// The components are *[cyan]*, *[magenta]*, *[yellow]* and optional *[alpha]*.
///
/// [alpha]: ../el/trait.Pixel.html#method.alpha
/// [cmy]: https://en.wikipedia.org/wiki/CMY_color_model
/// [color model]: ../trait.ColorModel.html
/// [cyan]: #method.cyan
/// [magenta]: #method.magenta
/// [yellow]: #method.yellow
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cmy {}

impl Cmy {
    /// Get the *cyan* component.
    ///
    /// # Example: Get CMY Cyan
    /// ```
    /// use pix::cmy::{Cmy, Cmy8};
    /// use pix::chan::Ch8;
    ///
    /// let p = Cmy8::new(0x93, 0x80, 0xA0);
    /// assert_eq!(Cmy::cyan(p), Ch8::new(0x93));
    /// ```
    pub fn cyan<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }

    /// Get a mutable reference to the *cyan* component.
    ///
    /// # Example: Modify CMY Cyan
    /// ```
    /// use pix::cmy::{Cmy, Cmy8};
    /// use pix::chan::Ch8;
    ///
    /// let mut p = Cmy8::new(0x88, 0x77, 0x66);
    /// *Cmy::cyan_mut(&mut p) = 0x55.into();
    /// assert_eq!(Cmy::cyan(p), Ch8::new(0x55));
    /// ```
    pub fn cyan_mut<P: Pixel>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one_mut()
    }

    /// Get the *magenta* component.
    ///
    /// # Example: CMY Magenta
    /// ```
    /// use pix::cmy::{Cmy, Cmy16};
    /// use pix::chan::Ch16;
    ///
    /// let p = Cmy16::new(0x2000, 0x1234, 0x8000);
    /// assert_eq!(Cmy::magenta(p), Ch16::new(0x1234));
    /// ```
    pub fn magenta<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two()
    }

    /// Get a mutable reference to the *magenta* component.
    ///
    /// # Example: Modify CMY Magenta
    /// ```
    /// use pix::cmy::{Cmy, Cmy16};
    /// use pix::chan::Ch16;
    ///
    /// let mut p = Cmy16::new(0x2000, 0x1234, 0x8000);
    /// *Cmy::magenta_mut(&mut p) = 0x4321.into();
    /// assert_eq!(Cmy::magenta(p), Ch16::new(0x4321));
    /// ```
    pub fn magenta_mut<P: Pixel>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two_mut()
    }

    /// Get the *yellow* component.
    ///
    /// # Example: CMY Yellow
    /// ```
    /// use pix::cmy::{Cmy, Cmy32};
    /// use pix::chan::Ch32;
    ///
    /// let p = Cmy32::new(0.25, 0.5, 1.0);
    /// assert_eq!(Cmy::yellow(p), Ch32::new(1.0));
    /// ```
    pub fn yellow<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }

    /// Get a mutable reference to the *yellow* component.
    ///
    /// # Example: Modify CMY Yellow
    /// ```
    /// use pix::cmy::{Cmy, Cmy32};
    /// use pix::chan::Ch32;
    ///
    /// let mut p = Cmy32::new(0.25, 0.5, 1.0);
    /// *Cmy::yellow_mut(&mut p) = Ch32::new(0.75);
    /// assert_eq!(Cmy::yellow(p), Ch32::new(0.75));
    /// ```
    pub fn yellow_mut<P: Pixel>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three_mut()
    }
}

impl ColorModel for Cmy {
    const CIRCULAR: Range<usize> = 0..0;
    const LINEAR: Range<usize> = 0..3;
    const ALPHA: usize = 3;

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> PixRgba<P>
    where
        P: Pixel<Model = Self>,
    {
        let red = P::Chan::MAX - Cmy::cyan(p);
        let green = P::Chan::MAX - Cmy::magenta(p);
        let blue = P::Chan::MAX - Cmy::yellow(p);
        PixRgba::<P>::new::<P::Chan>(red, green, blue, p.alpha())
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P>(rgba: PixRgba<P>) -> P
    where
        P: Pixel<Model = Self>,
    {
        let chan = rgba.channels();
        let cyan = P::Chan::MAX - chan[0];
        let magenta = P::Chan::MAX - chan[1];
        let yellow = P::Chan::MAX - chan[2];
        let alpha = chan[3];
        P::from_channels(&[cyan, magenta, yellow, alpha])
    }
}

/// [Cmy](struct.Cmy.html) 8-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Cmy8 = Pix3<Ch8, Cmy, Straight, Linear>;

/// [Cmy](struct.Cmy.html) 16-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Cmy16 = Pix3<Ch16, Cmy, Straight, Linear>;

/// [Cmy](struct.Cmy.html) 32-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Cmy32 = Pix3<Ch32, Cmy, Straight, Linear>;

/// [Cmy](struct.Cmy.html) 8-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html)
/// gamma [pixel](../el/trait.Pixel.html) format.
pub type Cmya8 = Pix4<Ch8, Cmy, Straight, Linear>;

/// [Cmy](struct.Cmy.html) 16-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Cmya16 = Pix4<Ch16, Cmy, Straight, Linear>;

/// [Cmy](struct.Cmy.html) 32-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Cmya32 = Pix4<Ch32, Cmy, Straight, Linear>;

/// [Cmy](struct.Cmy.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Cmya8p = Pix4<Ch8, Cmy, Premultiplied, Linear>;

/// [Cmy](struct.Cmy.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Cmya16p = Pix4<Ch16, Cmy, Premultiplied, Linear>;

/// [Cmy](struct.Cmy.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Cmya32p = Pix4<Ch32, Cmy, Premultiplied, Linear>;

/// [Cmy](struct.Cmy.html) 8-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SCmy8 = Pix3<Ch8, Cmy, Straight, Srgb>;

/// [Cmy](struct.Cmy.html) 16-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SCmy16 = Pix3<Ch16, Cmy, Straight, Srgb>;

/// [Cmy](struct.Cmy.html) 32-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SCmy32 = Pix3<Ch32, Cmy, Straight, Srgb>;

/// [Cmy](struct.Cmy.html) 8-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SCmya8 = Pix4<Ch8, Cmy, Straight, Srgb>;

/// [Cmy](struct.Cmy.html) 16-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SCmya16 = Pix4<Ch16, Cmy, Straight, Srgb>;

/// [Cmy](struct.Cmy.html) 32-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SCmya32 = Pix4<Ch32, Cmy, Straight, Srgb>;

/// [Cmy](struct.Cmy.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SCmya8p = Pix4<Ch8, Cmy, Premultiplied, Srgb>;

/// [Cmy](struct.Cmy.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SCmya16p = Pix4<Ch16, Cmy, Premultiplied, Srgb>;

/// [Cmy](struct.Cmy.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SCmya32p = Pix4<Ch32, Cmy, Premultiplied, Srgb>;

#[cfg(test)]
mod test {
    use crate::cmy::*;
    use crate::el::Pixel;
    use crate::rgb::*;

    #[test]
    fn cmy_to_rgb() {
        assert_eq!(Rgb8::new(255, 1, 2), Cmy8::new(0, 254, 253).convert());
        assert_eq!(Rgb8::new(255, 255, 0), Cmy32::new(0.0, 0.0, 1.0).convert(),);
        assert_eq!(Rgb8::new(0, 0, 255), Cmy16::new(65535, 65535, 0).convert(),);
    }

    #[test]
    fn rgb_to_cmy() {
        assert_eq!(Cmy8::new(0, 255, 127), Rgb8::new(255, 0, 128).convert());
        assert_eq!(Cmy32::new(1.0, 0.0, 0.0), Rgb8::new(0, 255, 255).convert(),);
        assert_eq!(Cmy16::new(0, 65535, 65535), Rgb8::new(255, 0, 0).convert(),);
    }
}
