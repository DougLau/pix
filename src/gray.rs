// gray.rs      Grayscale pixel format.
//
// Copyright (c) 2018-2020  Douglas P Lau
//
use crate::{
    Alpha, AlphaMode, AlphaModeID, AssociatedAlpha, Ch16, Ch32, Ch8, Channel,
    Format, GammaMode, GammaModeID, LinearGamma, Opaque, SeparatedAlpha,
    SrgbGamma, Translucent,
};
use std::marker::PhantomData;
use std::ops::Mul;

/// Gray pixel [Format](trait.Format.html), with optional
/// [Alpha](trait.Alpha.html) channel.
///
/// For types, see: [Gray8](type.Gray8.html), [Gray16](type.Gray16.html),
/// [Gray32](type.Gray32.html), [GrayAlpha8](type.GrayAlpha8.html),
/// [GrayAlpha16](type.GrayAlpha16.html), [GrayAlpha32](type.GrayAlpha32.html)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Gray<C: Channel, A: Alpha, M: AlphaMode, G: GammaMode> {
    mode: PhantomData<M>,
    gamma: PhantomData<G>,
    value: C,
    alpha: A,
}

impl<C: Channel, A: Alpha, M: AlphaMode, G: GammaMode> GammaMode
    for Gray<C, A, M, G>
{
    const ID: GammaModeID = G::ID;

    /// Encode one `Channel` using the gamma mode.
    fn encode<H: Channel>(h: H) -> H {
        G::encode(h)
    }
    /// Decode one `Channel` using the gamma mode.
    fn decode<H: Channel>(h: H) -> H {
        G::decode(h)
    }
}

impl<C: Channel, A: Alpha, M: AlphaMode, G: GammaMode> AlphaMode
    for Gray<C, A, M, G>
{
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

impl<C: Channel, A: Alpha, M: AlphaMode, G: GammaMode> Iterator
    for Gray<C, A, M, G>
{
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

impl<C, M: AlphaMode, G: GammaMode> From<Gray<C, Translucent<C>, M, G>>
    for Gray<C, Opaque<C>, M, G>
where
    C: Channel,
{
    fn from(c: Gray<C, Translucent<C>, M, G>) -> Self {
        Gray::new(c.value())
    }
}

impl<C, M: AlphaMode, G: GammaMode> From<Gray<C, Opaque<C>, M, G>>
    for Gray<C, Translucent<C>, M, G>
where
    C: Channel,
{
    fn from(c: Gray<C, Opaque<C>, M, G>) -> Self {
        Gray::with_alpha(c.value(), C::MAX)
    }
}

impl<C, A, M: AlphaMode, G: GammaMode> From<u8> for Gray<C, A, M, G>
where
    C: Channel,
    C: From<Ch8>,
    A: Alpha,
    A: From<Opaque<C>>,
{
    /// Convert from a `u8` value.
    fn from(c: u8) -> Self {
        Gray::new(Ch8::new(c))
    }
}

impl<C: Channel, A: Alpha, M: AlphaMode, G: GammaMode> Mul<Self>
    for Gray<C, A, M, G>
{
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self.value = self.value * rhs.value;
        self.alpha = self.alpha * rhs.alpha;
        self
    }
}

impl<C: Channel, A: Alpha, M: AlphaMode, G: GammaMode> Gray<C, A, M, G> {
    /// Create an [Opaque](struct.Opaque.html) gray value.
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
    /// Create a [Translucent](struct.Translucent.html) gray value.
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

impl<C, A, M: AlphaMode, G: GammaMode> Format for Gray<C, A, M, G>
where
    C: Channel,
    A: Alpha<Chan = C> + From<C>,
{
    type Chan = C;

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

/// [Opaque](struct.Opaque.html) 8-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type Gray8 = Gray<Ch8, Opaque<Ch8>, SeparatedAlpha, SrgbGamma>;

/// [Opaque](struct.Opaque.html) 16-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type Gray16 = Gray<Ch16, Opaque<Ch16>, SeparatedAlpha, SrgbGamma>;

/// [Opaque](struct.Opaque.html) 32-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type Gray32 = Gray<Ch32, Opaque<Ch32>, SeparatedAlpha, SrgbGamma>;

/// [Opaque](struct.Opaque.html) 8-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type LinearGray8 = Gray<Ch8, Opaque<Ch8>, SeparatedAlpha, LinearGamma>;

/// [Opaque](struct.Opaque.html) 16-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type LinearGray16 = Gray<Ch16, Opaque<Ch16>, SeparatedAlpha, LinearGamma>;

/// [Opaque](struct.Opaque.html) 32-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type LinearGray32 = Gray<Ch32, Opaque<Ch32>, SeparatedAlpha, LinearGamma>;

/// [Translucent](struct.Translucent.html) 8-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type GrayAlpha8 = Gray<Ch8, Translucent<Ch8>, SeparatedAlpha, SrgbGamma>;

/// [Translucent](struct.Translucent.html) 16-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type GrayAlpha16 = Gray<Ch16, Translucent<Ch16>, SeparatedAlpha, SrgbGamma>;

/// [Translucent](struct.Translucent.html) 32-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type GrayAlpha32 = Gray<Ch32, Translucent<Ch32>, SeparatedAlpha, SrgbGamma>;

/// [Translucent](struct.Translucent.html) 8-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type LinearGrayAlpha8 =
    Gray<Ch8, Translucent<Ch8>, SeparatedAlpha, LinearGamma>;

/// [Translucent](struct.Translucent.html) 16-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type LinearGrayAlpha16 =
    Gray<Ch16, Translucent<Ch16>, SeparatedAlpha, LinearGamma>;

/// [Translucent](struct.Translucent.html) 32-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type LinearGrayAlpha32 =
    Gray<Ch32, Translucent<Ch32>, SeparatedAlpha, LinearGamma>;

/// [Translucent](struct.Translucent.html) 8-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type PremulGrayAlpha8 =
    Gray<Ch8, Translucent<Ch8>, AssociatedAlpha, SrgbGamma>;

/// [Translucent](struct.Translucent.html) 16-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type PremulGrayAlpha16 =
    Gray<Ch16, Translucent<Ch16>, AssociatedAlpha, SrgbGamma>;

/// [Translucent](struct.Translucent.html) 32-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type PremulGrayAlpha32 =
    Gray<Ch32, Translucent<Ch32>, AssociatedAlpha, SrgbGamma>;

/// [Translucent](struct.Translucent.html) 8-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type PremulLinearGrayAlpha8 =
    Gray<Ch8, Translucent<Ch8>, AssociatedAlpha, LinearGamma>;

/// [Translucent](struct.Translucent.html) 16-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type PremulLinearGrayAlpha16 =
    Gray<Ch16, Translucent<Ch16>, AssociatedAlpha, LinearGamma>;

/// [Translucent](struct.Translucent.html) 32-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type PremulLinearGrayAlpha32 =
    Gray<Ch32, Translucent<Ch32>, AssociatedAlpha, LinearGamma>;

#[cfg(test)]
mod test {
    use super::super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<Gray8>(), 1);
        assert_eq!(std::mem::size_of::<Gray16>(), 2);
        assert_eq!(std::mem::size_of::<Gray32>(), 4);
        assert_eq!(std::mem::size_of::<GrayAlpha8>(), 2);
        assert_eq!(std::mem::size_of::<GrayAlpha16>(), 4);
        assert_eq!(std::mem::size_of::<GrayAlpha32>(), 8);
    }
}
