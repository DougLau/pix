// gray.rs      Grayscale pixel format.
//
// Copyright (c) 2018-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::alpha::{
    self, Alpha, Mode as _, Opaque, Premultiplied, Straight, Translucent,
};
use crate::gamma::{self, Linear, Srgb};
use crate::{Ch16, Ch32, Ch8, Channel, Format};
use std::marker::PhantomData;
use std::ops::Mul;

/// Gray color model, with optional [Alpha](alpha/trait.Alpha.html) channel.
///
/// The `Channel` is grayscale, from *black* to *white*.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Gray<C: Channel, A: Alpha, M: alpha::Mode, G: gamma::Mode> {
    mode: PhantomData<M>,
    gamma: PhantomData<G>,
    value: C,
    alpha: A,
}

impl<C, A, M, G> Iterator for Gray<C, A, M, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

impl<C, M, G> From<Gray<C, Translucent<C>, M, G>> for Gray<C, Opaque<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Gray<C, Translucent<C>, M, G>) -> Self {
        Gray::new(c.value())
    }
}

impl<C, M, G> From<Gray<C, Opaque<C>, M, G>> for Gray<C, Translucent<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Gray<C, Opaque<C>, M, G>) -> Self {
        Gray::with_alpha(c.value(), C::MAX)
    }
}

impl<C, A, G> From<Gray<C, A, Straight, G>> for Gray<C, A, Premultiplied, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
    G: gamma::Mode,
{
    fn from(c: Gray<C, A, Straight, G>) -> Self {
        let value = Premultiplied::encode::<C, A>(c.value, c.alpha);
        Gray::with_alpha(value, c.alpha)
    }
}

impl<C, A, G> From<Gray<C, A, Premultiplied, G>> for Gray<C, A, Straight, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
    G: gamma::Mode,
{
    fn from(c: Gray<C, A, Premultiplied, G>) -> Self {
        let value = Premultiplied::decode::<C, A>(c.value, c.alpha);
        Gray::with_alpha(value, c.alpha)
    }
}

impl<C, A, M, G> From<u8> for Gray<C, A, M, G>
where
    C: Channel + From<Ch8>,
    A: Alpha<Chan = C> + From<Opaque<C>>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Convert from a `u8` value.
    fn from(c: u8) -> Self {
        Gray::new(Ch8::new(c))
    }
}

impl<C, A, M, G> Mul<Self> for Gray<C, A, M, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self.value = self.value * rhs.value;
        self.alpha = self.alpha * rhs.alpha;
        self
    }
}

impl<C, A, M, G> Gray<C, A, M, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Create an [Opaque](alpha/struct.Opaque.html) gray value.
    pub fn new<H>(value: H) -> Self
    where
        C: From<H>,
        A: From<Opaque<C>>,
    {
        let value = C::from(value);
        let alpha = A::from(Opaque::default());
        let gamma = PhantomData;
        let mode = PhantomData;
        Gray {
            value,
            alpha,
            gamma,
            mode,
        }
    }
    /// Create a [Translucent](alpha/struct.Translucent.html) gray value.
    pub fn with_alpha<H, B>(value: H, alpha: B) -> Self
    where
        C: From<H>,
        A: From<B>,
    {
        let value = C::from(value);
        let alpha = A::from(alpha);
        let gamma = PhantomData;
        let mode = PhantomData;
        Gray {
            value,
            alpha,
            gamma,
            mode,
        }
    }
    /// Get the gray value.
    pub fn value(self) -> C {
        self.value
    }
    /// Get the alpha value.
    pub fn alpha(self) -> A {
        self.alpha
    }
}

impl<C, A, M, G> Format for Gray<C, A, M, G>
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
        [self.value, self.value, self.value, self.alpha.value()]
    }

    /// Make a pixel with given RGBA `Channel`s
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self {
        let value = rgba[0].max(rgba[1]).max(rgba[2]); // FIXME
        let alpha = rgba[3];
        Gray::with_alpha(value, alpha)
    }

    /// Get channel-wise difference
    fn difference(self, rhs: Self) -> Self {
        let v = if self.value > rhs.value {
            self.value - rhs.value
        } else {
            rhs.value - self.value
        };
        let a = if self.alpha.value() > rhs.alpha.value() {
            self.alpha.value() - rhs.alpha.value()
        } else {
            rhs.alpha.value() - self.alpha.value()
        };
        Gray::with_alpha(v, a)
    }

    /// Check if all `Channel`s are within threshold
    fn within_threshold(self, rhs: Self) -> bool {
        self.value <= rhs.value && self.alpha.value() <= rhs.alpha.value()
    }
}

/// [Gray](struct.Gray.html) 8-bit [opaque](alpha/struct.Opaque.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type Gray8 = Gray<Ch8, Opaque<Ch8>, Straight, Linear>;
/// [Gray](struct.Gray.html) 16-bit [opaque](alpha/struct.Opaque.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type Gray16 = Gray<Ch16, Opaque<Ch16>, Straight, Linear>;
/// [Gray](struct.Gray.html) 32-bit [opaque](alpha/struct.Opaque.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type Gray32 = Gray<Ch32, Opaque<Ch32>, Straight, Linear>;

type GrayAlpha<C, A> = Gray<C, A, Straight, Linear>;
/// [Gray](struct.Gray.html) 8-bit
/// [straight alpha](alpha/struct.Straight.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type GrayAlpha8 = GrayAlpha<Ch8, Translucent<Ch8>>;
/// [Gray](struct.Gray.html) 16-bit
/// [straight alpha](alpha/struct.Straight.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type GrayAlpha16 = GrayAlpha<Ch16, Translucent<Ch16>>;
/// [Gray](struct.Gray.html) 32-bit
/// [straight alpha](alpha/struct.Straight.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type GrayAlpha32 = GrayAlpha<Ch32, Translucent<Ch32>>;

type GrayAlphap<C, A> = Gray<C, A, Premultiplied, Linear>;
/// [Gray](struct.Gray.html) 8-bit
/// [premultiplied alpha](alpha/struct.Premultiplied.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type GrayAlpha8p = GrayAlphap<Ch8, Translucent<Ch8>>;
/// [Gray](struct.Gray.html) 16-bit
/// [premultiplied alpha](alpha/struct.Premultiplied.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type GrayAlpha16p = GrayAlphap<Ch16, Translucent<Ch16>>;
/// [Gray](struct.Gray.html) 32-bit
/// [premultiplied alpha](alpha/struct.Premultiplied.html)
/// [linear gamma](gamma/struct.Linear.html)
/// pixel [format](trait.Format.html).
pub type GrayAlpha32p = GrayAlphap<Ch32, Translucent<Ch32>>;

type SGray<C, A> = Gray<C, A, Straight, Srgb>;
/// [Gray](struct.Gray.html) 8-bit [opaque](alpha/struct.Opaque.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SGray8 = SGray<Ch8, Opaque<Ch8>>;
/// [Gray](struct.Gray.html) 16-bit [opaque](alpha/struct.Opaque.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SGray16 = SGray<Ch16, Opaque<Ch16>>;
/// [Gray](struct.Gray.html) 32-bit [opaque](alpha/struct.Opaque.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SGray32 = SGray<Ch32, Opaque<Ch32>>;

type SGrayAlpha<C, A> = Gray<C, A, Straight, Srgb>;
/// [Gray](struct.Gray.html) 8-bit
/// [straight alpha](alpha/struct.Straight.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SGrayAlpha8 = SGrayAlpha<Ch8, Translucent<Ch8>>;
/// [Gray](struct.Gray.html) 16-bit
/// [straight alpha](alpha/struct.Straight.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SGrayAlpha16 = SGrayAlpha<Ch16, Translucent<Ch16>>;
/// [Gray](struct.Gray.html) 32-bit
/// [straight alpha](alpha/struct.Straight.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SGrayAlpha32 = SGrayAlpha<Ch32, Translucent<Ch32>>;

type SGrayAlphap<C, A> = Gray<C, A, Premultiplied, Srgb>;
/// [Gray](struct.Gray.html) 8-bit
/// [premultiplied alpha](alpha/struct.Premultiplied.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SGrayAlpha8p = SGrayAlphap<Ch8, Translucent<Ch8>>;
/// [Gray](struct.Gray.html) 16-bit
/// [premultiplied alpha](alpha/struct.Premultiplied.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SGrayAlpha16p = SGrayAlphap<Ch16, Translucent<Ch16>>;
/// [Gray](struct.Gray.html) 32-bit
/// [premultiplied alpha](alpha/struct.Premultiplied.html)
/// [sRGB gamma](gamma/struct.Srgb.html) pixel [format](trait.Format.html).
pub type SGrayAlpha32p = SGrayAlphap<Ch32, Translucent<Ch32>>;

#[cfg(test)]
mod test {
    use super::super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<SGray8>(), 1);
        assert_eq!(std::mem::size_of::<SGray16>(), 2);
        assert_eq!(std::mem::size_of::<SGray32>(), 4);
        assert_eq!(std::mem::size_of::<SGrayAlpha8>(), 2);
        assert_eq!(std::mem::size_of::<SGrayAlpha16>(), 4);
        assert_eq!(std::mem::size_of::<SGrayAlpha32>(), 8);
    }
}
