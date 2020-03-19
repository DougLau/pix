// rgb.rs       RGB color model.
//
// Copyright (c) 2018-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::alpha::{
    self, AChannel, Mode as _, Opaque, Premultiplied, Straight, Translucent,
};
use crate::gamma::{self, Linear};
use crate::{Ch16, Ch32, Ch8, Channel, ColorModel, Pixel};
use std::marker::PhantomData;
use std::ops::Mul;

/// `RGB` additive [color model].
///
/// The components are *red*, *green* and *blue*, with optional *[alpha]*.
///
/// [alpha]: alpha/trait.AChannel.html
/// [color model]: trait.ColorModel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Rgb<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    components: [C; 3],
    alpha: A,
    mode: PhantomData<M>,
    gamma: PhantomData<G>,
}

impl<C, A, M, G> Rgb<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Create an `Rgb` color.
    ///
    /// ## Example
    /// ```
    /// # use pix::*;
    /// let opaque_rgb = Rgb8::new(50, 255, 128, ());
    /// let translucent_rgb = Rgba8::new(100, 128, 255, 200);
    /// ```
    pub fn new<H, B>(red: H, green: H, blue: H, alpha: B) -> Self
    where
        C: From<H>,
        A: From<B>,
    {
        let red = C::from(red);
        let green = C::from(green);
        let blue = C::from(blue);
        let components = [red, green, blue];
        let alpha = A::from(alpha);
        Rgb {
            components,
            alpha,
            mode: PhantomData,
            gamma: PhantomData,
        }
    }
    /// Get the *red* component.
    pub fn red(self) -> C {
        self.components[0]
    }
    /// Get the *green* component.
    pub fn green(self) -> C {
        self.components[1]
    }
    /// Get the *blue* component.
    pub fn blue(self) -> C {
        self.components[2]
    }

    /// Get channel-wise difference
    pub fn difference(self, rhs: Self) -> Self
    where
        A: From<C>,
    {
        let r = if self.red() > rhs.red() {
            self.red() - rhs.red()
        } else {
            rhs.red() - self.red()
        };
        let g = if self.green() > rhs.green() {
            self.green() - rhs.green()
        } else {
            rhs.green() - self.green()
        };
        let b = if self.blue() > rhs.blue() {
            self.blue() - rhs.blue()
        } else {
            rhs.blue() - self.blue()
        };
        let a = if self.alpha.value() > rhs.alpha.value() {
            self.alpha.value() - rhs.alpha.value()
        } else {
            rhs.alpha.value() - self.alpha.value()
        };
        Rgb::new(r, g, b, a)
    }

    /// Check if all `Channel`s are within threshold
    pub fn within_threshold(self, rhs: Self) -> bool {
        self.red() <= rhs.red()
            && self.green() <= rhs.green()
            && self.blue() <= rhs.blue()
            && self.alpha.value() <= rhs.alpha.value()
    }
}

impl<C, A, M, G> ColorModel for Rgb<C, A, M, G>
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
        [self.red(), self.green(), self.blue(), self.alpha()]
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self {
        let red = rgba[0];
        let green = rgba[1];
        let blue = rgba[2];
        let alpha = rgba[3];
        Rgb::new(red, green, blue, alpha)
    }
}

impl<C, A, M, G> Pixel for Rgb<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Alpha = M;
    type Gamma = G;
}

impl<C, A, M, G> Iterator for Rgb<C, A, M, G>
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

impl<C, M, G> From<Rgb<C, Translucent<C>, M, G>> for Rgb<C, Opaque<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Rgb<C, Translucent<C>, M, G>) -> Self {
        Rgb::new(c.red(), c.green(), c.blue(), ())
    }
}

impl<C, M, G> From<Rgb<C, Opaque<C>, M, G>> for Rgb<C, Translucent<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Rgb<C, Opaque<C>, M, G>) -> Self {
        Rgb::new(c.red(), c.green(), c.blue(), ())
    }
}

impl<C, A, G> From<Rgb<C, A, Straight, G>> for Rgb<C, A, Premultiplied, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: Rgb<C, A, Straight, G>) -> Self {
        let red = Premultiplied::encode(c.red(), c.alpha());
        let green = Premultiplied::encode(c.green(), c.alpha());
        let blue = Premultiplied::encode(c.blue(), c.alpha());
        Rgb::new(red, green, blue, c.alpha())
    }
}

impl<C, A, G> From<Rgb<C, A, Premultiplied, G>> for Rgb<C, A, Straight, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: Rgb<C, A, Premultiplied, G>) -> Self {
        let alpha = c.alpha();
        let red = Premultiplied::decode(c.red(), alpha);
        let green = Premultiplied::decode(c.green(), alpha);
        let blue = Premultiplied::decode(c.blue(), alpha);
        Rgb::new(red, green, blue, alpha)
    }
}

impl<C, A, M, G> From<i32> for Rgb<C, A, M, G>
where
    C: Channel + From<Ch8>,
    A: AChannel<Chan = C> + From<Translucent<Ch8>>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Get an `Rgb` from an `i32`
    fn from(c: i32) -> Self {
        let red = Ch8::from(c as u8);
        let green = Ch8::from((c >> 8) as u8);
        let blue = Ch8::from((c >> 16) as u8);
        let alpha = Ch8::from((c >> 24) as u8);
        Rgb::new(red, green, blue, Translucent::new(alpha))
    }
}

impl<C, A, M, G> From<Rgb<C, A, M, G>> for i32
where
    C: Channel,
    Ch8: From<C>,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Get an `i32` from an `Rgb`
    fn from(c: Rgb<C, A, M, G>) -> i32 {
        let red: u8 = Ch8::from(c.red()).into();
        let red = i32::from(red);
        let green: u8 = Ch8::from(c.green()).into();
        let green = i32::from(green) << 8;
        let blue: u8 = Ch8::from(c.blue()).into();
        let blue = i32::from(blue) << 16;
        let alpha: u8 = Ch8::from(c.alpha()).into();
        let alpha = i32::from(alpha) << 24;
        red | green | blue | alpha
    }
}

impl<C, A, G> Mul<Self> for Rgb<C, A, Straight, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    G: gamma::Mode,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let red = self.red() * rhs.red();
        let green = self.green() * rhs.green();
        let blue = self.blue() * rhs.blue();
        let components = [red, green, blue];
        let alpha = self.alpha * rhs.alpha;
        Rgb {
            components,
            alpha,
            mode: PhantomData,
            gamma: PhantomData,
        }
    }
}

impl<C, A, G> Mul<Self> for Rgb<C, A, Premultiplied, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let this: Rgb<C, A, Straight, G> = self.into();
        let other: Rgb<C, A, Straight, G> = rhs.into();

        (this * other).into()
    }
}

/// [Rgb](struct.Rgb.html) 8-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgb8 = Rgb<Ch8, Opaque<Ch8>, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 16-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgb16 = Rgb<Ch16, Opaque<Ch16>, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 32-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgb32 = Rgb<Ch32, Opaque<Ch32>, Straight, Linear>;

/// [Rgb](struct.Rgb.html) 8-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgba8 = Rgb<Ch8, Translucent<Ch8>, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 16-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgba16 = Rgb<Ch16, Translucent<Ch16>, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 32-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgba32 = Rgb<Ch32, Translucent<Ch32>, Straight, Linear>;

/// [Rgb](struct.Rgb.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgba8p = Rgb<Ch8, Translucent<Ch8>, Premultiplied, Linear>;
/// [Rgb](struct.Rgb.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgba16p = Rgb<Ch16, Translucent<Ch16>, Premultiplied, Linear>;
/// [Rgb](struct.Rgb.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgba32p = Rgb<Ch32, Translucent<Ch32>, Premultiplied, Linear>;

type SRgb<C, A> = Rgb<C, A, Straight, gamma::Srgb>;
/// [Rgb](struct.Rgb.html) 8-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgb8 = SRgb<Ch8, Opaque<Ch8>>;
/// [Rgb](struct.Rgb.html) 16-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgb16 = SRgb<Ch16, Opaque<Ch16>>;
/// [Rgb](struct.Rgb.html) 32-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgb32 = SRgb<Ch32, Opaque<Ch32>>;

type SRgba<C, A> = Rgb<C, A, Straight, gamma::Srgb>;
/// [Rgb](struct.Rgb.html) 8-bit [straight](alpha/struct.Straight.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgba8 = SRgba<Ch8, Translucent<Ch8>>;
/// [Rgb](struct.Rgb.html) 16-bit [straight](alpha/struct.Straight.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgba16 = SRgba<Ch16, Translucent<Ch16>>;
/// [Rgb](struct.Rgb.html) 32-bit [straight](alpha/struct.Straight.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgba32 = SRgba<Ch32, Translucent<Ch32>>;

type SRgbap<C, A> = Rgb<C, A, Premultiplied, gamma::Srgb>;
/// [Rgb](struct.Rgb.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgba8p = SRgbap<Ch8, Translucent<Ch8>>;
/// [Rgb](struct.Rgb.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgba16p = SRgbap<Ch16, Translucent<Ch16>>;
/// [Rgb](struct.Rgb.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgba32p = SRgbap<Ch32, Translucent<Ch32>>;

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

    #[test]
    fn check_mul() {
        let a = SRgba8::new(0xFF, 0xFF, 0xFF, 0xFF);
        let b = SRgba8::new(0x00, 0x00, 0x00, 0x00);
        assert_eq!(a * b, b);

        let a = SRgba8::new(0xFF, 0xFF, 0xFF, 0xFF);
        let b = SRgba8::new(0x80, 0x80, 0x80, 0x80);
        assert_eq!(a * b, b);

        let a = SRgba8::new(0xFF, 0xF0, 0x00, 0x70);
        let b = SRgba8::new(0x80, 0x00, 0x60, 0xFF);
        assert_eq!(a * b, SRgba8::new(0x80, 0x00, 0x00, 0x70));

        let a = SRgba8::new(0xFF, 0x00, 0x80, 0xFF);
        let b = SRgba8::new(0xFF, 0xFF, 0xFF, 0x10);
        assert_eq!(a * b, SRgba8::new(0xFF, 0x00, 0x80, 0x10));
    }
}
