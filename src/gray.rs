// gray.rs      Linear grayscale pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use crate::{Format, Rgb};
use crate::alpha::{Alpha, Opaque, Translucent};
use crate::channel::{Channel, Ch8, Ch16, Ch32};

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

impl<C, A> From<u8> for Gray<C, A>
    where C: Channel, C: From<Ch8>, A: Alpha, A: From<Opaque<C>>
{
    /// Convert from a u8 value.
    fn from(c: u8) -> Self {
        Gray::new(Ch8::new(c))
    }
}

impl<C, H, A, B> From<Rgb<H, B>> for Gray<C, A>
    where C: Channel, C: From<H>, H: Channel, A: Alpha, A: From<B>, B: Alpha
{
    /// Get a Gray from an Rgb
    fn from(c: Rgb<H, B>) -> Self {
        let red = C::from(c.red());
        let green = C::from(c.green());
        let blue = C::from(c.blue());
        let a = A::from(c.alpha());
        // FIXME: adjust luminance based on channels
        let v: C = red.max(green).max(blue);
        Gray::with_alpha(v, a)
    }
}

impl<C, H, A, B> From<Gray<H, B>> for Rgb<C, A>
    where C: Channel, C: From<H>, H: Channel, A: Alpha, A: From<B>, B: Alpha
{
    /// Get an Rgb from a Gray
    fn from(c: Gray<H, B>) -> Self {
        let v = C::from(c.value());
        let a = A::from(c.alpha());
        Rgb::with_alpha(v, v, v, a)
    }
}

impl<C: Channel, A: Alpha> Gray<C, A> {
    /// Create an opaque gray value.
    pub fn new<H>(value: H) -> Self
        where C: From<H>, A: From<Opaque<C>>
    {
        let value = C::from(value);
        let alpha = A::from(Opaque::default());
        Gray { value, alpha }
    }
    /// Create a gray value with alpha.
    pub fn with_alpha<H, B>(value: H, alpha: B) -> Self
        where C: From<H>, A: From<B>
    {
        let value = C::from(value);
        let alpha = A::from(alpha);
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

impl<C: Channel, A: Alpha> Format for Gray<C, A> {
    type Chan = C;
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
    use super::*;

    #[test]
    fn check_sizes() {
        assert!(std::mem::size_of::<Gray8>() == 1);
        assert!(std::mem::size_of::<Gray16>() == 2);
        assert!(std::mem::size_of::<Gray32>() == 4);
        assert!(std::mem::size_of::<GrayAlpha8>() == 2);
        assert!(std::mem::size_of::<GrayAlpha16>() == 4);
        assert!(std::mem::size_of::<GrayAlpha32>() == 8);
    }
}
