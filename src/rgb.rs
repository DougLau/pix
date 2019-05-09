// rgb.rs       Linear RGB pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use crate::Format;
use crate::alpha::{Alpha, Opaque, Translucent};
use crate::channel::{Channel, Ch8, Ch16, Ch32};

/// RGB pixel [Format](trait.Format.html), with optional
/// [Alpha](trait.Alpha.html) channel.
///
/// The channels are *red*, *green* and *blue*.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Rgb<C: Channel, A: Alpha<C>> {
    red: C,
    green: C,
    blue: C,
    alpha: A,
}

impl<C, A> From<Rgb<C, A>> for i32
    where C: Channel, Ch8: From<C>, A: Alpha<C>
{
    /// Get an i32 from an Rgb
    fn from(c: Rgb<C, A>) -> i32 {
        let red: u8 = Ch8::from(c.red()).into();
        let red = i32::from(red);
        let green: u8 = Ch8::from(c.green()).into();
        let green = i32::from(green) << 8;
        let blue: u8 = Ch8::from(c.blue()).into();
        let blue = i32::from(blue) << 16;
        let alpha: u8 = Ch8::from(c.alpha().value()).into();
        let alpha = i32::from(alpha) << 24;
        red | green | blue | alpha
    }
}

impl<C, H> From<Rgb<H, Translucent<H>>> for Rgb<C, Opaque<C>>
    where C: Channel, C: From<H>, H: Channel
{
    /// Get an Opaque Rgb from a Translucent Rgb
    fn from(c: Rgb<H, Translucent<H>>) -> Self {
        let red = C::from(c.red());
        let green = C::from(c.green());
        let blue = C::from(c.blue());
        let a = C::from(c.alpha().value());
        Rgb::new(red / a, green / a, blue / a)
    }
}

impl<C, H> From<Rgb<H, Opaque<H>>> for Rgb<C, Translucent<C>>
    where C: Channel, C: From<H>, H: Channel, Translucent<C>: From<Opaque<H>>
{
    /// Get a Translucent Rgb from an Opaque Rgb
    fn from(c: Rgb<H, Opaque<H>>) -> Self {
        let red = C::from(c.red());
        let green = C::from(c.green());
        let blue = C::from(c.blue());
        let alpha = Translucent::<C>::from(c.alpha());
        Rgb::with_alpha(red, green, blue, alpha)
    }
}

impl<C: Channel, A: Alpha<C>> Rgb<C, A> {
    /// Build a color by specifying red, green and blue values.
    pub fn new<H>(red: H, green: H, blue: H) -> Self
        where C: From<H>, A: From<Opaque<C>>
    {
        let red = C::from(red);
        let green = C::from(green);
        let blue = C::from(blue);
        let alpha = A::from(Opaque::default());
        Rgb { red, green, blue, alpha }
    }
    /// Create a color by specifying red, green, blue and alpha values.
    pub fn with_alpha<H, B>(red: H, green: H, blue: H, alpha: B) -> Self
        where C: From<H>, A: From<B>
    {
        let red = C::from(red);
        let green = C::from(green);
        let blue = C::from(blue);
        let alpha = A::from(alpha);
        // FIXME: do alpha properly and premultiply if necessary
        Rgb { red, green, blue, alpha }
    }
    /// Get the red channel.
    pub fn red(self) -> C {
        self.red
    }
    /// Get the green channel.
    pub fn green(self) -> C {
        self.green
    }
    /// Get the blue channel.
    pub fn blue(self) -> C {
        self.blue
    }
    /// Get the alpha channel.
    pub fn alpha(self) -> A {
        self.alpha
    }
}

impl<C: Channel, A: Alpha<C>> Format for Rgb<C, A> { }

/// [Opaque](struct.Opaque.html) 8-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgb8 = Rgb<Ch8, Opaque<Ch8>>;

/// [Opaque](struct.Opaque.html) 16-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgb16 = Rgb<Ch16, Opaque<Ch16>>;

/// [Opaque](struct.Opaque.html) 32-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgb32 = Rgb<Ch32, Opaque<Ch32>>;

/// [Translucent](struct.Translucent.html) 8-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgba8 = Rgb<Ch8, Translucent<Ch8>>;

/// [Translucent](struct.Translucent.html) 16-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgba16 = Rgb<Ch16, Translucent<Ch16>>;

/// [Translucent](struct.Translucent.html) 32-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgba32 = Rgb<Ch32, Translucent<Ch32>>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sizes() {
        assert!(std::mem::size_of::<Rgb8>() == 3);
        assert!(std::mem::size_of::<Rgb16>() == 6);
        assert!(std::mem::size_of::<Rgb32>() == 12);
        assert!(std::mem::size_of::<Rgba8>() == 4);
        assert!(std::mem::size_of::<Rgba16>() == 8);
        assert!(std::mem::size_of::<Rgba32>() == 16);
    }
}
