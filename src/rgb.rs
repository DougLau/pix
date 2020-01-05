// rgb.rs       RGB pixel format.
//
// Copyright (c) 2018-2020  Douglas P Lau
//
use crate::{
    Alpha, Ch16, Ch32, Ch8, Channel, Format, Opaque, PixModes, Translucent, AlphaModeID, AlphaMode, Associated, Separated, GammaMode, Srgb, Linear, GammaModeID
};
use std::ops::Mul;
use std::marker::PhantomData;

/// RGB pixel [Format](trait.Format.html), with optional
/// [Alpha](trait.Alpha.html) channel.
///
/// The `Channel`s are *red*, *green* and *blue*.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Rgb<C: Channel, A: Alpha, M: AlphaMode, G: GammaMode> {
    mode: PhantomData<M>,
    gamma: PhantomData<G>,
    red: C,
    green: C,
    blue: C,
    alpha: A,
}

impl<C: Channel, A: Alpha, M: AlphaMode, G: GammaMode> GammaMode for Rgb<C, A, M, G> {
    const ID: GammaModeID = G::ID;

    /// Encode one `Channel` using the gamma mode.
    fn encode<H: Channel, Gm: GammaMode>(h: H) -> H {
        G::encode::<H, Gm>(h)
    }
    /// Decode one `Channel` using the gamma mode.
    fn decode<H: Channel, Gm: GammaMode>(h: H) -> H {
        G::decode::<H, Gm>(h)
    }
}

impl<C: Channel, A: Alpha, M: AlphaMode, G: GammaMode> AlphaMode for Rgb<C, A, M, G> {
    const ID: AlphaModeID = M::ID;

    /// Encode one `Channel` using the gamma mode.
    fn encode<H: Channel, B: Alpha<Chan = H>>(h: H, b: B) -> H {
        M::encode::<H, B>(h, b)
    }
    /// Decode one `Channel` using the gamma mode.
    fn decode<H: Channel, B: Alpha<Chan = H>>(h: H, b: B) -> H {
        M::decode::<H, B>(h, b)
    }
}

impl<C: Channel, A: Alpha> PixModes for Rgb<C, A, Associated, Srgb> {
    fn alpha_mode() -> AlphaModeID {
        AlphaModeID::Associated
    }

    fn gamma_mode() -> GammaModeID {
        GammaModeID::Srgb
    }
}

impl<C: Channel, A: Alpha> PixModes for Rgb<C, A, Separated, Srgb> {
    fn alpha_mode() -> AlphaModeID {
        AlphaModeID::Separated
    }

    fn gamma_mode() -> GammaModeID {
        GammaModeID::Srgb
    }
}

impl<C: Channel, A: Alpha> PixModes for Rgb<C, A, Associated, Linear> {
    fn alpha_mode() -> AlphaModeID {
        AlphaModeID::Associated
    }

    fn gamma_mode() -> GammaModeID {
        GammaModeID::Linear
    }
}

impl<C: Channel, A: Alpha> PixModes for Rgb<C, A, Separated, Linear> {
    fn alpha_mode() -> AlphaModeID {
        AlphaModeID::Separated
    }

    fn gamma_mode() -> GammaModeID {
        GammaModeID::Linear
    }
}

impl<C: Channel, A: Alpha, M: AlphaMode, G: GammaMode> Iterator for Rgb<C, A, M, G> {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

impl<C, M, G: GammaMode> From<Rgb<C, Translucent<C>, M, G>> for Rgb<C, Opaque<C>, M, G>
where
    C: Channel,
    M: AlphaMode
{
    fn from(c: Rgb<C, Translucent<C>, M, G>) -> Self {
        Rgb::new(c.red(), c.green(), c.blue())
    }
}

impl<C, M, G: GammaMode> From<Rgb<C, Opaque<C>, M, G>> for Rgb<C, Translucent<C>, M, G>
where
    C: Channel,
    M: AlphaMode
{
    fn from(c: Rgb<C, Opaque<C>, M, G>) -> Self {
        Rgb::with_alpha(c.red(), c.green(), c.blue(), C::MAX)
    }
}

impl<C, A, G: GammaMode> From<Rgb<C, A, Separated, G>> for Rgb<C, A, Associated, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
{
    fn from(c: Rgb<C, A, Separated, G>) -> Self {
        let red = Associated::encode::<C, A>(c.red, c.alpha);
        let green = Associated::encode::<C, A>(c.green, c.alpha);
        let blue = Associated::encode::<C, A>(c.blue, c.alpha);

        Rgb::with_alpha(red, green, blue, c.alpha())
    }
}

impl<C, A, G: GammaMode> From<Rgb<C, A, Associated, G>> for Rgb<C, A, Separated, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
{
    fn from(c: Rgb<C, A, Associated, G>) -> Self {
        let red = Associated::decode::<C, A>(c.red, c.alpha);
        let green = Associated::decode::<C, A>(c.green, c.alpha);
        let blue = Associated::decode::<C, A>(c.blue, c.alpha);

        Rgb::with_alpha(red, green, blue, c.alpha())
    }
}

impl<C, A, M, G: GammaMode> From<i32> for Rgb<C, A, M, G>
where
    C: Channel + From<Ch8>,
    A: Alpha<Chan = C> + From<Translucent<Ch8>>,
    M: AlphaMode
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

impl<C, A, M, G: GammaMode> From<Rgb<C, A, M, G>> for i32
where
    C: Channel,
    Ch8: From<C>,
    A: Alpha<Chan = C>,
    M: AlphaMode
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

impl<C: Channel, A: Alpha, G: GammaMode> Mul<Self> for Rgb<C, A, Separated, G> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let red = self.red * rhs.red;
        let green = self.green * rhs.green;
        let blue = self.blue * rhs.blue;
        let alpha = self.alpha * rhs.alpha;
        Rgb {
            mode: std::marker::PhantomData,
            gamma: std::marker::PhantomData,
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl<C: Channel, A: Alpha<Chan = C>, G: GammaMode> Mul<Self> for Rgb<C, A, Associated, G> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let this: Rgb<C, A, Separated, G> = self.into();
        let other: Rgb<C, A, Separated, G> = rhs.into();

        (this * other).into()
    }
}

impl<C: Channel, A: Alpha, M: AlphaMode, G: GammaMode> Rgb<C, A, M, G> {
    /// Create an [Opaque](struct.Opaque.html) color by specifying *red*,
    /// *green* and *blue* values.
    pub fn new<H>(red: H, green: H, blue: H) -> Self
    where
        C: From<H>,
        A: From<Opaque<C>>,
    {
        Self::with_alpha(red, green, blue, Opaque::default())
    }
    /// Create a [Translucent](struct.Translucent.html) color by specifying
    /// *red*, *green*, *blue* and *alpha* values.
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
            mode: std::marker::PhantomData,
            gamma: std::marker::PhantomData,
            red,
            green,
            blue,
            alpha,
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

impl<C, A, M, G: GammaMode> Format for Rgb<C, A, M, G>
where
    C: Channel,
    A: Alpha<Chan = C> + From<C>,
    M: AlphaMode
{
    type Chan = C;

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

    /// Encode into associated alpha from separate alpha.
    fn encode(mut self) -> Self {
        self.red = Associated::encode(self.red, self.alpha);
        self.green = Associated::encode(self.green, self.alpha);
        self.blue = Associated::encode(self.blue, self.alpha);
        self
    }

    /// Decode into separate alpha from associated alpha.
    fn decode(mut self) -> Self {
        self.red = Associated::decode(self.red, self.alpha);
        self.green = Associated::decode(self.green, self.alpha);
        self.blue = Associated::decode(self.blue, self.alpha);
        self
    }
}

/// [Opaque](struct.Opaque.html) 8-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgb8 = Rgb<Ch8, Opaque<Ch8>, Separated, Srgb>;

/// [Opaque](struct.Opaque.html) 16-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgb16 = Rgb<Ch16, Opaque<Ch16>, Separated, Srgb>;

/// [Opaque](struct.Opaque.html) 32-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgb32 = Rgb<Ch32, Opaque<Ch32>, Separated, Srgb>;

/// [Opaque](struct.Opaque.html) 8-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type LinearRgb8 = Rgb<Ch8, Opaque<Ch8>, Separated, Linear>;

/// [Opaque](struct.Opaque.html) 16-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type LinearRgb16 = Rgb<Ch16, Opaque<Ch16>, Separated, Linear>;

/// [Opaque](struct.Opaque.html) 32-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type LinearRgb32 = Rgb<Ch32, Opaque<Ch32>, Separated, Linear>;

/// [Translucent](struct.Translucent.html) 8-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgba8 = Rgb<Ch8, Translucent<Ch8>, Separated, Srgb>;

/// [Translucent](struct.Translucent.html) 16-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgba16 = Rgb<Ch16, Translucent<Ch16>, Separated, Srgb>;

/// [Translucent](struct.Translucent.html) 32-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgba32 = Rgb<Ch32, Translucent<Ch32>, Separated, Srgb>;

/// [Translucent](struct.Translucent.html) 8-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type LinearRgba8 = Rgb<Ch8, Translucent<Ch8>, Separated, Linear>;

/// [Translucent](struct.Translucent.html) 16-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type LinearRgba16 = Rgb<Ch16, Translucent<Ch16>, Separated, Linear>;

/// [Translucent](struct.Translucent.html) 32-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type LinearRgba32 = Rgb<Ch32, Translucent<Ch32>, Separated, Linear>;

/// [Translucent](struct.Translucent.html) 8-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type PremulRgba8 = Rgb<Ch8, Translucent<Ch8>, Associated, Srgb>;

/// [Translucent](struct.Translucent.html) 16-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type PremulRgba16 = Rgb<Ch16, Translucent<Ch16>, Associated, Srgb>;

/// [Translucent](struct.Translucent.html) 32-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type PremulRgba32 = Rgb<Ch32, Translucent<Ch32>, Associated, Srgb>;

/// [Translucent](struct.Translucent.html) 8-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type PremulLinearRgba8 = Rgb<Ch8, Translucent<Ch8>, Associated, Linear>;

/// [Translucent](struct.Translucent.html) 16-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type PremulLinearRgba16 = Rgb<Ch16, Translucent<Ch16>, Associated, Linear>;

/// [Translucent](struct.Translucent.html) 32-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type PremulLinearRgba32 = Rgb<Ch32, Translucent<Ch32>, Associated, Linear>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<Rgb8>(), 3);
        assert_eq!(std::mem::size_of::<Rgb16>(), 6);
        assert_eq!(std::mem::size_of::<Rgb32>(), 12);
        assert_eq!(std::mem::size_of::<Rgba8>(), 4);
        assert_eq!(std::mem::size_of::<Rgba16>(), 8);
        assert_eq!(std::mem::size_of::<Rgba32>(), 16);
    }

    #[test]
    fn check_mul() {
        let a = Rgba8::with_alpha(0xFF, 0xFF, 0xFF, 0xFF);
        let b = Rgba8::with_alpha(0x00, 0x00, 0x00, 0x00);

        assert_eq!(a * b, b);

        let a = Rgba8::with_alpha(0xFF, 0xFF, 0xFF, 0xFF);
        let b = Rgba8::with_alpha(0x80, 0x80, 0x80, 0x80);

        assert_eq!(a * b, b);

        let a = Rgba8::with_alpha(0xFF, 0xF0, 0x00, 0x70);
        let b = Rgba8::with_alpha(0x80, 0x00, 0x60, 0xFF);

        assert_eq!(a * b, Rgba8::with_alpha(0x80, 0x00, 0x00, 0x70));

        let a = Rgba8::with_alpha(0xFF, 0x00, 0x80, 0xFF);
        let b = Rgba8::with_alpha(0xFF, 0xFF, 0xFF, 0x10);

        assert_eq!(a * b, Rgba8::with_alpha(0xFF, 0x00, 0x80, 0x10));
    }
}
