// hsv.rs       HSV color model
//
// Copyright (c) 2019-2020  Jeron Aldaron Lau
// Copyright (c) 2020  Douglas P Lau
//
//! [HSV] color model and types.
//!
//! [hsv]: https://en.wikipedia.org/wiki/HSL_and_HSV
use crate::chan::{Ch16, Ch32, Ch8, Linear, Premultiplied, Srgb, Straight};
use crate::el::{Pix3, Pix4, PixRgba, Pixel};
use crate::hue::{rgb_to_hue_chroma_value, Hexcone};
use crate::ColorModel;
use std::ops::Range;

/// [HSV] hexcone [color model], also known as HSB.
///
/// The components are *[hue]*, *[saturation]*, *[value]* (or *brightness*) and
/// optional *[alpha]*.
///
/// [alpha]: ../el/trait.Pixel.html#method.alpha
/// [color model]: ../trait.ColorModel.html
/// [hue]: #method.hue
/// [hsv]: https://en.wikipedia.org/wiki/HSL_and_HSV
/// [saturation]: #method.saturation
/// [value]: #method.value
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
    /// use pix::chan::Ch32;
    /// use pix::hsv::{Hsv, Hsv32};
    ///
    /// let p = Hsv32::new(0.25, 0.5, 1.0);
    /// assert_eq!(Hsv::hue(p), Ch32::new(0.25));
    /// ```
    /// [Channel::MIN]: ../chan/trait.Channel.html#associatedconstant.MIN
    /// [Channel::MAX]: ../chan/trait.Channel.html#associatedconstant.MAX
    pub fn hue<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }

    /// Get a mutable reference to the *hue* component.
    ///
    /// # Example: Modify HSV Hue
    /// ```
    /// use pix::chan::{Ch32, Channel};
    /// use pix::hsv::{Hsv, Hsv32};
    ///
    /// let mut p = Hsv32::new(0.2, 0.75, 0.5);
    /// let mut h = Hsv::hue_mut(&mut p);
    /// *h = h.wrapping_sub(Ch32::new(0.4));
    /// assert_eq!(Hsv::hue(p), Ch32::new(0.8));
    /// ```
    pub fn hue_mut<P: Pixel>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one_mut()
    }

    /// Get the *saturation* component.
    ///
    /// Lower values are more gray (desaturated), while higher values are more
    /// colorful.  NOTE: HSV saturation is slightly different from [HSL]
    /// saturation.
    ///
    /// # Example: HSV Saturation
    /// ```
    /// use pix::chan::Ch16;
    /// use pix::hsv::{Hsv, Hsv16};
    ///
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

    /// Get a mutable reference to the *saturation* component.
    ///
    /// # Example: Modify HSV Saturation
    /// ```
    /// use pix::chan::Ch16;
    /// use pix::hsv::{Hsv, Hsv16};
    ///
    /// let mut p = Hsv16::new(0x2000, 0x1234, 0x8000);
    /// *Hsv::saturation_mut(&mut p) = Ch16::new(0x4321);
    /// assert_eq!(Hsv::saturation(p), Ch16::new(0x4321));
    /// ```
    pub fn saturation_mut<P: Pixel>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two_mut()
    }

    /// Get the *value* (or *brightness*) component.
    ///
    /// Lower values are closer to *black*, while higher values are closer to
    /// fully bright colors.
    ///
    /// # Example: HSV Value
    /// ```
    /// use pix::chan::Ch8;
    /// use pix::hsv::{Hsv, Hsv8};
    ///
    /// let p = Hsv8::new(0x93, 0x80, 0xA0);
    /// assert_eq!(Hsv::value(p), Ch8::new(0xA0));
    /// ```
    pub fn value<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }

    /// Get a mutable reference to the *value* component.
    ///
    /// # Example: Modify HSV Value
    /// ```
    /// use pix::chan::Ch8;
    /// use pix::hsv::{Hsv, Hsv8};
    ///
    /// let mut p = Hsv8::new(0x93, 0x80, 0xA0);
    /// *Hsv::value_mut(&mut p) = Ch8::new(0xBB);
    /// assert_eq!(Hsv::value(p), Ch8::new(0xBB));
    /// ```
    pub fn value_mut<P: Pixel>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three_mut()
    }
}

impl ColorModel for Hsv {
    const CIRCULAR: Range<usize> = 0..1;
    const LINEAR: Range<usize> = 1..3;
    const ALPHA: usize = 3;

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> PixRgba<P>
    where
        P: Pixel<Model = Self>,
    {
        let v = Self::value(p);
        let chroma = v * Self::saturation(p);
        let hp = Self::hue(p).into() * 6.0; // 0.0..=6.0
        let hc = Hexcone::from_hue_prime(hp);
        let (red, green, blue) = hc.rgb(chroma);
        let m = v - chroma;
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
        let sat_v = chroma / val;
        P::from_channels(&[hue, sat_v, val, alpha])
    }
}

/// [Hsv](struct.Hsv.html) 8-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Hsv8 = Pix3<Ch8, Hsv, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 16-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Hsv16 = Pix3<Ch16, Hsv, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 32-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Hsv32 = Pix3<Ch32, Hsv, Straight, Linear>;

/// [Hsv](struct.Hsv.html) 8-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Hsva8 = Pix4<Ch8, Hsv, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 16-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Hsva16 = Pix4<Ch16, Hsv, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 32-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Hsva32 = Pix4<Ch32, Hsv, Straight, Linear>;

/// [Hsv](struct.Hsv.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Hsva8p = Pix4<Ch8, Hsv, Premultiplied, Linear>;
/// [Hsv](struct.Hsv.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Hsva16p = Pix4<Ch16, Hsv, Premultiplied, Linear>;
/// [Hsv](struct.Hsv.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Hsva32p = Pix4<Ch32, Hsv, Premultiplied, Linear>;

/// [Hsv](struct.Hsv.html) 8-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SHsv8 = Pix3<Ch8, Hsv, Straight, Srgb>;
/// [Hsv](struct.Hsv.html) 16-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SHsv16 = Pix3<Ch16, Hsv, Straight, Srgb>;
/// [Hsv](struct.Hsv.html) 32-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SHsv32 = Pix3<Ch32, Hsv, Straight, Srgb>;

/// [Hsv](struct.Hsv.html) 8-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type SHsva8 = Pix4<Ch8, Hsv, Straight, Srgb>;
/// [Hsv](struct.Hsv.html) 16-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type SHsva16 = Pix4<Ch16, Hsv, Straight, Srgb>;
/// [Hsv](struct.Hsv.html) 32-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type SHsva32 = Pix4<Ch32, Hsv, Straight, Srgb>;

/// [Hsv](struct.Hsv.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SHsva8p = Pix4<Ch8, Hsv, Premultiplied, Srgb>;
/// [Hsv](struct.Hsv.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SHsva16p = Pix4<Ch16, Hsv, Premultiplied, Srgb>;
/// [Hsv](struct.Hsv.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SHsva32p = Pix4<Ch32, Hsv, Premultiplied, Srgb>;

#[cfg(test)]
mod test {
    use crate::el::Pixel;
    use crate::hsv::*;
    use crate::rgb::*;

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
