// mask.rs      Alpha mask color model.
//
// Copyright (c) 2019-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::alpha::{AChannel, Straight, Translucent};
use crate::gamma::Linear;
use crate::model::Channels;
use crate::{Ch16, Ch32, Ch8, Channel, ColorModel, Pixel};
use std::ops::Mul;

/// `Mask` [color model] (*alpha* only).
///
/// [color model]: trait.ColorModel.html
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
        let alpha = Translucent::new(C::from(alpha));
        Mask { alpha }
    }

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba(self) -> [C; 4] {
        [C::MAX, C::MAX, C::MAX, self.alpha()]
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba(rgba: [C; 4]) -> Self {
        Mask::new(rgba[3])
    }
}

impl<C: Channel> ColorModel for Mask<C> {
    type Chan = C;

    /// Get the *alpha* component
    fn alpha(self) -> Self::Chan {
        self.alpha.value()
    }

    /// Convert into channels shared by types
    fn into_channels<R: ColorModel>(self) -> Channels<C> {
        Channels::new(self.into_rgba(), 3)
    }

    /// Convert from channels shared by types
    fn from_channels<R: ColorModel>(channels: Channels<C>) -> Self {
        debug_assert_eq!(channels.alpha(), 3);
        Self::from_rgba(channels.into_array())
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
