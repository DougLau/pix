// hsl.rs       HSL color model
//
// Copyright (c) 2019-2020  Jeron Aldaron Lau
// Copyright (c) 2020  Douglas P Lau
//
use crate::chan::{Ch16, Ch32, Ch8, Channel, Linear, Premultiplied, Straight};
use crate::clr::{
    hue::{rgb_to_hue_chroma_value, Hexcone},
    ColorModel,
};
use crate::el::{Pix3, Pix4, PixRgba, Pixel};
use std::ops::Range;

/// [HSL] bi-hexcone [color model].
///
/// The components are *[hue]*, *[saturation]*, *[lightness]* and optional
/// *[alpha]*.
///
/// [alpha]: #method.alpha
/// [color model]: trait.ColorModel.html
/// [hue]: #method.hue
/// [hsl]: https://en.wikipedia.org/wiki/HSL_and_HSV
/// [lightness]: #method.lightness
/// [saturation]: #method.saturation
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
    /// use pix::Hsl32;
    /// use pix::chan::Ch32;
    /// use pix::clr::Hsl;
    ///
    /// let p = Hsl32::new(0.25, 0.5, 1.0);
    /// assert_eq!(Hsl::hue(p), Ch32::new(0.25));
    /// ```
    /// [Channel::MIN]: chan/trait.Channel.html#associatedconstant.MIN
    /// [Channel::MAX]: chan/trait.Channel.html#associatedconstant.MAX
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
    /// use pix::Hsl16;
    /// use pix::chan::Ch16;
    /// use pix::clr::Hsl;
    ///
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
    /// use pix::Hsl8;
    /// use pix::chan::Ch8;
    /// use pix::clr::Hsl;
    ///
    /// let p = Hsl8::new(0x93, 0x80, 0xA0);
    /// assert_eq!(Hsl::lightness(p), Ch8::new(0xA0));
    /// ```
    pub fn lightness<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }

    /// Get the *alpha* component.
    ///
    /// # Example: HSL Alpha
    /// ```
    /// use pix::Hsla8;
    /// use pix::chan::Ch8;
    /// use pix::clr::Hsl;
    ///
    /// let p = Hsla8::new(0x50, 0xA0, 0x80, 0xB0);
    /// assert_eq!(Hsl::alpha(p), Ch8::new(0xB0));
    /// ```
    pub fn alpha<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.four()
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
        let vl = 1.0 - (2.0 * Self::lightness(p).into() - 1.0).abs();
        let chroma = P::Chan::from(vl) * Self::saturation(p);
        let hp = Self::hue(p).into() * 6.0; // 0.0..=6.0
        let hc = Hexcone::from_hue_prime(hp);
        let (red, green, blue) = hc.rgb(chroma);
        let m = Self::lightness(p) - chroma * P::Chan::from(0.5);

        let red = (red + m).into();
        let green = (green + m).into();
        let blue = (blue + m).into();
        PixRgba::<P>::new(red, green, blue, Self::alpha(p).into())
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

/// [Hsl](clr/struct.Hsl.html) 8-bit opaque (no *alpha* channel)
/// [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsl8 = Pix3<Ch8, Hsl, Straight, Linear>;
/// [Hsl](clr/struct.Hsl.html) 16-bit opaque (no *alpha* channel)
/// [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsl16 = Pix3<Ch16, Hsl, Straight, Linear>;
/// [Hsl](clr/struct.Hsl.html) 32-bit opaque (no *alpha* channel)
/// [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsl32 = Pix3<Ch32, Hsl, Straight, Linear>;

/// [Hsl](clr/struct.Hsl.html) 8-bit [straight](chan/struct.Straight.html)
/// alpha [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsla8 = Pix4<Ch8, Hsl, Straight, Linear>;
/// [Hsl](clr/struct.Hsl.html) 16-bit [straight](chan/struct.Straight.html)
/// alpha [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsla16 = Pix4<Ch16, Hsl, Straight, Linear>;
/// [Hsl](clr/struct.Hsl.html) 32-bit [straight](chan/struct.Straight.html)
/// alpha [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsla32 = Pix4<Ch32, Hsl, Straight, Linear>;

/// [Hsl](clr/struct.Hsl.html) 8-bit
/// [premultiplied](chan/struct.Premultiplied.html) alpha
/// [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsla8p = Pix4<Ch8, Hsl, Premultiplied, Linear>;
/// [Hsl](clr/struct.Hsl.html) 16-bit
/// [premultiplied](chan/struct.Premultiplied.html) alpha
/// [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsla16p = Pix4<Ch16, Hsl, Premultiplied, Linear>;
/// [Hsl](clr/struct.Hsl.html) 32-bit
/// [premultiplied](chan/struct.Premultiplied.html) alpha
/// [linear](chan/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Hsla32p = Pix4<Ch32, Hsl, Premultiplied, Linear>;

#[cfg(test)]
mod test {
    use crate::el::Pixel;
    use crate::*;

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
