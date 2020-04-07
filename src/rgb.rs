// rgb.rs       RGB color model.
//
// Copyright (c) 2018-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::alpha::{Premultiplied, Straight};
use crate::channel::{Ch16, Ch32, Ch8};
use crate::el::{Pix3, Pix4, PixRgba, Pixel};
use crate::gamma::{Linear, Srgb};
use crate::model::ColorModel;
use std::ops::Range;

/// [RGB] additive [color model].
///
/// The components are *[red]*, *[green]*, *[blue]* and optional *alpha*.
///
/// [blue]: model/struct.Rgb.html#method.blue
/// [color model]: trait.ColorModel.html
/// [green]: model/struct.Rgb.html#method.green
/// [red]: model/struct.Rgb.html#method.red
/// [rgb]: https://en.wikipedia.org/wiki/RGB_color_model
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rgb {}

impl Rgb {
    /// Get the *red* component.
    ///
    /// # Example: RGB Red
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch32;
    /// # use pix::model::Rgb;
    /// let p = Rgb32::new(0.25, 0.5, 1.0);
    /// assert_eq!(Rgb::red(p), Ch32::new(0.25));
    /// ```
    pub fn red<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }

    /// Get the *green* component.
    ///
    /// # Example: RGB Green
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch16;
    /// # use pix::model::Rgb;
    /// let p = Rgb16::new(0x2000, 0x1234, 0x8000);
    /// assert_eq!(Rgb::green(p), Ch16::new(0x1234));
    /// ```
    pub fn green<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two()
    }

    /// Get the *blue* component.
    ///
    /// # Example: RGB Blue
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch8;
    /// # use pix::model::Rgb;
    /// let p = Rgb8::new(0x93, 0x80, 0xA0);
    /// assert_eq!(Rgb::blue(p), Ch8::new(0xA0));
    /// ```
    pub fn blue<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }

    /// Get the *alpha* component.
    ///
    /// # Example: RGB Alpha
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch8;
    /// # use pix::model::Rgb;
    /// let p = Rgba8::new(0x50, 0xA0, 0x80, 0xB0);
    /// assert_eq!(Rgb::alpha(p), Ch8::new(0xB0));
    /// ```
    pub fn alpha<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.four()
    }

    /// Get channel-wise difference
    pub fn difference<P: Pixel>(p: P, rhs: P) -> P
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
        let alpha = if Self::alpha(p) > Self::alpha(rhs) {
            Self::alpha(p) - Self::alpha(rhs)
        } else {
            Self::alpha(rhs) - Self::alpha(p)
        };
        P::from_channels(&[red, green, blue, alpha])
    }

    /// Check if all `Channel`s are within threshold
    pub fn within_threshold<P: Pixel>(p: P, rhs: P) -> bool
    where
        P: Pixel<Model = Self>,
    {
        Self::red(p) <= Self::red(rhs)
            && Self::green(p) <= Self::green(rhs)
            && Self::blue(p) <= Self::blue(rhs)
            && Self::alpha(p) <= Self::alpha(rhs)
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
        let red = Rgb::red(p).into();
        let green = Rgb::green(p).into();
        let blue = Rgb::blue(p).into();
        let alpha = Rgb::alpha(p).into();
        PixRgba::<P>::new(red, green, blue, alpha)
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P>(rgba: PixRgba<P>) -> P
    where
        P: Pixel<Model = Self>,
    {
        P::from_channels(rgba.channels())
    }
}

/// [Rgb](model/struct.Rgb.html) 8-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgb8 = Pix3<Ch8, Rgb, Straight, Linear>;
/// [Rgb](model/struct.Rgb.html) 16-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgb16 = Pix3<Ch16, Rgb, Straight, Linear>;
/// [Rgb](model/struct.Rgb.html) 32-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgb32 = Pix3<Ch32, Rgb, Straight, Linear>;

/// [Rgb](model/struct.Rgb.html) 8-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgba8 = Pix4<Ch8, Rgb, Straight, Linear>;
/// [Rgb](model/struct.Rgb.html) 16-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgba16 = Pix4<Ch16, Rgb, Straight, Linear>;
/// [Rgb](model/struct.Rgb.html) 32-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgba32 = Pix4<Ch32, Rgb, Straight, Linear>;

/// [Rgb](model/struct.Rgb.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgba8p = Pix4<Ch8, Rgb, Premultiplied, Linear>;
/// [Rgb](model/struct.Rgb.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgba16p = Pix4<Ch16, Rgb, Premultiplied, Linear>;
/// [Rgb](model/struct.Rgb.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgba32p = Pix4<Ch32, Rgb, Premultiplied, Linear>;

/// [Rgb](model/struct.Rgb.html) 8-bit opaque (no *alpha* channel)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SRgb8 = Pix3<Ch8, Rgb, Straight, Srgb>;
/// [Rgb](model/struct.Rgb.html) 16-bit opaque (no *alpha* channel)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SRgb16 = Pix3<Ch16, Rgb, Straight, Srgb>;
/// [Rgb](model/struct.Rgb.html) 32-bit opaque (no *alpha* channel)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SRgb32 = Pix3<Ch32, Rgb, Straight, Srgb>;

/// [Rgb](model/struct.Rgb.html) 8-bit [straight](alpha/struct.Straight.html)
/// alpha [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type SRgba8 = Pix4<Ch8, Rgb, Straight, Srgb>;
/// [Rgb](model/struct.Rgb.html) 16-bit [straight](alpha/struct.Straight.html)
/// alpha [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type SRgba16 = Pix4<Ch16, Rgb, Straight, Srgb>;
/// [Rgb](model/struct.Rgb.html) 32-bit [straight](alpha/struct.Straight.html)
/// alpha [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type SRgba32 = Pix4<Ch32, Rgb, Straight, Srgb>;

/// [Rgb](model/struct.Rgb.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SRgba8p = Pix4<Ch8, Rgb, Premultiplied, Srgb>;
/// [Rgb](model/struct.Rgb.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SRgba16p = Pix4<Ch16, Rgb, Premultiplied, Srgb>;
/// [Rgb](model/struct.Rgb.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SRgba32p = Pix4<Ch32, Rgb, Premultiplied, Srgb>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<SRgb8>(), 3);
        assert_eq!(std::mem::size_of::<SRgb16>(), 6);
        assert_eq!(std::mem::size_of::<SRgb32>(), 12);
        assert_eq!(std::mem::size_of::<SRgba8>(), 4);
        assert_eq!(std::mem::size_of::<SRgba16>(), 8);
        assert_eq!(std::mem::size_of::<SRgba32>(), 16);
    }
}
