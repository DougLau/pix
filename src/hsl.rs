// hsl.rs       HSL color model
//
// Copyright (c) 2019-2020  Jeron Aldaron Lau
// Copyright (c) 2020  Douglas P Lau
//
use crate::alpha::{Premultiplied, Straight};
use crate::channel::{Ch16, Ch32, Ch8, Channel};
use crate::el::{Pix3, Pix4, Pixel};
use crate::gamma::Linear;
use crate::hue::{rgb_to_hue_chroma_value, Hexcone};
use crate::model::{Channels, ColorModel};
use std::any::TypeId;

/// [HSL] bi-hexcone [color model].
///
/// The components are *[hue]*, *[saturation]*, *[lightness]* and optional
/// *alpha*.
///
/// [color model]: trait.ColorModel.html
/// [hue]: struct.Hsl.html#method.hue
/// [hsl]: https://en.wikipedia.org/wiki/HSL_and_HSV
/// [lightness]: struct.Hsl.html#method.lightness
/// [saturation]: struct.Hsl.html#method.saturation
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Hsl {}

impl Hsl {
    /// Get the *hue* component.
    ///
    /// *Hue* is divided into 6 equal intervals arranged into a circle of
    /// degrees:
    ///
    /// * 0: Red
    /// * 60: Yellow
    /// * 120: Green
    /// * 180: Cyan
    /// * 240: Blue
    /// * 300: Magenta
    ///
    /// The degrees are mapped from [Channel::MIN] (0) to [Channel::MAX] (360)
    ///
    /// # Example: HSL Hue
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch32;
    /// let p = Hsl32::new(0.25, 0.5, 1.0);
    /// assert_eq!(Hsl::hue(p), Ch32::new(0.25));
    /// ```
    /// [Channel::MIN]: channel/trait.Channel.html#associatedconstant.MIN
    /// [Channel::MAX]: channel/trait.Channel.html#associatedconstant.MAX
    pub fn hue<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }

    /// Get the *saturation* component.
    ///
    /// Lower values are more gray (desaturated), while higher values are more
    /// colorful.  NOTE: HSL saturation is slightly different from [HSV]
    /// saturation.
    ///
    /// # Example: HSL Saturation
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch16;
    /// let p = Hsl16::new(0x2000, 0x1234, 0x8000);
    /// assert_eq!(Hsl::saturation(p), Ch16::new(0x1234));
    /// ```
    /// [hsv]: struct.Hsv.html
    pub fn saturation<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two()
    }

    /// Get the *lightness* component.
    ///
    /// Lower values are closer to *black*, while higher values are closer to
    /// *white*.
    ///
    /// # Example: HSL Lightness
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch8;
    /// let p = Hsl8::new(0x93, 0x80, 0xA0);
    /// assert_eq!(Hsl::lightness(p), Ch8::new(0xA0));
    /// ```
    pub fn lightness<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }
}

impl ColorModel for Hsl {
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
        if TypeId::of::<S::Model>() == TypeId::of::<D::Model>() {
            Channels::new(
                [
                    Self::saturation(src),
                    Self::lightness(src),
                    Self::alpha(src),
                    Self::hue(src),
                ],
                2,
            )
        } else {
            Channels::new(Self::into_rgba(src), 3)
        }
    }

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> [P::Chan; 4]
    where
        P: Pixel<Model = Self>,
    {
        let vl = 1.0 - (2.0 * Self::lightness(p).into() - 1.0).abs();
        let chroma = P::Chan::from(vl) * Self::saturation(p);
        let hp = Self::hue(p).into() * 6.0; // 0.0..=6.0
        let hc = Hexcone::from_hue_prime(hp);
        let (red, green, blue) = hc.rgb(chroma);
        let m = Self::lightness(p) - chroma * P::Chan::from(0.5);
        [red + m, green + m, blue + m, Self::alpha(p)]
    }

    /// Convert from channels shared by pixel types
    fn from_channels<S: Pixel, D: Pixel>(channels: Channels<D::Chan>) -> D {
        if TypeId::of::<S::Model>() == TypeId::of::<D::Model>() {
            debug_assert_eq!(channels.alpha_idx(), 2);
            let ch = channels.into_array();
            D::from_channels::<D::Chan>([ch[3], ch[0], ch[1], ch[2]])
        } else {
            debug_assert_eq!(channels.alpha_idx(), 3);
            Self::from_rgba::<D>(channels.into_array())
        }
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P: Pixel>(rgba: [P::Chan; 4]) -> P {
        let red = rgba[0];
        let green = rgba[1];
        let blue = rgba[2];
        let alpha = rgba[3];
        let (hue, chroma, val) = rgb_to_hue_chroma_value(red, green, blue);
        let lightness = val - chroma * P::Chan::from(0.5);
        let min_l = lightness.min(P::Chan::MAX - lightness);
        let sat_l = (val - lightness) / min_l;
        P::from_channels::<P::Chan>([hue, sat_l, lightness, alpha])
    }
}

/// [Hsl](struct.Hsl.html) 8-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsl8 = Pix3<Ch8, Hsl, Straight, Linear>;
/// [Hsl](struct.Hsl.html) 16-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsl16 = Pix3<Ch16, Hsl, Straight, Linear>;
/// [Hsl](struct.Hsl.html) 32-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsl32 = Pix3<Ch32, Hsl, Straight, Linear>;

/// [Hsl](struct.Hsl.html) 8-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsla8 = Pix4<Ch8, Hsl, Straight, Linear>;
/// [Hsl](struct.Hsl.html) 16-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsla16 = Pix4<Ch16, Hsl, Straight, Linear>;
/// [Hsl](struct.Hsl.html) 32-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsla32 = Pix4<Ch32, Hsl, Straight, Linear>;

/// [Hsl](struct.Hsl.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsla8p = Pix4<Ch8, Hsl, Premultiplied, Linear>;
/// [Hsl](struct.Hsl.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsla16p = Pix4<Ch16, Hsl, Premultiplied, Linear>;
/// [Hsl](struct.Hsl.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsla32p = Pix4<Ch32, Hsl, Premultiplied, Linear>;

#[cfg(test)]
mod test {
    use super::super::*;
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<Hsl8>(), 3);
        assert_eq!(std::mem::size_of::<Hsl16>(), 6);
        assert_eq!(std::mem::size_of::<Hsl32>(), 12);
        assert_eq!(std::mem::size_of::<Hsla8>(), 4);
        assert_eq!(std::mem::size_of::<Hsla16>(), 8);
        assert_eq!(std::mem::size_of::<Hsla32>(), 16);
    }

    #[test]
    fn hsl_to_rgb() {
        assert_eq!(Rgb8::new(255, 1, 1), Hsl8::new(0, 255, 128).convert());
        assert_eq!(
            Rgb8::new(255, 255, 0),
            Hsl32::new(60.0 / 360.0, 1.0, 0.5).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 255, 0),
            Hsl16::new(21845, 65535, 32768).convert(),
        );
        assert_eq!(Rgb8::new(0, 255, 255), Hsl32::new(0.5, 1.0, 0.5).convert());
        assert_eq!(
            Rgb8::new(0, 0, 255),
            Hsl32::new(240.0 / 360.0, 1.0, 0.5).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 0, 255),
            Hsl32::new(300.0 / 360.0, 1.0, 0.5).convert(),
        );
    }

    #[test]
    fn rgb_to_hsl() {
        assert_eq!(Hsl8::new(0, 255, 127), Rgb8::new(255, 0, 0).convert());
        assert_eq!(
            Hsl32::new(60.0 / 360.0, 1.0, 0.5),
            Rgb8::new(255, 255, 0).convert(),
        );
        assert_eq!(
            Hsl16::new(21845, 65535, 32767),
            Rgb8::new(0, 255, 0).convert(),
        );
        assert_eq!(Hsl32::new(0.5, 1.0, 0.5), Rgb8::new(0, 255, 255).convert());
        assert_eq!(
            Hsl32::new(240.0 / 360.0, 1.0, 0.5),
            Rgb8::new(0, 0, 255).convert(),
        );
        assert_eq!(
            Hsl32::new(300.0 / 360.0, 1.0, 0.5),
            Rgb8::new(255, 0, 255).convert(),
        );
    }
}
