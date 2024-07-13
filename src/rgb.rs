// rgb.rs       RGB color model.
//
// Copyright (c) 2018-2024  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
//! [RGB] color model and types.
//!
//! [rgb]: https://en.wikipedia.org/wiki/RGB_color_model
use crate::chan::{Ch16, Ch32, Ch8, Linear, Premultiplied, Srgb, Straight};
use crate::el::{Pix3, Pix4, PixRgba, Pixel};
use crate::ColorModel;
use std::ops::Range;

/// [RGB] additive [color model].
///
/// The components are *[red]*, *[green]*, *[blue]* and optional *[alpha]*.
///
/// [alpha]: ../el/trait.Pixel.html#method.alpha
/// [blue]: #method.blue
/// [color model]: ../trait.ColorModel.html
/// [green]: #method.green
/// [red]: #method.red
/// [rgb]: https://en.wikipedia.org/wiki/RGB_color_model
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rgb {}

impl Rgb {
    /// Get the *red* component.
    ///
    /// # Example: RGB Red
    /// ```
    /// use pix::chan::Ch32;
    /// use pix::rgb::{Rgb, Rgb32};
    ///
    /// let p = Rgb32::new(0.25, 0.5, 1.0);
    /// assert_eq!(Rgb::red(p), Ch32::new(0.25));
    /// ```
    pub fn red<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }

    /// Get a mutable reference to the *red* component.
    ///
    /// # Example: Modify RGB Red
    /// ```
    /// use pix::chan::Ch32;
    /// use pix::rgb::{Rgb, Rgb32};
    ///
    /// let mut p = Rgb32::new(0.25, 0.5, 1.0);
    /// *Rgb::red_mut(&mut p) = Ch32::new(0.75);
    /// assert_eq!(Rgb::red(p), Ch32::new(0.75));
    /// ```
    pub fn red_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one_mut()
    }

    /// Get the *green* component.
    ///
    /// # Example: RGB Green
    /// ```
    /// use pix::chan::Ch16;
    /// use pix::rgb::{Rgb, Rgb16};
    ///
    /// let p = Rgb16::new(0x2000, 0x1234, 0x8000);
    /// assert_eq!(Rgb::green(p), Ch16::new(0x1234));
    /// ```
    pub fn green<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two()
    }

    /// Get a mutable reference to the *green* component.
    ///
    /// # Example: Modify RGB Green
    /// ```
    /// use pix::chan::Ch16;
    /// use pix::rgb::{Rgb, Rgb16};
    ///
    /// let mut p = Rgb16::new(0x2000, 0x1234, 0x8000);
    /// *Rgb::green_mut(&mut p) = 0x4321.into();
    /// assert_eq!(Rgb::green(p), Ch16::new(0x4321));
    /// ```
    pub fn green_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two_mut()
    }

    /// Get the *blue* component.
    ///
    /// # Example: RGB Blue
    /// ```
    /// use pix::chan::Ch8;
    /// use pix::rgb::{Rgb, Rgb8};
    ///
    /// let p = Rgb8::new(0x93, 0x80, 0xA0);
    /// assert_eq!(Rgb::blue(p), Ch8::new(0xA0));
    /// ```
    pub fn blue<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }

    /// Get a mutable reference to the *blue* component.
    ///
    /// # Example: Modify RGB Blue
    /// ```
    /// use pix::chan::Ch8;
    /// use pix::rgb::{Rgb, Rgb8};
    ///
    /// let mut p = Rgb8::new(0x88, 0x77, 0x66);
    /// *Rgb::blue_mut(&mut p) = 0x55.into();
    /// assert_eq!(Rgb::blue(p), Ch8::new(0x55));
    /// ```
    pub fn blue_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three_mut()
    }

    /// Get channel-wise difference
    pub fn difference<P>(p: P, rhs: P) -> P
    where
        P: Pixel<Model = Self>,
    {
        let red = if Self::red(p) > Self::red(rhs) {
            Self::red(p) - Self::red(rhs)
        } else {
            Self::red(rhs) - Self::red(p)
        };
        let green = if Self::green(p) > Self::green(rhs) {
            Self::green(p) - Self::green(rhs)
        } else {
            Self::green(rhs) - Self::green(p)
        };
        let blue = if Self::blue(p) > Self::blue(rhs) {
            Self::blue(p) - Self::blue(rhs)
        } else {
            Self::blue(rhs) - Self::blue(p)
        };
        let alpha = if Pixel::alpha(p) > Pixel::alpha(rhs) {
            Pixel::alpha(p) - Pixel::alpha(rhs)
        } else {
            Pixel::alpha(rhs) - Pixel::alpha(p)
        };
        P::from_channels(&[red, green, blue, alpha])
    }

    /// Check if all `Channel`s are within threshold
    pub fn within_threshold<P>(p: P, rhs: P) -> bool
    where
        P: Pixel<Model = Self>,
    {
        Self::red(p) <= Self::red(rhs)
            && Self::green(p) <= Self::green(rhs)
            && Self::blue(p) <= Self::blue(rhs)
            && Pixel::alpha(p) <= Pixel::alpha(rhs)
    }
}

impl ColorModel for Rgb {
    const CIRCULAR: Range<usize> = 0..0;
    const LINEAR: Range<usize> = 0..3;
    const ALPHA: usize = 3;

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> PixRgba<P>
    where
        P: Pixel<Model = Self>,
    {
        let red = Rgb::red(p);
        let green = Rgb::green(p);
        let blue = Rgb::blue(p);
        PixRgba::<P>::new::<P::Chan>(red, green, blue, p.alpha())
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P>(rgba: PixRgba<P>) -> P
    where
        P: Pixel<Model = Self>,
    {
        P::from_channels(rgba.channels())
    }
}

/// [Rgb](struct.Rgb.html) 8-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Rgb8 = Pix3<Ch8, Rgb, Straight, Linear>;

/// [Rgb](struct.Rgb.html) 16-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Rgb16 = Pix3<Ch16, Rgb, Straight, Linear>;

/// [Rgb](struct.Rgb.html) 32-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Rgb32 = Pix3<Ch32, Rgb, Straight, Linear>;

/// [Rgb](struct.Rgb.html) 8-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Rgba8 = Pix4<Ch8, Rgb, Straight, Linear>;

/// [Rgb](struct.Rgb.html) 16-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Rgba16 = Pix4<Ch16, Rgb, Straight, Linear>;

/// [Rgb](struct.Rgb.html) 32-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Rgba32 = Pix4<Ch32, Rgb, Straight, Linear>;

/// [Rgb](struct.Rgb.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Rgba8p = Pix4<Ch8, Rgb, Premultiplied, Linear>;

/// [Rgb](struct.Rgb.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Rgba16p = Pix4<Ch16, Rgb, Premultiplied, Linear>;

/// [Rgb](struct.Rgb.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Rgba32p = Pix4<Ch32, Rgb, Premultiplied, Linear>;

/// [Rgb](struct.Rgb.html) 8-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SRgb8 = Pix3<Ch8, Rgb, Straight, Srgb>;

/// [Rgb](struct.Rgb.html) 16-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SRgb16 = Pix3<Ch16, Rgb, Straight, Srgb>;

/// [Rgb](struct.Rgb.html) 32-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SRgb32 = Pix3<Ch32, Rgb, Straight, Srgb>;

/// [Rgb](struct.Rgb.html) 8-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SRgba8 = Pix4<Ch8, Rgb, Straight, Srgb>;

/// [Rgb](struct.Rgb.html) 16-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SRgba16 = Pix4<Ch16, Rgb, Straight, Srgb>;

/// [Rgb](struct.Rgb.html) 32-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SRgba32 = Pix4<Ch32, Rgb, Straight, Srgb>;

/// [Rgb](struct.Rgb.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SRgba8p = Pix4<Ch8, Rgb, Premultiplied, Srgb>;

/// [Rgb](struct.Rgb.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SRgba16p = Pix4<Ch16, Rgb, Premultiplied, Srgb>;

/// [Rgb](struct.Rgb.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SRgba32p = Pix4<Ch32, Rgb, Premultiplied, Srgb>;

#[cfg(test)]
mod tests {
    use crate::el::Pixel;
    use crate::ops::SrcOver;
    use crate::rgb::*;

    #[test]
    fn rgba8_transparent() {
        let mut dst = Rgba8p::new(0, 0, 0, 0);
        let src = Rgba8p::new(20, 40, 80, 160);

        dst.composite_channels(&src, SrcOver);
        assert_eq!(dst, src);

        dst.composite_channels(&Rgba8p::new(0, 0, 0, 0), SrcOver);
        assert_eq!(dst, src);

        dst = Rgba8p::new(0xFF, 0xFF, 0xFF, 0x00);
        dst.composite_channels(&Rgba8p::new(0, 0, 0, 0), SrcOver);
        assert_eq!(dst, Rgba8p::new(0xFF, 0xFF, 0xFF, 0x00));
    }
}
