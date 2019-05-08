// mask.rs      Alpha mask pixel format.
//
// Copyright (c) 2019  Douglas P Lau
//
use crate::{Blend, Format, Rgb};
use crate::alpha::{Alpha, Translucent};
use crate::channel::{Channel, Ch8, Ch16, Ch32};
use std::marker::PhantomData;

/// [Translucent](struct.Translucent.html) alpha mask pixel
/// [Format](trait.Format.html).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Mask<C: Channel, A: Alpha<C>> {
    value: PhantomData<C>,
    alpha: A,
}

impl<C, A> From<u8> for Mask<C, A>
    where C: Channel, A: Alpha<C>, A: From<u8>
{
    /// Get a Mask from a u8
    fn from(c: u8) -> Self {
        let a = Into::<A>::into(c);
        Mask::new(a)
    }
}

impl<C, H, A, B> From<Rgb<H, B>> for Mask<C, A>
    where C: Channel, H: Channel, C: From<H>, A: From<B>, A: Alpha<C>,
          B: Alpha<H>
{
    /// Get a Mask from an Rgb
    fn from(c: Rgb<H, B>) -> Self {
        let a = Into::<A>::into(c.alpha());
        Mask::new(a)
    }
}

impl<C, H, A, B> From<Mask<H, B>> for Rgb<C, A>
    where C: Channel, H: Channel, C: From<H>, A: From<B>, A: Alpha<C>,
          B: Alpha<H>
{
    /// Get an Rgb from a Mask
    fn from(c: Mask<H, B>) -> Self {
        let v = C::MAX;
        let a = Into::<A>::into(c.alpha());
        Rgb::new(v, v, v, a)
    }
}

impl<C: Channel, A: Alpha<C>> Mask<C, A> {
    /// Create a new Mask value.
    pub fn new<V>(alpha: V) -> Self
        where A: From<V>
    {
        let value = PhantomData;
        let alpha = A::from(alpha);
        Mask { value, alpha }
    }
    /// Get the alpha value.
    pub fn alpha(self) -> A {
        self.alpha
    }
    /// Blend pixel on top of another, using "over".
    fn with_alpha_over(self, _dst: Mask<C, A>, _alpha: u8) -> Self {
        let alpha = self.alpha();
        Mask::new(alpha)
    }
}

impl<C: Channel, A: Alpha<C>> Format for Mask<C, A> { }

impl<C: Channel, A: Alpha<C>> Blend for Mask<C, A> {

    /// Blend pixels with an alpha mask (slow fallback).
    ///
    /// * `dst` Destination pixels.
    /// * `mask` Alpha mask for compositing.
    /// * `src` Source color.
    fn mask_over_fallback(dst: &mut [Self], mask: &[u8], src: Self) {
        for (bot, m) in dst.iter_mut().zip(mask) {
            *bot = src.with_alpha_over(*bot, *m);
        }
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
        assert!(std::mem::size_of::<Mask8>() == 1);
        assert!(std::mem::size_of::<Mask16>() == 2);
        assert!(std::mem::size_of::<Mask32>() == 4);
    }
}
