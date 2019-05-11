// mask.rs      Alpha mask pixel format.
//
// Copyright (c) 2019  Douglas P Lau
//
use crate::{Format, Rgb};
use crate::alpha::{Alpha, Translucent};
use crate::channel::{Channel, Ch8, Ch16, Ch32};
use std::marker::PhantomData;

/// [Translucent](struct.Translucent.html) alpha mask pixel
/// [Format](trait.Format.html).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Mask<C: Channel, A: Alpha> {
    value: PhantomData<C>,
    alpha: A,
}

impl<C, A> From<u8> for Mask<C, A>
    where C: Channel, A: Alpha, A: From<Ch8>
{
    /// Get a Mask from a u8
    fn from(c: u8) -> Self {
        Mask::new(Ch8::new(c))
    }
}

impl<C, A> From<u16> for Mask<C, A>
    where C: Channel, A: Alpha, A: From<Ch16>
{
    /// Get a Mask from a u16
    fn from(c: u16) -> Self {
        Mask::new(Ch16::new(c))
    }
}

impl<A: Alpha> From<i32> for Mask<Ch8, A>
    where A: From<Ch8>
{
    /// Get a Mask<Ch8, A> from an i32
    fn from(c: i32) -> Self {
        Mask::new(Ch8::new(c as u8))
    }
}

impl<A: Alpha> From<i32> for Mask<Ch16, A>
    where A: From<Ch16>
{
    /// Get a Mask<Ch16, A> from an i32
    fn from(c: i32) -> Self {
        Mask::new(Ch16::new(c as u16))
    }
}

impl<C, A> From<f32> for Mask<C, A>
    where C: Channel, A: Alpha, A: From<Ch32>
{
    /// Get a Mask from an f32
    fn from(c: f32) -> Self {
        Mask::new(Ch32::new(c))
    }
}

impl<C, H, A, B> From<Rgb<H, B>> for Mask<C, A>
    where C: Channel, C: From<H>, H: Channel, A: Alpha, A: From<B>, B: Alpha
{
    /// Get a Mask from an Rgb
    fn from(c: Rgb<H, B>) -> Self {
        Mask::new(c.alpha())
    }
}

impl<C, H, A, B> From<Mask<H, B>> for Rgb<C, A>
    where C: Channel, C: From<H>, H: Channel, A: Alpha, A: From<B>, B: Alpha
{
    /// Get an Rgb from a Mask
    fn from(c: Mask<H, B>) -> Self {
        let v = C::MAX;
        let a = A::from(c.alpha());
        Rgb::with_alpha(v, v, v, a)
    }
}

impl<C: Channel, A: Alpha> Mask<C, A> {
    /// Create a new Mask value.
    pub fn new<V>(alpha: V) -> Self where A: From<V> {
        let value = PhantomData;
        let alpha = A::from(alpha);
        Mask { value, alpha }
    }
    /// Get the alpha value.
    pub fn alpha(self) -> A {
        self.alpha
    }
}

impl<C: Channel, A: Alpha> Format for Mask<C, A> {
    type Chan = C;
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
        assert!(std::mem::size_of::<Mask8>() == 1);
        assert!(std::mem::size_of::<Mask16>() == 2);
        assert!(std::mem::size_of::<Mask32>() == 4);
    }
}
