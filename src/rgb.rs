// rgb.rs       RGB color model.
//
// Copyright (c) 2018-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::alpha::{
    self, AChannel, Opaque, Premultiplied, Straight, Translucent,
};
use crate::gamma::{self, Linear};
use crate::{Ch16, Ch32, Ch8, Channel, ColorModel, Pixel};
use std::marker::PhantomData;

/// `RGB` additive [color model].
///
/// The components are *red*, *green* and *blue*, with optional *[alpha]*.
///
/// [alpha]: alpha/trait.AChannel.html
/// [color model]: trait.ColorModel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Rgb<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    red: C,
    green: C,
    blue: C,
    alpha: A,
    mode: PhantomData<M>,
    gamma: PhantomData<G>,
}

impl<C, A, M, G> Rgb<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Create an `Rgb` color.
    ///
    /// ## Example
    /// ```
    /// # use pix::*;
    /// let opaque_rgb = Rgb8::new(50, 255, 128, ());
    /// let translucent_rgb = Rgba8::new(100, 128, 255, 200);
    /// ```
    pub fn new<H, B>(red: H, green: H, blue: H, alpha: B) -> Self
    where
        C: From<H>,
        A: From<B>,
    {
        let red = C::from(red);
        let green = C::from(green);
        let blue = C::from(blue);
        let alpha = A::from(alpha);
        Rgb {
            red,
            green,
            blue,
            alpha,
            mode: PhantomData,
            gamma: PhantomData,
        }
    }

    /// Get the *red* component.
    pub fn red(self) -> C {
        self.red
    }

    /// Get the *green* component.
    pub fn green(self) -> C {
        self.green
    }

    /// Get the *blue* component.
    pub fn blue(self) -> C {
        self.blue
    }

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba(self) -> [C; 4] {
        [self.red(), self.green(), self.blue(), self.alpha()]
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba(rgba: [C; 4]) -> Self {
        let red = rgba[0];
        let green = rgba[1];
        let blue = rgba[2];
        let alpha = rgba[3];
        Rgb::new(red, green, blue, alpha)
    }

    /// Get channel-wise difference
    pub fn difference(self, rhs: Self) -> Self
    where
        A: From<C>,
    {
        let r = if self.red() > rhs.red() {
            self.red() - rhs.red()
        } else {
            rhs.red() - self.red()
        };
        let g = if self.green() > rhs.green() {
            self.green() - rhs.green()
        } else {
            rhs.green() - self.green()
        };
        let b = if self.blue() > rhs.blue() {
            self.blue() - rhs.blue()
        } else {
            rhs.blue() - self.blue()
        };
        let a = if self.alpha.value() > rhs.alpha.value() {
            self.alpha.value() - rhs.alpha.value()
        } else {
            rhs.alpha.value() - self.alpha.value()
        };
        Rgb::new(r, g, b, a)
    }

    /// Check if all `Channel`s are within threshold
    pub fn within_threshold(self, rhs: Self) -> bool {
        self.red() <= rhs.red()
            && self.green() <= rhs.green()
            && self.blue() <= rhs.blue()
            && self.alpha.value() <= rhs.alpha.value()
    }
}

impl<C, A, M, G> ColorModel for Rgb<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Chan = C;

    /// Get the *alpha* component
    fn alpha(self) -> Self::Chan {
        self.alpha.value()
    }

    /// Convert into channels shared by types
    fn into_channels<R: ColorModel>(self) -> ([C; 4], usize) {
        (self.into_rgba(), 3)
    }

    /// Convert from channels shared by types
    fn from_channels<R: ColorModel>(chan: [C; 4], alpha: usize) -> Self {
        debug_assert_eq!(alpha, 3);
        Self::from_rgba(chan)
    }
}

impl<C, A, M, G> Pixel for Rgb<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Alpha = M;
    type Gamma = G;
}

impl<C, A, M, G> Iterator for Rgb<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

impl<C, A, M, G> From<i32> for Rgb<C, A, M, G>
where
    C: Channel + From<Ch8>,
    A: AChannel<Chan = C> + From<C> + From<Translucent<Ch8>>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Get an `Rgb` from an `i32`
    fn from(c: i32) -> Self {
        let red = Ch8::from(c as u8);
        let green = Ch8::from((c >> 8) as u8);
        let blue = Ch8::from((c >> 16) as u8);
        let alpha = Ch8::from((c >> 24) as u8);
        Rgb::new(red, green, blue, Translucent::new(alpha))
    }
}

impl<C, A, M, G> From<Rgb<C, A, M, G>> for i32
where
    C: Channel,
    Ch8: From<C>,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Get an `i32` from an `Rgb`
    fn from(c: Rgb<C, A, M, G>) -> i32 {
        let red: u8 = Ch8::from(c.red()).into();
        let red = i32::from(red);
        let green: u8 = Ch8::from(c.green()).into();
        let green = i32::from(green) << 8;
        let blue: u8 = Ch8::from(c.blue()).into();
        let blue = i32::from(blue) << 16;
        let alpha: u8 = Ch8::from(c.alpha()).into();
        let alpha = i32::from(alpha) << 24;
        red | green | blue | alpha
    }
}

/// [Rgb](struct.Rgb.html) 8-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgb8 = Rgb<Ch8, Opaque<Ch8>, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 16-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgb16 = Rgb<Ch16, Opaque<Ch16>, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 32-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgb32 = Rgb<Ch32, Opaque<Ch32>, Straight, Linear>;

/// [Rgb](struct.Rgb.html) 8-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgba8 = Rgb<Ch8, Translucent<Ch8>, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 16-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgba16 = Rgb<Ch16, Translucent<Ch16>, Straight, Linear>;
/// [Rgb](struct.Rgb.html) 32-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgba32 = Rgb<Ch32, Translucent<Ch32>, Straight, Linear>;

/// [Rgb](struct.Rgb.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgba8p = Rgb<Ch8, Translucent<Ch8>, Premultiplied, Linear>;
/// [Rgb](struct.Rgb.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgba16p = Rgb<Ch16, Translucent<Ch16>, Premultiplied, Linear>;
/// [Rgb](struct.Rgb.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Rgba32p = Rgb<Ch32, Translucent<Ch32>, Premultiplied, Linear>;

type SRgb<C, A> = Rgb<C, A, Straight, gamma::Srgb>;
/// [Rgb](struct.Rgb.html) 8-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgb8 = SRgb<Ch8, Opaque<Ch8>>;
/// [Rgb](struct.Rgb.html) 16-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgb16 = SRgb<Ch16, Opaque<Ch16>>;
/// [Rgb](struct.Rgb.html) 32-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgb32 = SRgb<Ch32, Opaque<Ch32>>;

type SRgba<C, A> = Rgb<C, A, Straight, gamma::Srgb>;
/// [Rgb](struct.Rgb.html) 8-bit [straight](alpha/struct.Straight.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgba8 = SRgba<Ch8, Translucent<Ch8>>;
/// [Rgb](struct.Rgb.html) 16-bit [straight](alpha/struct.Straight.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgba16 = SRgba<Ch16, Translucent<Ch16>>;
/// [Rgb](struct.Rgb.html) 32-bit [straight](alpha/struct.Straight.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgba32 = SRgba<Ch32, Translucent<Ch32>>;

type SRgbap<C, A> = Rgb<C, A, Premultiplied, gamma::Srgb>;
/// [Rgb](struct.Rgb.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgba8p = SRgbap<Ch8, Translucent<Ch8>>;
/// [Rgb](struct.Rgb.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgba16p = SRgbap<Ch16, Translucent<Ch16>>;
/// [Rgb](struct.Rgb.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SRgba32p = SRgbap<Ch32, Translucent<Ch32>>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<SRgb8>(), 3);
        assert_eq!(std::mem::size_of::<SRgb16>(), 6);
        assert_eq!(std::mem::size_of::<SRgb32>(), 12);
        assert_eq!(std::mem::size_of::<SRgba8>(), 4);
        assert_eq!(std::mem::size_of::<SRgba16>(), 8);
        assert_eq!(std::mem::size_of::<SRgba32>(), 16);
    }
}
