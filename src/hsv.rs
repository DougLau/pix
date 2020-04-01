// hsv.rs       HSV color model
//
// Copyright (c) 2019-2020  Jeron Aldaron Lau
// Copyright (c) 2020  Douglas P Lau
//
use crate::alpha::{Premultiplied, Straight};
use crate::channel::{Ch16, Ch32, Ch8};
use crate::el::{Pix3, Pix4, Pixel};
use crate::gamma::Linear;
use crate::hue::{rgb_to_hue_chroma_value, Hexcone};
use crate::model::{Channels, ColorModel};
use std::any::TypeId;

/// [HSV] hexcone [color model], also known as HSB.
///
/// The components are *[hue]*, *[saturation]*, *[value]* (or *brightness*) and
/// optional *alpha*.
///
/// [color model]: trait.ColorModel.html
/// [hue]: model/struct.Hsv.html#method.hue
/// [hsv]: https://en.wikipedia.org/wiki/HSL_and_HSV
/// [saturation]: model/struct.Hsv.html#method.saturation
/// [value]: model/struct.Hsv.html#method.value
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Hsv {}

impl Hsv {
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
    /// # Example: HSV Hue
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch32;
    /// let p = Hsv32::new(0.25, 0.5, 1.0);
    /// assert_eq!(Hsv::hue(p), Ch32::new(0.25));
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
    /// colorful.  NOTE: HSV saturation is slightly different from [HSL]
    /// saturation.
    ///
    /// # Example: HSV Saturation
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch16;
    /// let p = Hsv16::new(0x2000, 0x1234, 0x8000);
    /// assert_eq!(Hsv::saturation(p), Ch16::new(0x1234));
    /// ```
    /// [hsl]: struct.Hsl.html
    pub fn saturation<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two()
    }

    /// Get the *value* (or *brightness*) component.
    ///
    /// Lower values are closer to *black*, while higher values are closer to
    /// fully bright colors.
    ///
    /// # Example: HSV Lightness
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch8;
    /// let p = Hsv8::new(0x93, 0x80, 0xA0);
    /// assert_eq!(Hsv::value(p), Ch8::new(0xA0));
    /// ```
    pub fn value<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }

    /// Get the *alpha* component.
    ///
    /// # Example: HSV Alpha
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch8;
    /// let p = Hsva8::new(0x50, 0xA0, 0x80, 0xB0);
    /// assert_eq!(Hsv::alpha(p), Ch8::new(0xB0));
    /// ```
    pub fn alpha<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.four()
    }
}

impl ColorModel for Hsv {
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
                    Self::value(src),
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
        let v = Self::value(p);
        let chroma = v * Self::saturation(p);
        let hp = Self::hue(p).into() * 6.0; // 0.0..=6.0
        let hc = Hexcone::from_hue_prime(hp);
        let (red, green, blue) = hc.rgb(chroma);
        let m = v - chroma;
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
        let sat_v = chroma / val;
        P::from_channels::<P::Chan>([hue, sat_v, val, alpha])
    }
}

/// [Hsv](model/struct.Hsv.html) 8-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsv8 = Pix3<Ch8, Hsv, Straight, Linear>;
/// [Hsv](model/struct.Hsv.html) 16-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsv16 = Pix3<Ch16, Hsv, Straight, Linear>;
/// [Hsv](model/struct.Hsv.html) 32-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsv32 = Pix3<Ch32, Hsv, Straight, Linear>;

/// [Hsv](model/struct.Hsv.html) 8-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsva8 = Pix4<Ch8, Hsv, Straight, Linear>;
/// [Hsv](model/struct.Hsv.html) 16-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsva16 = Pix4<Ch16, Hsv, Straight, Linear>;
/// [Hsv](model/struct.Hsv.html) 32-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsva32 = Pix4<Ch32, Hsv, Straight, Linear>;

/// [Hsv](model/struct.Hsv.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsva8p = Pix4<Ch8, Hsv, Premultiplied, Linear>;
/// [Hsv](model/struct.Hsv.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsva16p = Pix4<Ch16, Hsv, Premultiplied, Linear>;
/// [Hsv](model/struct.Hsv.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsva32p = Pix4<Ch32, Hsv, Premultiplied, Linear>;

#[cfg(test)]
mod test {
    use super::super::*;
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<Hsv8>(), 3);
        assert_eq!(std::mem::size_of::<Hsv16>(), 6);
        assert_eq!(std::mem::size_of::<Hsv32>(), 12);
        assert_eq!(std::mem::size_of::<Hsva8>(), 4);
        assert_eq!(std::mem::size_of::<Hsva16>(), 8);
        assert_eq!(std::mem::size_of::<Hsva32>(), 16);
    }

    #[test]
    fn hsv_to_rgb() {
        assert_eq!(Rgb8::new(255, 0, 0), Hsv8::new(0, 255, 255).convert());
        assert_eq!(
            Rgb8::new(255, 255, 0),
            Hsv32::new(60.0 / 360.0, 1.0, 1.0).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 255, 0),
            Hsv16::new(21845, 65535, 65535).convert(),
        );
        assert_eq!(Rgb8::new(0, 255, 255), Hsv32::new(0.5, 1.0, 1.0).convert());
        assert_eq!(
            Rgb8::new(0, 0, 255),
            Hsv32::new(240.0 / 360.0, 1.0, 1.0).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 0, 255),
            Hsv32::new(300.0 / 360.0, 1.0, 1.0).convert(),
        );
    }

    #[test]
    fn hsv_to_rgb_unsat() {
        assert_eq!(Rgb8::new(255, 127, 127), Hsv8::new(0, 128, 255).convert());
        assert_eq!(
            Rgb8::new(255, 255, 128),
            Hsv32::new(60.0 / 360.0, 0.5, 1.0).convert(),
        );
        assert_eq!(
            Rgb8::new(127, 255, 127),
            Hsv16::new(21845, 32768, 65535).convert(),
        );
        assert_eq!(
            Rgb8::new(128, 255, 255),
            Hsv32::new(180.0 / 360.0, 0.5, 1.0).convert(),
        );
        assert_eq!(
            Rgb8::new(128, 128, 255),
            Hsv32::new(240.0 / 360.0, 0.5, 1.0).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 128, 255),
            Hsv32::new(300.0 / 360.0, 0.5, 1.0).convert(),
        );
    }

    #[test]
    fn hsv_to_rgb_dark() {
        assert_eq!(Rgb8::new(128, 0, 0), Hsv8::new(0, 255, 128).convert());
        assert_eq!(
            Rgb8::new(128, 128, 0),
            Hsv32::new(60.0 / 360.0, 1.0, 0.5).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 128, 0),
            Hsv16::new(21845, 65535, 32768).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 128, 128),
            Hsv32::new(180.0 / 360.0, 1.0, 0.5).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 0, 128),
            Hsv32::new(240.0 / 360.0, 1.0, 0.5).convert(),
        );
        assert_eq!(
            Rgb8::new(128, 0, 128),
            Hsv32::new(300.0 / 360.0, 1.0, 0.5).convert(),
        );
    }

    #[test]
    fn hsv_to_rgb_hue() {
        assert_eq!(Rgb8::new(255, 192, 0), Hsv8::new(32, 255, 255).convert());
        assert_eq!(Rgb8::new(126, 255, 0), Hsv8::new(64, 255, 255).convert());
        assert_eq!(Rgb8::new(0, 255, 66), Hsv8::new(96, 255, 255).convert());
        assert_eq!(Rgb8::new(0, 60, 255), Hsv8::new(160, 255, 255).convert());
        assert_eq!(Rgb8::new(132, 0, 255), Hsv8::new(192, 255, 255).convert());
        assert_eq!(Rgb8::new(255, 0, 186), Hsv8::new(224, 255, 255).convert());
    }

    #[test]
    fn hsv_to_rgb_grays() {
        assert_eq!(Rgb8::new(255, 255, 255), Hsv8::new(0, 0, 255).convert());
        assert_eq!(Rgb8::new(128, 128, 128), Hsv8::new(100, 0, 128).convert());
        assert_eq!(Hsv8::new(0, 0, 255), Rgb8::new(255, 255, 255).convert());
        assert_eq!(Hsv8::new(0, 0, 128), Rgb8::new(128, 128, 128).convert());
    }

    #[test]
    fn rgb_to_hsv() {
        assert_eq!(Hsv8::new(0, 255, 255), Rgb8::new(255, 0, 0).convert());
        assert_eq!(
            Hsv32::new(60.0 / 360.0, 1.0, 1.0),
            Rgb8::new(255, 255, 0).convert(),
        );
        assert_eq!(
            Hsv16::new(21845, 65535, 65535),
            Rgb8::new(0, 255, 0).convert(),
        );
        assert_eq!(Hsv32::new(0.5, 1.0, 1.0), Rgb8::new(0, 255, 255).convert());
        assert_eq!(
            Hsv32::new(240.0 / 360.0, 1.0, 1.0),
            Rgb8::new(0, 0, 255).convert(),
        );
        assert_eq!(
            Hsv32::new(300.0 / 360.0, 1.0, 1.0),
            Rgb8::new(255, 0, 255).convert(),
        );
    }

    #[test]
    fn rgb_to_hsv_unsat() {
        assert_eq!(Hsv8::new(0, 128, 255), Rgb8::new(255, 127, 127).convert());
        assert_eq!(Hsv8::new(42, 128, 255), Rgb8::new(255, 255, 127).convert());
        assert_eq!(Hsv8::new(85, 127, 255), Rgb8::new(128, 255, 128).convert());
        assert_eq!(
            Hsv8::new(128, 127, 255),
            Rgb8::new(128, 255, 255).convert()
        );
        assert_eq!(
            Hsv8::new(170, 127, 255),
            Rgb8::new(128, 128, 255).convert(),
        );
        assert_eq!(
            Hsv8::new(213, 127, 255),
            Rgb8::new(255, 128, 255).convert(),
        );
    }
}
