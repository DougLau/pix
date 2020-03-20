// hwb.rs       HWB color model
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::alpha::{
    self, AChannel, Mode as _, Opaque, Premultiplied, Straight, Translucent,
};
use crate::gamma::{self, Linear};
use crate::hue::{Hexcone, rgb_to_hue_chroma_value};
use crate::{Ch16, Ch32, Ch8, Channel, ColorModel, Pixel};
use std::marker::PhantomData;

/// `HWB` [color model].
///
/// The components are *hue*, *whiteness* and *blackness*, with optional
/// *[alpha]*.
///
/// [alpha]: alpha/trait.AChannel.html
/// [color model]: trait.ColorModel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Hwb<C, A, M, G>
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

impl<C, A, M, G> Hwb<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Create an `Hwb` color.
    ///
    /// ## Example
    /// ```
    /// # use pix::*;
    /// let opaque_hwb = Hwb8::new(50, 64, 0, ());
    /// let translucent_hwb = Hwba8::new(100, 0, 128, 200);
    /// ```
    pub fn new<H, B>(hue: H, whiteness: H, blackness: H, alpha: B) -> Self
    where
        C: From<H>,
        A: From<B>,
    {
        let hue = C::from(hue);
        let whiteness = C::from(whiteness);
        let blackness = C::from(blackness);
        let components = [whiteness, blackness];
        let alpha = A::from(alpha);
        Hwb {
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
    /// Get the *whiteness* component.
    pub fn whiteness(self) -> C {
        self.components[0]
    }
    /// Get the *blackness* component.
    pub fn blackness(self) -> C {
        self.components[1]
    }
    /// Get *whiteness* and *blackness* clamped to 1.0 at the same ratio
    fn whiteness_blackness(self) -> (C, C) {
        let whiteness = self.whiteness();
        let blackness = self.blackness();
        if whiteness + blackness - blackness < whiteness {
            let (w, b) = (whiteness.into(), blackness.into());
            let ratio = 1.0 / (w + b);
            (C::from(w * ratio), C::from(b * ratio))
        } else {
            (whiteness, blackness)
        }
    }
}

impl<C, A, M, G> ColorModel for Hwb<C, A, M, G>
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
        let (whiteness, blackness) = self.whiteness_blackness();
        let v = C::MAX - blackness;
        let chroma = v - whiteness;
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
        let whiteness = (C::MAX - sat_v) * val;
        let blackness = C::MAX - val;
        Hwb::new(hue, whiteness, blackness, alpha)
    }
}

impl<C, A, M, G> Pixel for Hwb<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Alpha = M;
    type Gamma = G;
}

impl<C, A, M, G> Iterator for Hwb<C, A, M, G>
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

impl<C, M, G> From<Hwb<C, Translucent<C>, M, G>> for Hwb<C, Opaque<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Hwb<C, Translucent<C>, M, G>) -> Self {
        Hwb::new(c.hue(), c.whiteness(), c.blackness(), ())
    }
}

impl<C, M, G> From<Hwb<C, Opaque<C>, M, G>> for Hwb<C, Translucent<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Hwb<C, Opaque<C>, M, G>) -> Self {
        Hwb::new(c.hue(), c.whiteness(), c.blackness(), ())
    }
}

impl<C, A, G> From<Hwb<C, A, Straight, G>> for Hwb<C, A, Premultiplied, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: Hwb<C, A, Straight, G>) -> Self {
        let hue = c.hue();
        let alpha = c.alpha();
        let whiteness = Premultiplied::encode(c.whiteness(), alpha);
        let blackness = Premultiplied::encode(c.blackness(), alpha);
        Hwb::new(hue, whiteness, blackness, alpha)
    }
}

impl<C, A, G> From<Hwb<C, A, Premultiplied, G>> for Hwb<C, A, Straight, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: Hwb<C, A, Premultiplied, G>) -> Self {
        let hue = c.hue();
        let alpha = c.alpha();
        let whiteness = Premultiplied::decode(c.whiteness(), alpha);
        let blackness = Premultiplied::decode(c.blackness(), alpha);
        Hwb::new(hue, whiteness, blackness, alpha)
    }
}

/// [Hwb](struct.Hwb.html) 8-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hwb8 = Hwb<Ch8, Opaque<Ch8>, Straight, Linear>;
/// [Hwb](struct.Hwb.html) 16-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hwb16 = Hwb<Ch16, Opaque<Ch16>, Straight, Linear>;
/// [Hwb](struct.Hwb.html) 32-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hwb32 = Hwb<Ch32, Opaque<Ch32>, Straight, Linear>;

/// [Hwb](struct.Hwb.html) 8-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hwba8 = Hwb<Ch8, Translucent<Ch8>, Straight, Linear>;
/// [Hwb](struct.Hwb.html) 16-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hwba16 = Hwb<Ch16, Translucent<Ch16>, Straight, Linear>;
/// [Hwb](struct.Hwb.html) 32-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hwba32 = Hwb<Ch32, Translucent<Ch32>, Straight, Linear>;

/// [Hwb](struct.Hwb.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hwba8p = Hwb<Ch8, Translucent<Ch8>, Premultiplied, Linear>;
/// [Hwb](struct.Hwb.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hwba16p = Hwb<Ch16, Translucent<Ch16>, Premultiplied, Linear>;
/// [Hwb](struct.Hwb.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hwba32p = Hwb<Ch32, Translucent<Ch32>, Premultiplied, Linear>;

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
        assert_eq!(
            Rgb8::new(127, 127, 127, ()),
            Hwb8::new(0, 128, 128, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(127, 127, 127, ()),
            Hwb8::new(0, 255, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(85, 85, 85, ()),
            Hwb8::new(0, 128, 255, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 0, 0, ()),
            Hwb8::new(0, 0, 0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 255, 128, ()),
            Hwb32::new(60.0 / 360.0, 0.5, 0.0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 127, 0, ()),
            Hwb16::new(21845, 0, 32768, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(128, 255, 255, ()),
            Hwb32::new(0.5, 0.5, 0.0, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(0, 0, 128, ()),
            Hwb32::new(240.0 / 360.0, 0.0, 0.5, ()).convert(),
        );
        assert_eq!(
            Rgb8::new(255, 128, 255, ()),
            Hwb32::new(300.0 / 360.0, 0.5, 0.0, ()).convert(),
        );
    }

    #[test]
    fn rgb_to_hwb() {
        assert_eq!(
            Hwb8::new(0, 0, 0, ()),
            Rgb8::new(255, 0, 0, ()).convert(),
        );
        assert_eq!(
            Hwb8::new(0, 64, 0, ()),
            Rgb8::new(255, 64, 64, ()).convert(),
        );
        assert_eq!(
            Hwb32::new(60.0 / 360.0, 0.0, 0.50196075, ()),
            Rgb8::new(127, 127, 0, ()).convert(),
        );
        assert_eq!(
            Hwb16::new(21845, 8224, 0, ()),
            Rgb8::new(32, 255, 32, ()).convert(),
        );
        assert_eq!(
            Hwb32::new(0.5, 0.0, 0.7490196, ()),
            Rgb8::new(0, 64, 64, ()).convert(),
        );
        assert_eq!(
            Hwb32::new(240.0 / 360.0, 0.7529412, 0.0, ()),
            Rgb8::new(192, 192, 255, ()).convert(),
        );
        assert_eq!(
            Hwb32::new(300.0 / 360.0, 0.0, 0.0, ()),
            Rgb8::new(255, 0, 255, ()).convert(),
        );
    }
}
