// gray.rs      Grayscale pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use crate::{
    Alpha, Channel, Ch8, Ch16, Ch32, Format, Opaque, PixModes, Translucent,
};

/// Gray pixel [Format](trait.Format.html), with optional
/// [Alpha](trait.Alpha.html) channel.
///
/// For types, see: [Gray8](type.Gray8.html), [Gray16](type.Gray16.html),
/// [Gray32](type.Gray32.html), [GrayAlpha8](type.GrayAlpha8.html),
/// [GrayAlpha16](type.GrayAlpha16.html), [GrayAlpha32](type.GrayAlpha32.html)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Gray<C: Channel, A: Alpha> {
    value: C,
    alpha: A,
}

impl<C: Channel, A: Alpha> PixModes for Gray<C, A> { }

impl<C: Channel, A: Alpha> Iterator for Gray<C, A> {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

impl<C, A> From<u8> for Gray<C, A>
    where C: Channel, C: From<Ch8>, A: Alpha, A: From<Opaque<C>>
{
    /// Convert from a `u8` value.
    fn from(c: u8) -> Self {
        Gray::new(Ch8::new(c))
    }
}

impl<C: Channel, A: Alpha> Gray<C, A> {
    /// Create an [Opaque](struct.Opaque.html) gray value.
    pub fn new<H>(value: H) -> Self
        where C: From<H>, A: From<Opaque<C>>
    {
        let value = C::from(value);
        let alpha = A::from(Opaque::default());
        Gray { value, alpha }
    }
    /// Create a [Translucent](struct.Translucent.html) gray value.
    pub fn with_alpha<H>(value: H, alpha: H) -> Self
        where C: From<H>, A: From<C>
    {
        let value = C::from(value);
        let alpha = A::from(C::from(alpha));
        Gray { value, alpha }
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

impl<C, A> Format for Gray<C, A>
    where C: Channel, A: Alpha<Chan=C> + From<C>
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
        let v = if self.value > rhs.value { self.value - rhs.value }
                else { rhs.value - self.value };
        let a = if self.alpha.value() > rhs.alpha.value() {
            self.alpha.value() - rhs.alpha.value()
        } else {
            rhs.alpha.value() - self.alpha.value()
        };
        Gray::with_alpha(v, a)
    }

    /// Check if all `Channel`s are within threshold
    fn within_threshold(self, rhs: Self) -> bool {
        self.value <= rhs.value &&
        self.alpha.value() <= rhs.alpha.value()
    }
}

/// [Opaque](struct.Opaque.html) 8-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type Gray8 = Gray<Ch8, Opaque<Ch8>>;

/// [Opaque](struct.Opaque.html) 16-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type Gray16 = Gray<Ch16, Opaque<Ch16>>;

/// [Opaque](struct.Opaque.html) 32-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type Gray32 = Gray<Ch32, Opaque<Ch32>>;

/// [Translucent](struct.Translucent.html) 8-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type GrayAlpha8 = Gray<Ch8, Translucent<Ch8>>;

/// [Translucent](struct.Translucent.html) 16-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type GrayAlpha16 = Gray<Ch16, Translucent<Ch16>>;

/// [Translucent](struct.Translucent.html) 32-bit [Gray](struct.Gray.html) pixel
/// [Format](trait.Format.html).
pub type GrayAlpha32 = Gray<Ch32, Translucent<Ch32>>;

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
