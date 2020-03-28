// rgb.rs       RGB color model.
//
// Copyright (c) 2018-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::alpha::{Premultiplied, Straight};
use crate::channel::{Ch16, Ch32, Ch8};
use crate::el::{Pix3, Pix4, Pixel};
use crate::gamma::{Linear, Srgb};
use crate::model::{Channels, ColorModel};

/// RGB additive [color model].
///
/// The components are *red*, *green* and *blue*, with optional *alpha*.
///
/// [color model]: trait.ColorModel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rgb {}

impl Rgb {
    /// Get the *red* component.
    pub fn red<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }

    /// Get the *green* component.
    pub fn green<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two()
    }

    /// Get the *blue* component.
    pub fn blue<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }

    /// Get channel-wise difference
    pub fn difference<P: Pixel>(p: P, rhs: P) -> P
    where
        P: Pixel<Model = Self>,
    {
        let r = if Self::red(p) > Self::red(rhs) {
            Self::red(p) - Self::red(rhs)
        } else {
            Self::red(rhs) - Self::red(p)
        };
        let g = if Self::green(p) > Self::green(rhs) {
            Self::green(p) - Self::green(rhs)
        } else {
            Self::green(rhs) - Self::green(p)
        };
        let b = if Self::blue(p) > Self::blue(rhs) {
            Self::blue(p) - Self::blue(rhs)
        } else {
            Self::blue(rhs) - Self::blue(p)
        };
        let a = if Self::alpha(p) > Self::alpha(rhs) {
            Self::alpha(p) - Self::alpha(rhs)
        } else {
            Self::alpha(rhs) - Self::alpha(p)
        };
        P::from_channels::<P::Chan>([r, g, b, a])
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
    /// Get the *alpha* component.
    fn alpha<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.four()
    }

    /// Convert into channels shared by pixel types
    fn into_channels<S, D>(src: S) -> Channels<S::Chan>
    where
        S: Pixel<Model = Self>,
        D: Pixel,
    {
        Channels::new(Self::into_rgba(src), 3)
    }

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> [P::Chan; 4]
    where
        P: Pixel<Model = Self>,
    {
        [Rgb::red(p), Rgb::green(p), Rgb::blue(p), Rgb::alpha(p)]
    }

    /// Convert from channels shared by pixel types
    fn from_channels<S: Pixel, D: Pixel>(channels: Channels<D::Chan>) -> D {
        debug_assert_eq!(channels.alpha_idx(), 3);
        Self::from_rgba::<D>(channels.into_array())
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P: Pixel>(rgba: [P::Chan; 4]) -> P {
        P::from_channels::<P::Chan>(rgba)
    }
}

/// [Rgb](struct.Rgb.html) 8-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgb8 = Pix3<Ch8, Rgb, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 16-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgb16 = Pix3<Ch16, Rgb, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 32-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgb32 = Pix3<Ch32, Rgb, Straight, Linear>;

/// [Rgb](struct.Rgb.html) 8-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgba8 = Pix4<Ch8, Rgb, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 16-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgba16 = Pix4<Ch16, Rgb, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 32-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgba32 = Pix4<Ch32, Rgb, Straight, Linear>;

/// [Rgb](struct.Rgb.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgba8p = Pix4<Ch8, Rgb, Premultiplied, Linear>;
/// [Rgb](struct.Rgb.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgba16p = Pix4<Ch16, Rgb, Premultiplied, Linear>;
/// [Rgb](struct.Rgb.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Rgba32p = Pix4<Ch32, Rgb, Premultiplied, Linear>;

/// [Rgb](struct.Rgb.html) 8-bit opaque (no *alpha* channel)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SRgb8 = Pix3<Ch8, Rgb, Straight, Srgb>;
/// [Rgb](struct.Rgb.html) 16-bit opaque (no *alpha* channel)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SRgb16 = Pix3<Ch16, Rgb, Straight, Srgb>;
/// [Rgb](struct.Rgb.html) 32-bit opaque (no *alpha* channel)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SRgb32 = Pix3<Ch32, Rgb, Straight, Srgb>;

/// [Rgb](struct.Rgb.html) 8-bit [straight](alpha/struct.Straight.html)
/// alpha [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type SRgba8 = Pix4<Ch8, Rgb, Straight, Srgb>;
/// [Rgb](struct.Rgb.html) 16-bit [straight](alpha/struct.Straight.html)
/// alpha [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type SRgba16 = Pix4<Ch16, Rgb, Straight, Srgb>;
/// [Rgb](struct.Rgb.html) 32-bit [straight](alpha/struct.Straight.html)
/// alpha [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type SRgba32 = Pix4<Ch32, Rgb, Straight, Srgb>;

/// [Rgb](struct.Rgb.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SRgba8p = Pix4<Ch8, Rgb, Premultiplied, Srgb>;
/// [Rgb](struct.Rgb.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SRgba16p = Pix4<Ch16, Rgb, Premultiplied, Srgb>;
/// [Rgb](struct.Rgb.html) 32-bit
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
