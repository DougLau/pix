// hwb.rs       HWB color model
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::alpha::{Premultiplied, Straight};
use crate::channel::{Ch16, Ch32, Ch8, Channel};
use crate::el::{Pix3, Pix4, Pixel};
use crate::gamma::Linear;
use crate::hue::{rgb_to_hue_chroma_value, Hexcone};
use crate::model::{Channels, ColorModel};
use std::any::TypeId;

/// [HWB] [color model].
///
/// The components are *[hue]*, *[whiteness]*, *[blackness]* and optional
/// *alpha*.
///
/// [blackness]: struct.Hwb.html#method.blackness
/// [color model]: trait.ColorModel.html
/// [hue]: struct.Hwb.html#method.hue
/// [hwb]: https://en.wikipedia.org/wiki/HWB_color_model
/// [whiteness]: struct.Hwb.html#method.whiteness
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Hwb {}

impl Hwb {
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
    /// # Example: HWB Hue
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch32;
    /// let p = Hwb32::new(0.25, 0.5, 1.0);
    /// assert_eq!(Hwb::hue(p), Ch32::new(0.25));
    /// ```
    /// [Channel::MIN]: channel/trait.Channel.html#associatedconstant.MIN
    /// [Channel::MAX]: channel/trait.Channel.html#associatedconstant.MAX
    pub fn hue<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }

    /// Get the *whiteness* component.
    ///
    /// This is the amount of *whiteness* mixed in with a "pure" hue.
    ///
    /// # Example: HWB Whiteness
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch16;
    /// let p = Hwb16::new(0x2000, 0x2345, 0x5432);
    /// assert_eq!(Hwb::whiteness(p), Ch16::new(0x2345));
    /// ```
    pub fn whiteness<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two()
    }

    /// Get the *blackness* component.
    ///
    /// This is the amount of *blackness* mixed in with a "pure" hue.
    ///
    /// # Example: HWB Blackness
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch8;
    /// let p = Hwb8::new(0x43, 0x22, 0x19);
    /// assert_eq!(Hwb::blackness(p), Ch8::new(0x19));
    /// ```
    pub fn blackness<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }

    /// Get the *alpha* component.
    ///
    /// # Example: HWB Alpha
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch8;
    /// let p = Hwba8::new(0x50, 0xA0, 0x80, 0xB0);
    /// assert_eq!(Hwb::alpha(p), Ch8::new(0xB0));
    /// ```
    pub fn alpha<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.four()
    }

    /// Get *whiteness* and *blackness* clamped to 1.0 at the same ratio
    fn whiteness_blackness<P: Pixel>(p: P) -> (P::Chan, P::Chan)
    where
        P: Pixel<Model = Self>,
    {
        let whiteness = Hwb::whiteness(p);
        let blackness = Hwb::blackness(p);
        if whiteness + blackness - blackness < whiteness {
            let (w, b) = (whiteness.into(), blackness.into());
            let ratio = 1.0 / (w + b);
            (P::Chan::from(w * ratio), P::Chan::from(b * ratio))
        } else {
            (whiteness, blackness)
        }
    }
}

impl ColorModel for Hwb {
    /// Convert into channels shared by pixel types
    fn into_channels<S, D>(src: S) -> Channels<S::Chan>
    where
        S: Pixel<Model = Self>,
        D: Pixel,
    {
        if TypeId::of::<S::Model>() == TypeId::of::<D::Model>() {
            Channels::new(
                [
                    Self::whiteness(src),
                    Self::blackness(src),
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
        let (whiteness, blackness) = Self::whiteness_blackness(p);
        let v = P::Chan::MAX - blackness;
        let chroma = v - whiteness;
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
        let whiteness = (P::Chan::MAX - sat_v) * val;
        let blackness = P::Chan::MAX - val;
        P::from_channels::<P::Chan>([hue, whiteness, blackness, alpha])
    }
}

/// [Hwb](struct.Hwb.html) 8-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hwb8 = Pix3<Ch8, Hwb, Straight, Linear>;
/// [Hwb](struct.Hwb.html) 16-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hwb16 = Pix3<Ch16, Hwb, Straight, Linear>;
/// [Hwb](struct.Hwb.html) 32-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hwb32 = Pix3<Ch32, Hwb, Straight, Linear>;

/// [Hwb](struct.Hwb.html) 8-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hwba8 = Pix4<Ch8, Hwb, Straight, Linear>;
/// [Hwb](struct.Hwb.html) 16-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hwba16 = Pix4<Ch16, Hwb, Straight, Linear>;
/// [Hwb](struct.Hwb.html) 32-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hwba32 = Pix4<Ch32, Hwb, Straight, Linear>;

/// [Hwb](struct.Hwb.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hwba8p = Pix4<Ch8, Hwb, Premultiplied, Linear>;
/// [Hwb](struct.Hwb.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hwba16p = Pix4<Ch16, Hwb, Premultiplied, Linear>;
/// [Hwb](struct.Hwb.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hwba32p = Pix4<Ch32, Hwb, Premultiplied, Linear>;

#[cfg(test)]
mod test {
    use super::super::*;
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<Hwb8>(), 3);
        assert_eq!(std::mem::size_of::<Hwb16>(), 6);
        assert_eq!(std::mem::size_of::<Hwb32>(), 12);
        assert_eq!(std::mem::size_of::<Hwba8>(), 4);
        assert_eq!(std::mem::size_of::<Hwba16>(), 8);
        assert_eq!(std::mem::size_of::<Hwba32>(), 16);
    }

    #[test]
    fn hwb_to_rgb() {
        assert_eq!(Rgb8::new(127, 127, 127), Hwb8::new(0, 128, 128).convert());
        assert_eq!(Rgb8::new(127, 127, 127), Hwb8::new(0, 255, 255).convert());
        assert_eq!(Rgb8::new(85, 85, 85), Hwb8::new(0, 128, 255).convert());
        assert_eq!(Rgb8::new(255, 0, 0), Hwb8::new(0, 0, 0).convert());
        assert_eq!(
            Rgb8::new(255, 255, 128),
            Hwb32::new(60.0 / 360.0, 0.5, 0.0).convert(),
        );
        assert_eq!(Rgb8::new(0, 127, 0), Hwb16::new(21845, 0, 32768).convert());
        assert_eq!(
            Rgb8::new(128, 255, 255),
            Hwb32::new(0.5, 0.5, 0.0).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 0, 128),
            Hwb32::new(240.0 / 360.0, 0.0, 0.5).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 128, 255),
            Hwb32::new(300.0 / 360.0, 0.5, 0.0).convert(),
        );
    }

    #[test]
    fn rgb_to_hwb() {
        assert_eq!(Hwb8::new(0, 0, 0), Rgb8::new(255, 0, 0).convert());
        assert_eq!(Hwb8::new(0, 64, 0), Rgb8::new(255, 64, 64).convert());
        assert_eq!(
            Hwb32::new(60.0 / 360.0, 0.0, 0.50196075),
            Rgb8::new(127, 127, 0).convert(),
        );
        assert_eq!(
            Hwb16::new(21845, 8224, 0),
            Rgb8::new(32, 255, 32).convert(),
        );
        assert_eq!(
            Hwb32::new(0.5, 0.0, 0.7490196),
            Rgb8::new(0, 64, 64).convert(),
        );
        assert_eq!(
            Hwb32::new(240.0 / 360.0, 0.7529412, 0.0),
            Rgb8::new(192, 192, 255).convert(),
        );
        assert_eq!(
            Hwb32::new(300.0 / 360.0, 0.0, 0.0),
            Rgb8::new(255, 0, 255).convert(),
        );
    }
}
