// rgb.rs       RGB pixel format.
//
// Copyright (c) 2018-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::alpha::{
    self, Alpha, Mode as _, Opaque, Premultiplied, Straight, Translucent,
};
use crate::gamma::{self, Linear};
use crate::{Ch16, Ch32, Ch8, Channel, Format};
use std::marker::PhantomData;
use std::ops::Mul;

/// RGB color model, with optional [Alpha](alpha/trait.Alpha.html) channel.
///
/// The `Channel`s are *red*, *green* and *blue*.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Rgb<C: Channel, A: Alpha, M: alpha::Mode, G: gamma::Mode> {
    mode: PhantomData<M>,
    gamma: PhantomData<G>,
    red: C,
    green: C,
    blue: C,
    alpha: A,
}

impl<C, A, M, G> Iterator for Rgb<C, A, M, G>
where
    C: Channel,
    A: Alpha,
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
        Rgb::new(c.red(), c.green(), c.blue())
    }
}

impl<C, M, G> From<Rgb<C, Opaque<C>, M, G>> for Rgb<C, Translucent<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Rgb<C, Opaque<C>, M, G>) -> Self {
        Rgb::with_alpha(c.red(), c.green(), c.blue(), C::MAX)
    }
}

impl<C, A, G> From<Rgb<C, A, Straight, G>> for Rgb<C, A, Premultiplied, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
    G: gamma::Mode,
{
    fn from(c: Rgb<C, A, Straight, G>) -> Self {
        let red = Premultiplied::encode::<C, A>(c.red, c.alpha);
        let green = Premultiplied::encode::<C, A>(c.green, c.alpha);
        let blue = Premultiplied::encode::<C, A>(c.blue, c.alpha);
        Rgb::with_alpha(red, green, blue, c.alpha)
    }
}

impl<C, A, G> From<Rgb<C, A, Premultiplied, G>> for Rgb<C, A, Straight, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
    G: gamma::Mode,
{
    fn from(c: Rgb<C, A, Premultiplied, G>) -> Self {
        let red = Premultiplied::decode::<C, A>(c.red, c.alpha);
        let green = Premultiplied::decode::<C, A>(c.green, c.alpha);
        let blue = Premultiplied::decode::<C, A>(c.blue, c.alpha);
        Rgb::with_alpha(red, green, blue, c.alpha)
    }
}

impl<C, A, M, G> From<i32> for Rgb<C, A, M, G>
where
    C: Channel + From<Ch8>,
    A: Alpha + From<Translucent<Ch8>>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Get an `Rgb` from an `i32`
    fn from(c: i32) -> Self {
        let red = Ch8::from(c as u8);
        let green = Ch8::from((c >> 8) as u8);
        let blue = Ch8::from((c >> 16) as u8);
        let alpha = Ch8::from((c >> 24) as u8);
        Rgb::with_alpha(red, green, blue, Translucent::new(alpha))
    }
}

impl<C, A, M, G> From<Rgb<C, A, M, G>> for i32
where
    C: Channel,
    Ch8: From<C>,
    A: Alpha<Chan = C>,
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
        let alpha: u8 = Ch8::from(c.alpha().value()).into();
        let alpha = i32::from(alpha) << 24;
        red | green | blue | alpha
    }
}

impl<C, A, G> Mul<Self> for Rgb<C, A, Straight, G>
where
    C: Channel,
    A: Alpha,
    G: gamma::Mode,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let red = self.red * rhs.red;
        let green = self.green * rhs.green;
        let blue = self.blue * rhs.blue;
        let alpha = self.alpha * rhs.alpha;
        Rgb {
            red,
            green,
            blue,
            alpha,
            mode: PhantomData,
            gamma: PhantomData,
        }
    }
}

impl<C, A, G> Mul<Self> for Rgb<C, A, Premultiplied, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
    G: gamma::Mode,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let this: Rgb<C, A, Straight, G> = self.into();
        let other: Rgb<C, A, Straight, G> = rhs.into();

        (this * other).into()
    }
}

impl<C, A, M, G> Rgb<C, A, M, G>
where
    C: Channel,
    A: Alpha,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Create an [Opaque](alpha/struct.Opaque.html) color by specifying *red*,
    /// *green* and *blue* values.
    pub fn new<H>(red: H, green: H, blue: H) -> Self
    where
        C: From<H>,
        A: From<Opaque<C>>,
    {
        Self::with_alpha(red, green, blue, Opaque::default())
    }
    /// Create a [Translucent](alpha/struct.Translucent.html) color by
    /// specifying *red*, *green*, *blue* and *alpha* values.
    pub fn with_alpha<H, B>(red: H, green: H, blue: H, alpha: B) -> Self
    where
        C: From<H>,
        A: From<B>,
    {
        let red = C::from(red);
        let green = C::from(green);
        let blue = C::from(blue);
        let alpha = A::from(alpha);
        Rgb {
            red,
            green,
            blue,
            alpha,
            mode: PhantomData,
            gamma: PhantomData,
        }
    }
    /// Get the red `Channel`.
    pub fn red(self) -> C {
        self.red
    }
    /// Get the green `Channel`.
    pub fn green(self) -> C {
        self.green
    }
    /// Get the blue `Channel`.
    pub fn blue(self) -> C {
        self.blue
    }
    /// Get the alpha `Channel`.
    pub fn alpha(self) -> A {
        self.alpha
    }
}

impl<C, A, M, G> Format for Rgb<C, A, M, G>
where
    C: Channel,
    A: Alpha<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Chan = C;
    type Alpha = M;
    type Gamma = G;

    /// Get *red*, *green*, *blue* and *alpha* `Channel`s
    fn rgba(self) -> [Self::Chan; 4] {
        [self.red, self.green, self.blue, self.alpha.value()]
    }

    /// Make a pixel with given RGBA `Channel`s
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self {
        let red = rgba[0];
        let green = rgba[1];
        let blue = rgba[2];
        let alpha = rgba[3];
        Rgb::with_alpha(red, green, blue, alpha)
    }

    /// Get channel-wise difference
    fn difference(self, rhs: Self) -> Self {
        let r = if self.red > rhs.red {
            self.red - rhs.red
        } else {
            rhs.red - self.red
        };
        let g = if self.green > rhs.green {
            self.green - rhs.green
        } else {
            rhs.green - self.green
        };
        let b = if self.blue > rhs.blue {
            self.blue - rhs.blue
        } else {
            rhs.blue - self.blue
        };
        let a = if self.alpha.value() > rhs.alpha.value() {
            self.alpha.value() - rhs.alpha.value()
        } else {
            rhs.alpha.value() - self.alpha.value()
        };
        Rgb::with_alpha(r, g, b, a)
    }

    /// Check if all `Channel`s are within threshold
    fn within_threshold(self, rhs: Self) -> bool {
        self.red <= rhs.red
            && self.green <= rhs.green
            && self.blue <= rhs.blue
            && self.alpha.value() <= rhs.alpha.value()
    }
}

/// [Rgb](struct.Rgb.html) 8-bit [opaque](alpha/struct.Opaque.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type Rgb8 = Rgb<Ch8, Opaque<Ch8>, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 16-bit [opaque](alpha/struct.Opaque.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type Rgb16 = Rgb<Ch16, Opaque<Ch16>, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 32-bit [opaque](alpha/struct.Opaque.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type Rgb32 = Rgb<Ch32, Opaque<Ch32>, Straight, Linear>;

/// [Rgb](struct.Rgb.html) 8-bit
/// [straight alpha](alpha/struct.Straight.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type Rgba8 = Rgb<Ch8, Translucent<Ch8>, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 16-bit
/// [straight alpha](alpha/struct.Straight.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type Rgba16 = Rgb<Ch16, Translucent<Ch16>, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 32-bit
/// [straight alpha](alpha/struct.Straight.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type Rgba32 = Rgb<Ch32, Translucent<Ch32>, Straight, Linear>;

/// [Rgb](struct.Rgb.html) 8-bit
/// [premultiplied alpha](alpha/struct.Premultiplied.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type Rgba8p = Rgb<Ch8, Translucent<Ch8>, Premultiplied, Linear>;
/// [Rgb](struct.Rgb.html) 16-bit
/// [premultiplied alpha](alpha/struct.Premultiplied.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type Rgba16p = Rgb<Ch16, Translucent<Ch16>, Premultiplied, Linear>;
/// [Rgb](struct.Rgb.html) 32-bit
/// [premultiplied alpha](alpha/struct.Premultiplied.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type Rgba32p = Rgb<Ch32, Translucent<Ch32>, Premultiplied, Linear>;

type SRgb<C, A> = Rgb<C, A, Straight, gamma::Srgb>;
/// [Rgb](struct.Rgb.html) 8-bit [opaque](alpha/struct.Opaque.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SRgb8 = SRgb<Ch8, Opaque<Ch8>>;
/// [Rgb](struct.Rgb.html) 16-bit [opaque](alpha/struct.Opaque.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SRgb16 = SRgb<Ch16, Opaque<Ch16>>;
/// [Rgb](struct.Rgb.html) 32-bit [opaque](alpha/struct.Opaque.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SRgb32 = SRgb<Ch32, Opaque<Ch32>>;

type SRgba<C, A> = Rgb<C, A, Straight, gamma::Srgb>;
/// [Rgb](struct.Rgb.html) 8-bit
/// [straight alpha](alpha/struct.Straight.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SRgba8 = SRgba<Ch8, Translucent<Ch8>>;
/// [Rgb](struct.Rgb.html) 16-bit
/// [straight alpha](alpha/struct.Straight.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SRgba16 = SRgba<Ch16, Translucent<Ch16>>;
/// [Rgb](struct.Rgb.html) 32-bit
/// [straight alpha](alpha/struct.Straight.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SRgba32 = SRgba<Ch32, Translucent<Ch32>>;

type SRgbap<C, A> = Rgb<C, A, Premultiplied, gamma::Srgb>;
/// [Rgb](struct.Rgb.html) 8-bit
/// [premultiplied alpha](alpha/struct.Premultiplied.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SRgba8p = SRgbap<Ch8, Translucent<Ch8>>;
/// [Rgb](struct.Rgb.html) 16-bit
/// [premultiplied alpha](alpha/struct.Premultiplied.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SRgba16p = SRgbap<Ch16, Translucent<Ch16>>;
/// [Rgb](struct.Rgb.html) 32-bit
/// [premultiplied alpha](alpha/struct.Premultiplied.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
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
        let a = SRgba8::with_alpha(0xFF, 0xFF, 0xFF, 0xFF);
        let b = SRgba8::with_alpha(0x00, 0x00, 0x00, 0x00);
        assert_eq!(a * b, b);

        let a = SRgba8::with_alpha(0xFF, 0xFF, 0xFF, 0xFF);
        let b = SRgba8::with_alpha(0x80, 0x80, 0x80, 0x80);
        assert_eq!(a * b, b);

        let a = SRgba8::with_alpha(0xFF, 0xF0, 0x00, 0x70);
        let b = SRgba8::with_alpha(0x80, 0x00, 0x60, 0xFF);
        assert_eq!(a * b, SRgba8::with_alpha(0x80, 0x00, 0x00, 0x70));

        let a = SRgba8::with_alpha(0xFF, 0x00, 0x80, 0xFF);
        let b = SRgba8::with_alpha(0xFF, 0xFF, 0xFF, 0x10);
        assert_eq!(a * b, SRgba8::with_alpha(0xFF, 0x00, 0x80, 0x10));
    }
}
