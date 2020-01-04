// mask.rs      Alpha mask pixel format.
//
// Copyright (c) 2019-2020  Douglas P Lau
//
use crate::{
    Alpha, Ch16, Ch32, Ch8, Channel, Format, PixModes, Rgb, Gray, Translucent, AlphaMode2, AlphaMode, GammaMode2, GammaMode
};
use std::ops::Mul;

/// [Translucent](struct.Translucent.html) alpha mask pixel
/// [Format](trait.Format.html).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Mask<A: Alpha> {
    alpha: A,
}

impl<A: Alpha> PixModes for Mask<A> {
    fn alpha_mode(&self) -> AlphaMode {
        // Alpha Mode is a no-op on Mask
        AlphaMode::UnknownAlpha
    }

    fn gamma_mode(&self) -> GammaMode {
        // Gamma Mode is a no-op on Mask
        GammaMode::UnknownGamma
    }
}

impl<A: Alpha> Iterator for Mask<A> {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

impl From<u8> for Mask8 {
    /// Get a `Mask` from a `u8`
    fn from(c: u8) -> Self {
        Mask::new(c)
    }
}

impl From<u16> for Mask16 {
    /// Get a `Mask` from a `u16`
    fn from(c: u16) -> Self {
        Mask::new(c)
    }
}

impl From<f32> for Mask32 {
    /// Get a `Mask` from an `f32`
    fn from(c: f32) -> Self {
        Mask::new(c)
    }
}

impl<C, A, M, G: GammaMode2> From<Mask<A>> for Rgb<C, A, M, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
    M: AlphaMode2
{
    /// Get an `Rgb` from a `Mask`
    fn from(c: Mask<A>) -> Self {
        let red = C::MAX;
        let green = C::MAX;
        let blue = C::MAX;
        let alpha = c.alpha();
        Rgb::with_alpha(red, green, blue, alpha)
    }
}

impl<C, A, M: AlphaMode2, G: GammaMode2> From<Mask<A>> for Gray<C, A, M, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
{
    /// Get a `Gray` from a `Mask`
    fn from(c: Mask<A>) -> Self {
        let value = C::MAX;
        let alpha = c.alpha().into();
        Gray::with_alpha(value, alpha)
    }
}

impl<A: Alpha> Mul<Self> for Mask<A> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let alpha = self.alpha * rhs.alpha;
        Mask { alpha }
    }
}

impl<A: Alpha> Mask<A> {
    /// Create a new `Mask` value.
    pub fn new<B>(alpha: B) -> Self
    where
        A: From<B>,
    {
        let alpha = A::from(alpha);
        Mask { alpha }
    }
    /// Get the alpha value.
    pub fn alpha(self) -> A {
        self.alpha
    }
}

impl<C, A> Format for Mask<A>
where
    C: Channel,
    A: Alpha<Chan = C> + From<C>,
{
    type Chan = C;

    /// Get *red*, *green*, *blue* and *alpha* `Channel`s
    fn rgba(self) -> [Self::Chan; 4] {
        [C::MAX, C::MAX, C::MAX, self.alpha.value()]
    }

    /// Make a pixel with given RGBA `Channel`s
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self {
        let alpha = rgba[3];
        Mask::new(alpha)
    }

    /// Get channel-wise difference
    fn difference(self, rhs: Self) -> Self {
        let a = if self.alpha.value() > rhs.alpha.value() {
            self.alpha.value() - rhs.alpha.value()
        } else {
            rhs.alpha.value() - self.alpha.value()
        };
        Mask::new(a)
    }

    /// Check if all `Channel`s are within threshold
    fn within_threshold(self, rhs: Self) -> bool {
        self.alpha.value() <= rhs.alpha.value()
    }

    /// Encode into associated alpha from separate alpha.
    fn encode(self) -> Self {
        self
    }

    /// Decode into separate alpha from associated alpha.
    fn decode(self) -> Self {
        self
    }
}

/// [Translucent](struct.Translucent.html) 8-bit alpha [Mask](struct.Mask.html)
/// pixel [Format](trait.Format.html).
pub type Mask8 = Mask<Translucent<Ch8>>;

/// [Translucent](struct.Translucent.html) 16-bit alpha [Mask](struct.Mask.html)
/// pixel [Format](trait.Format.html).
pub type Mask16 = Mask<Translucent<Ch16>>;

/// [Translucent](struct.Translucent.html) 32-bit alpha [Mask](struct.Mask.html)
/// pixel [Format](trait.Format.html).
pub type Mask32 = Mask<Translucent<Ch32>>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<Mask8>(), 1);
        assert_eq!(std::mem::size_of::<Mask16>(), 2);
        assert_eq!(std::mem::size_of::<Mask32>(), 4);
    }
}
