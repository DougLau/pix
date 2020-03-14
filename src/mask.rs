// mask.rs      Alpha mask color model.
//
// Copyright (c) 2019-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::alpha::{AChannel, Premultiplied, Straight, Translucent};
use crate::gamma::{self, Linear};
use crate::{Ch16, Ch32, Ch8, Channel, ColorModel, Gray, Pixel, Rgb};
use std::ops::Mul;

/// [Translucent] alpha mask [color model].
///
/// [color model]: trait.ColorModel.html
/// [translucent]: alpha/struct.Translucent.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Mask<C: Channel> {
    alpha: Translucent<C>,
}

impl<C: Channel> Mask<C> {
    /// Create a new `Mask` value.
    pub fn new<A>(alpha: A) -> Self
    where
        C: From<A>,
    {
        let alpha = C::from(alpha).into();
        Mask { alpha }
    }
}

impl<C: Channel> ColorModel for Mask<C> {
    type Chan = C;

    /// Get all non-alpha components
    fn components(&self) -> &[Self::Chan] {
        &[]
    }

    /// Get the *alpha* component
    fn alpha(self) -> Self::Chan {
        self.alpha.value()
    }

    /// Convert to *red*, *green*, *blue* and *alpha* components
    fn to_rgba(self) -> [Self::Chan; 4] {
        [C::MAX, C::MAX, C::MAX, self.alpha()]
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self {
        Mask::new(rgba[3])
    }

    /// Get channel-wise difference
    fn difference(self, rhs: Self) -> Self {
        let a = if self.alpha() > rhs.alpha() {
            self.alpha() - rhs.alpha()
        } else {
            rhs.alpha() - self.alpha()
        };
        Mask::new(a)
    }

    /// Check if all `Channel`s are within threshold
    fn within_threshold(self, rhs: Self) -> bool {
        self.alpha() <= rhs.alpha()
    }
}

impl<C> Pixel for Mask<C>
where
    C: Channel,
{
    type Alpha = Straight;
    type Gamma = Linear;
}

impl<C: Channel> Iterator for Mask<C> {
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

impl<C, A, G> From<Mask<C>> for Rgb<C, A, Straight, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    /// Get an `Rgb` from a `Mask`
    fn from(c: Mask<C>) -> Self {
        let red = C::MAX;
        let green = C::MAX;
        let blue = C::MAX;
        let alpha = c.alpha();
        Rgb::with_alpha(red, green, blue, alpha)
    }
}

impl<C, A, G> From<Mask<C>> for Rgb<C, A, Premultiplied, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    /// Get an `Rgb` from a `Mask`
    fn from(c: Mask<C>) -> Self {
        let red = c.alpha();
        let green = c.alpha();
        let blue = c.alpha();
        let alpha = c.alpha();
        Rgb::with_alpha(red, green, blue, alpha)
    }
}

impl<C, A, G> From<Mask<C>> for Gray<C, A, Straight, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    /// Get a `Gray` from a `Mask`
    fn from(c: Mask<C>) -> Self {
        let value = C::MAX;
        let alpha = c.alpha();
        Gray::with_alpha(value, alpha)
    }
}

impl<C, A, G> From<Mask<C>> for Gray<C, A, Premultiplied, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    /// Get a `Gray` from a `Mask`
    fn from(c: Mask<C>) -> Self {
        let value = c.alpha();
        let alpha = c.alpha();
        Gray::with_alpha(value, alpha)
    }
}

impl<C: Channel> Mul<Self> for Mask<C> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let alpha = self.alpha * rhs.alpha;
        Mask { alpha }
    }
}

/// [Mask](struct.Mask.html) 8-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Mask8 = Mask<Ch8>;

/// [Mask](struct.Mask.html) 16-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Mask16 = Mask<Ch16>;

/// [Mask](struct.Mask.html) 32-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Mask32 = Mask<Ch32>;

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
