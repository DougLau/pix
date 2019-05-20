// mask.rs      Alpha mask pixel format.
//
// Copyright (c) 2019  Douglas P Lau
//
use crate::{Alpha, Channel, Ch8, Ch16, Ch32, Format, PixModes, Translucent};
use std::marker::PhantomData;

/// [Translucent](struct.Translucent.html) alpha mask pixel
/// [Format](trait.Format.html).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Mask<C: Channel, A: Alpha> {
    value: PhantomData<C>,
    alpha: A,
}

impl<C: Channel, A: Alpha> PixModes for Mask<C, A> { }

impl<C: Channel, A: Alpha> Iterator for Mask<C, A> {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

impl<C, A> From<u8> for Mask<C, A>
    where C: Channel + From<u8>, A: Alpha + From<C>
{
    /// Get a `Mask` from a `u8`
    fn from(c: u8) -> Self {
        Mask::new(c)
    }
}

impl<C, A> From<u16> for Mask<C, A>
    where C: Channel + From<u16>, A: Alpha + From<C>
{
    /// Get a `Mask` from a `u16`
    fn from(c: u16) -> Self {
        Mask::new(c)
    }
}

impl<C, A> From<f32> for Mask<C, A>
    where C: Channel + From<f32>, A: Alpha + From<C>
{
    /// Get a `Mask` from an `f32`
    fn from(c: f32) -> Self {
        Mask::new(c)
    }
}

impl<C: Channel, A: Alpha> Mask<C, A> {
    /// Create a new `Mask` value.
    pub fn new<B>(alpha: B) -> Self
        where C: From<B>, A: From<C>
    {
        let value = PhantomData;
        let alpha = A::from(C::from(alpha));
        Mask { value, alpha }
    }
    /// Get the alpha value.
    pub fn alpha(self) -> A {
        self.alpha
    }
}

impl<C, A> Format for Mask<C, A>
    where C: Channel, A: Alpha<Chan=C> + From<C>
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
}

/// [Translucent](struct.Translucent.html) 8-bit alpha [Mask](struct.Mask.html)
/// pixel [Format](trait.Format.html).
pub type Mask8 = Mask<Ch8, Translucent<Ch8>>;

/// [Translucent](struct.Translucent.html) 16-bit alpha [Mask](struct.Mask.html)
/// pixel [Format](trait.Format.html).
pub type Mask16 = Mask<Ch16, Translucent<Ch16>>;

/// [Translucent](struct.Translucent.html) 32-bit alpha [Mask](struct.Mask.html)
/// pixel [Format](trait.Format.html).
pub type Mask32 = Mask<Ch32, Translucent<Ch32>>;

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
