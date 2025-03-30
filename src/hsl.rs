// hsl.rs       HSL color model
//
// Copyright (c) 2019-2020  Jeron Aldaron Lau
// Copyright (c) 2020-2024  Douglas P Lau
//
//! [HSL] color model and types.
//!
//! [hsl]: https://en.wikipedia.org/wiki/HSL_and_HSV
use crate::ColorModel;
use crate::chan::{
    Ch8, Ch16, Ch32, Channel, Linear, Premultiplied, Srgb, Straight,
};
use crate::el::{Pix, PixRgba, Pixel};
use crate::hue::{Hexcone, rgb_to_hue_chroma_value};
use std::ops::Range;

/// [HSL] bi-hexcone [color model].
///
/// The components are *[hue]*, *[saturation]*, *[lightness]* and optional
/// *[alpha]*.
///
/// [alpha]: ../el/trait.Pixel.html#method.alpha
/// [color model]: ../trait.ColorModel.html
/// [hue]: #method.hue
/// [hsl]: https://en.wikipedia.org/wiki/HSL_and_HSV
/// [lightness]: #method.lightness
/// [saturation]: #method.saturation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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
    /// # Example: Get HSL Hue
    /// ```
    /// use pix::chan::Ch32;
    /// use pix::hsl::{Hsl, Hsl32};
    ///
    /// let p = Hsl32::new(0.25, 0.5, 1.0);
    /// assert_eq!(Hsl::hue(p), Ch32::new(0.25));
    /// ```
    /// [Channel::MIN]: ../chan/trait.Channel.html#associatedconstant.MIN
    /// [Channel::MAX]: ../chan/trait.Channel.html#associatedconstant.MAX
    pub fn hue<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }

    /// Get a mutable reference to the *hue* component.
    ///
    /// # Example: Modify HSL Hue
    /// ```
    /// use pix::chan::{Ch32, Channel};
    /// use pix::hsl::{Hsl, Hsl32};
    ///
    /// let mut p = Hsl32::new(0.2, 0.75, 0.5);
    /// let mut h = Hsl::hue_mut(&mut p);
    /// *h = h.wrapping_sub(Ch32::new(0.4));
    /// assert_eq!(Hsl::hue(p), Ch32::new(0.8));
    /// ```
    pub fn hue_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one_mut()
    }

    /// Get the *saturation* component.
    ///
    /// Lower values are more gray (desaturated), while higher values are more
    /// colorful.  NOTE: HSL saturation is slightly different from [HSV]
    /// saturation.
    ///
    /// # Example: HSL Saturation
    /// ```
    /// use pix::chan::Ch16;
    /// use pix::hsl::{Hsl, Hsl16};
    ///
    /// let p = Hsl16::new(0x2000, 0x1234, 0x8000);
    /// assert_eq!(Hsl::saturation(p), Ch16::new(0x1234));
    /// ```
    /// [hsv]: struct.Hsv.html
    pub fn saturation<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two()
    }

    /// Get a mutable reference to the *saturation* component.
    ///
    /// # Example: Modify HSL Saturation
    /// ```
    /// use pix::chan::Ch16;
    /// use pix::hsl::{Hsl, Hsl16};
    ///
    /// let mut p = Hsl16::new(0x2000, 0x1234, 0x8000);
    /// *Hsl::saturation_mut(&mut p) = Ch16::new(0x4321);
    /// assert_eq!(Hsl::saturation(p), Ch16::new(0x4321));
    /// ```
    pub fn saturation_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two_mut()
    }

    /// Get the *lightness* component.
    ///
    /// Lower values are closer to *black*, while higher values are closer to
    /// *white*.
    ///
    /// # Example: HSL Lightness
    /// ```
    /// use pix::chan::Ch8;
    /// use pix::hsl::{Hsl, Hsl8};
    ///
    /// let p = Hsl8::new(0x93, 0x80, 0xA0);
    /// assert_eq!(Hsl::lightness(p), Ch8::new(0xA0));
    /// ```
    pub fn lightness<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }

    /// Get a mutable reference to the *lightness* component.
    ///
    /// # Example: Modify HSL Lightness
    /// ```
    /// use pix::chan::Ch8;
    /// use pix::hsl::{Hsl, Hsl8};
    ///
    /// let mut p = Hsl8::new(0x93, 0x80, 0xA0);
    /// *Hsl::lightness_mut(&mut p) = Ch8::new(0xBB);
    /// assert_eq!(Hsl::lightness(p), Ch8::new(0xBB));
    /// ```
    pub fn lightness_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three_mut()
    }
}

impl ColorModel for Hsl {
    const CIRCULAR: Range<usize> = 0..1;
    const LINEAR: Range<usize> = 1..3;
    const ALPHA: usize = 3;

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> PixRgba<P>
    where
        P: Pixel<Model = Self>,
    {
        let vl = 1.0 - (2.0 * Self::lightness(p).to_f32() - 1.0).abs();
        let chroma = P::Chan::from(vl) * Self::saturation(p);
        let hp = Self::hue(p).to_f32() * 6.0; // 0.0..=6.0
        let hc = Hexcone::from_hue_prime(hp);
        let (red, green, blue) = hc.rgb(chroma);
        let m = Self::lightness(p) - chroma * P::Chan::from(0.5);
        PixRgba::<P>::new::<P::Chan>(red + m, green + m, blue + m, p.alpha())
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P>(rgba: PixRgba<P>) -> P
    where
        P: Pixel<Model = Self>,
    {
        let chan = rgba.channels();
        let red = chan[0];
        let green = chan[1];
        let blue = chan[2];
        let alpha = chan[3];
        let (hue, chroma, val) = rgb_to_hue_chroma_value(red, green, blue);
        let lightness = val - chroma * P::Chan::from(0.5);
        let min_l = lightness.min(P::Chan::MAX - lightness);
        let sat_l = (val - lightness) / min_l;
        P::from_channels(&[hue, sat_l, lightness, alpha])
    }
}

/// [Hsl](struct.Hsl.html) 8-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Hsl8 = Pix<3, Ch8, Hsl, Straight, Linear>;

/// [Hsl](struct.Hsl.html) 16-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Hsl16 = Pix<3, Ch16, Hsl, Straight, Linear>;

/// [Hsl](struct.Hsl.html) 32-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Hsl32 = Pix<3, Ch32, Hsl, Straight, Linear>;

/// [Hsl](struct.Hsl.html) 8-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Hsla8 = Pix<4, Ch8, Hsl, Straight, Linear>;

/// [Hsl](struct.Hsl.html) 16-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Hsla16 = Pix<4, Ch16, Hsl, Straight, Linear>;

/// [Hsl](struct.Hsl.html) 32-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Hsla32 = Pix<4, Ch32, Hsl, Straight, Linear>;

/// [Hsl](struct.Hsl.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Hsla8p = Pix<4, Ch8, Hsl, Premultiplied, Linear>;

/// [Hsl](struct.Hsl.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Hsla16p = Pix<4, Ch16, Hsl, Premultiplied, Linear>;

/// [Hsl](struct.Hsl.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Hsla32p = Pix<4, Ch32, Hsl, Premultiplied, Linear>;

/// [Hsl](struct.Hsl.html) 8-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SHsl8 = Pix<3, Ch8, Hsl, Straight, Srgb>;

/// [Hsl](struct.Hsl.html) 16-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SHsl16 = Pix<3, Ch16, Hsl, Straight, Srgb>;

/// [Hsl](struct.Hsl.html) 32-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SHsl32 = Pix<3, Ch32, Hsl, Straight, Srgb>;

/// [Hsl](struct.Hsl.html) 8-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type SHsla8 = Pix<4, Ch8, Hsl, Straight, Srgb>;

/// [Hsl](struct.Hsl.html) 16-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type SHsla16 = Pix<4, Ch16, Hsl, Straight, Srgb>;

/// [Hsl](struct.Hsl.html) 32-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type SHsla32 = Pix<4, Ch32, Hsl, Straight, Srgb>;

/// [Hsl](struct.Hsl.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SHsla8p = Pix<4, Ch8, Hsl, Premultiplied, Srgb>;

/// [Hsl](struct.Hsl.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SHsla16p = Pix<4, Ch16, Hsl, Premultiplied, Srgb>;

/// [Hsl](struct.Hsl.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SHsla32p = Pix<4, Ch32, Hsl, Premultiplied, Srgb>;

#[cfg(test)]
mod test {
    use crate::el::Pixel;
    use crate::hsl::*;
    use crate::rgb::*;

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
