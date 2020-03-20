// hsv.rs       HSV color model
//
// Copyright (c) 2019-2020  Jeron Aldaron Lau
// Copyright (c) 2020  Douglas P Lau
//
use crate::alpha::{
    self, AChannel, Mode as _, Opaque, Premultiplied, Straight, Translucent,
};
use crate::gamma::{self, Linear};
use crate::hue::{Hexcone, rgb_to_hue_chroma_value};
use crate::{Ch16, Ch32, Ch8, Channel, ColorModel, Pixel};
use std::marker::PhantomData;

/// `HSV` hexcone [color model], also known as `HSB`.
///
/// The components are *hue*, *saturation* and *value* (or *brightness*), with
/// optional *[alpha]*.
///
/// [alpha]: alpha/trait.AChannel.html
/// [color model]: trait.ColorModel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    hue: C,
    components: [C; 2],
    alpha: A,
    mode: PhantomData<M>,
    gamma: PhantomData<G>,
}

impl<C, A, M, G> Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Create an `Hsv` color.
    ///
    /// ## Example
    /// ```
    /// # use pix::*;
    /// let opaque_hsv = Hsv8::new(50, 255, 128, ());
    /// let translucent_hsv = Hsva8::new(100, 128, 255, 200);
    /// ```
    pub fn new<H, B>(hue: H, saturation: H, value: H, alpha: B) -> Self
    where
        C: From<H>,
        A: From<B>,
    {
        let hue = C::from(hue);
        let saturation = C::from(saturation);
        let value = C::from(value);
        let components = [saturation, value];
        let alpha = A::from(alpha);
        Hsv {
            hue,
            components,
            alpha,
            mode: PhantomData,
            gamma: PhantomData,
        }
    }
    /// Get the *hue* component.
    pub fn hue(self) -> C {
        self.hue
    }
    /// Get the *saturation* component.
    pub fn saturation(self) -> C {
        self.components[0]
    }
    /// Get the *value* component.
    pub fn value(self) -> C {
        self.components[1]
    }
}

impl<C, A, M, G> ColorModel for Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Chan = C;

    /// Get all components affected by alpha/gamma
    fn components(&self) -> &[Self::Chan] {
        &self.components
    }

    /// Get the *alpha* component
    fn alpha(self) -> Self::Chan {
        self.alpha.value()
    }

    /// Convert to *red*, *green*, *blue* and *alpha* components
    fn to_rgba(self) -> [Self::Chan; 4] {
        let v = self.value();
        let chroma = v * self.saturation();
        let hp = self.hue().into() * 6.0; // 0.0..=6.0
        let hc = Hexcone::from_hue_prime(hp);
        let (red, green, blue) = hc.rgb(chroma);
        let m = v - chroma;
        [red + m, green + m, blue + m, self.alpha()]
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self {
        let red = rgba[0];
        let green = rgba[1];
        let blue = rgba[2];
        let alpha = rgba[3];
        let (hue, chroma, val) = rgb_to_hue_chroma_value(red, green, blue);
        let sat_v = if val > C::MIN { chroma / val } else { C::MIN };
        Hsv::new(hue, sat_v, val, alpha)
    }
}

impl<C, A, M, G> Pixel for Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Alpha = M;
    type Gamma = G;
}

impl<C, A, M, G> Iterator for Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

impl<C, M, G> From<Hsv<C, Translucent<C>, M, G>> for Hsv<C, Opaque<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Hsv<C, Translucent<C>, M, G>) -> Self {
        Hsv::new(c.hue(), c.saturation(), c.value(), ())
    }
}

impl<C, M, G> From<Hsv<C, Opaque<C>, M, G>> for Hsv<C, Translucent<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Hsv<C, Opaque<C>, M, G>) -> Self {
        Hsv::new(c.hue(), c.saturation(), c.value(), ())
    }
}

impl<C, A, G> From<Hsv<C, A, Straight, G>> for Hsv<C, A, Premultiplied, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: Hsv<C, A, Straight, G>) -> Self {
        let hue = c.hue();
        let alpha = c.alpha();
        let saturation = Premultiplied::encode(c.saturation(), alpha);
        let value = Premultiplied::encode(c.value(), alpha);
        Hsv::new(hue, saturation, value, alpha)
    }
}

impl<C, A, G> From<Hsv<C, A, Premultiplied, G>> for Hsv<C, A, Straight, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: Hsv<C, A, Premultiplied, G>) -> Self {
        let hue = c.hue();
        let alpha = c.alpha();
        let saturation = Premultiplied::decode(c.saturation(), alpha);
        let value = Premultiplied::decode(c.value(), alpha);
        Hsv::new(hue, saturation, value, alpha)
    }
}

/// [Hsv](struct.Hsv.html) 8-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsv8 = Hsv<Ch8, Opaque<Ch8>, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 16-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsv16 = Hsv<Ch16, Opaque<Ch16>, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 32-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsv32 = Hsv<Ch32, Opaque<Ch32>, Straight, Linear>;

/// [Hsv](struct.Hsv.html) 8-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva8 = Hsv<Ch8, Translucent<Ch8>, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 16-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva16 = Hsv<Ch16, Translucent<Ch16>, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 32-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva32 = Hsv<Ch32, Translucent<Ch32>, Straight, Linear>;

/// [Hsv](struct.Hsv.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva8p = Hsv<Ch8, Translucent<Ch8>, Premultiplied, Linear>;
/// [Hsv](struct.Hsv.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva16p = Hsv<Ch16, Translucent<Ch16>, Premultiplied, Linear>;
/// [Hsv](struct.Hsv.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva32p = Hsv<Ch32, Translucent<Ch32>, Premultiplied, Linear>;

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
        assert_eq!(
            Rgb8::new(255, 0, 0, ()),
            Hsv8::new(0, 255, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 255, 0, ()),
            Hsv32::new(60.0 / 360.0, 1.0, 1.0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 255, 0, ()),
            Hsv16::new(21845, 65535, 65535, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 255, 255, ()),
            Hsv32::new(0.5, 1.0, 1.0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 0, 255, ()),
            Hsv32::new(240.0 / 360.0, 1.0, 1.0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 0, 255, ()),
            Hsv32::new(300.0 / 360.0, 1.0, 1.0, ()).convert(),
        );
    }

    #[test]
    fn hsv_to_rgb_unsat() {
        assert_eq!(
            Rgb8::new(255, 127, 127, ()),
            Hsv8::new(0, 128, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 255, 128, ()),
            Hsv32::new(60.0 / 360.0, 0.5, 1.0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(127, 255, 127, ()),
            Hsv16::new(21845, 32768, 65535, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(128, 255, 255, ()),
            Hsv32::new(180.0 / 360.0, 0.5, 1.0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(128, 128, 255, ()),
            Hsv32::new(240.0 / 360.0, 0.5, 1.0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 128, 255, ()),
            Hsv32::new(300.0 / 360.0, 0.5, 1.0, ()).convert(),
        );
    }

    #[test]
    fn hsv_to_rgb_dark() {
        assert_eq!(
            Rgb8::new(128, 0, 0, ()),
            Hsv8::new(0, 255, 128, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(128, 128, 0, ()),
            Hsv32::new(60.0 / 360.0, 1.0, 0.5, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 128, 0, ()),
            Hsv16::new(21845, 65535, 32768, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 128, 128, ()),
            Hsv32::new(180.0 / 360.0, 1.0, 0.5, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 0, 128, ()),
            Hsv32::new(240.0 / 360.0, 1.0, 0.5, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(128, 0, 128, ()),
            Hsv32::new(300.0 / 360.0, 1.0, 0.5, ()).convert(),
        );
    }

    #[test]
    fn hsv_to_rgb_hue() {
        assert_eq!(
            Rgb8::new(255, 192, 0, ()),
            Hsv8::new(32, 255, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(126, 255, 0, ()),
            Hsv8::new(64, 255, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 255, 66, ()),
            Hsv8::new(96, 255, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 60, 255, ()),
            Hsv8::new(160, 255, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(132, 0, 255, ()),
            Hsv8::new(192, 255, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 0, 186, ()),
            Hsv8::new(224, 255, 255, ()).convert(),
        );
    }

    #[test]
    fn hsv_to_rgb_grays() {
        assert_eq!(
            Rgb8::new(255, 255, 255, ()),
            Hsv8::new(0, 0, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(128, 128, 128, ()),
            Hsv8::new(100, 0, 128, ()).convert(),
        );
        assert_eq!(
            Hsv8::new(0, 0, 255, ()),
            Rgb8::new(255, 255, 255, ()).convert(),
        );
        assert_eq!(
            Hsv8::new(0, 0, 128, ()),
            Rgb8::new(128, 128, 128, ()).convert(),
        );
    }

    #[test]
    fn rgb_to_hsv() {
        assert_eq!(
            Hsv8::new(0, 255, 255, ()),
            Rgb8::new(255, 0, 0, ()).convert(),
        );
        assert_eq!(
            Hsv32::new(60.0 / 360.0, 1.0, 1.0, ()),
            Rgb8::new(255, 255, 0, ()).convert(),
        );
        assert_eq!(
            Hsv16::new(21845, 65535, 65535, ()),
            Rgb8::new(0, 255, 0, ()).convert(),
        );
        assert_eq!(
            Hsv32::new(0.5, 1.0, 1.0, ()),
            Rgb8::new(0, 255, 255, ()).convert(),
        );
        assert_eq!(
            Hsv32::new(240.0 / 360.0, 1.0, 1.0, ()),
            Rgb8::new(0, 0, 255, ()).convert(),
        );
        assert_eq!(
            Hsv32::new(300.0 / 360.0, 1.0, 1.0, ()),
            Rgb8::new(255, 0, 255, ()).convert(),
        );
    }

    #[test]
    fn rgb_to_hsv_unsat() {
        assert_eq!(
            Hsv8::new(0, 128, 255, ()),
            Rgb8::new(255, 127, 127, ()).convert(),
        );
        assert_eq!(
            Hsv8::new(42, 128, 255, ()),
            Rgb8::new(255, 255, 127, ()).convert(),
        );
        assert_eq!(
            Hsv8::new(85, 127, 255, ()),
            Rgb8::new(128, 255, 128, ()).convert(),
        );
        assert_eq!(
            Hsv8::new(128, 127, 255, ()),
            Rgb8::new(128, 255, 255, ()).convert()
        );
        assert_eq!(
            Hsv8::new(170, 127, 255, ()),
            Rgb8::new(128, 128, 255, ()).convert(),
        );
        assert_eq!(
            Hsv8::new(213, 127, 255, ()),
            Rgb8::new(255, 128, 255, ()).convert(),
        );
    }
}
