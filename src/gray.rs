// gray.rs      Grayscale color model.
//
// Copyright (c) 2018-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::alpha::{
    self, AChannel, Mode as _, Opaque, Premultiplied, Straight, Translucent,
};
use crate::gamma::{self, Linear, Srgb};
use crate::{Ch16, Ch32, Ch8, Channel, ColorModel, Pixel};
use std::marker::PhantomData;
use std::ops::Mul;

/// Gray [color model], with optional [alpha channel].
///
/// The `Channel` ranges from *black* to *white*.
/// With [sRGB] gamma it is *luma*, but with [linear] gamma it is *relative
/// luminance*.
///
/// [alpha channel]: alpha/trait.AChannel.html
/// [color model]: trait.ColorModel.html
/// [linear]: gamma/struct.Linear.html
/// [sRGB]: gamma/struct.Srgb.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Gray<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    components: [C; 1],
    alpha: A,
    mode: PhantomData<M>,
    gamma: PhantomData<G>,
}

impl<C, A, M, G> Gray<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Create an [Opaque](alpha/struct.Opaque.html) gray value.
    pub fn new<H>(value: H) -> Self
    where
        C: From<H>,
        A: From<Opaque<C>>,
    {
        let components = [C::from(value)];
        let alpha = A::from(Opaque::default());
        let mode = PhantomData;
        let gamma = PhantomData;
        Gray {
            components,
            alpha,
            mode,
            gamma,
        }
    }
    /// Create a [Translucent](alpha/struct.Translucent.html) gray value.
    pub fn with_alpha<H>(value: H, alpha: H) -> Self
    where
        C: From<H>,
        A: From<H>,
    {
        let components = [C::from(value)];
        let alpha = A::from(alpha);
        let mode = PhantomData;
        let gamma = PhantomData;
        Gray {
            components,
            alpha,
            mode,
            gamma,
        }
    }
    /// Get the *luma* / *relative luminance* component.
    pub fn value(self) -> C {
        self.components[0]
    }
    /// Get the *alpha* value.
    pub fn alpha(self) -> C {
        self.alpha.value()
    }
}

impl<C, A, M, G> ColorModel for Gray<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Chan = C;

    /// Get all components affected by alpha/gamma
    fn components(&self) -> &[Self::Chan] {
        &self.components
    }

    /// Get the *alpha* component.
    fn alpha(self) -> Self::Chan {
        self.alpha.value()
    }

    /// Convert to *red*, *green*, *blue* and *alpha* components
    fn to_rgba(self) -> [Self::Chan; 4] {
        let value = self.value();
        [value, value, value, self.alpha()]
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self {
/*        const RED_COEF: f32 = 0.2126;
        const GREEN_COEF: f32 = 0.7152;
        const BLUE_COEF: f32 = 0.0722;

        let red = rgba[0] * RED_COEF.into();
        let green = rgba[1] * GREEN_COEF.into();
        let blue = rgba[2] * BLUE_COEF.into();
        let value = red + green + blue;*/
        let value = rgba[0].max(rgba[1]).max(rgba[2]); // FIXME
        let alpha = rgba[3];
        Gray::with_alpha(value, alpha)
    }

    /// Get channel-wise difference
    fn difference(self, rhs: Self) -> Self {
        let v = if self.value() > rhs.value() {
            self.value() - rhs.value()
        } else {
            rhs.value() - self.value()
        };
        let a = if self.alpha() > rhs.alpha() {
            self.alpha() - rhs.alpha()
        } else {
            rhs.alpha() - self.alpha()
        };
        Gray::with_alpha(v, a)
    }

    /// Check if all `Channel`s are within threshold
    fn within_threshold(self, rhs: Self) -> bool {
        self.value() <= rhs.value() && self.alpha() <= rhs.alpha()
    }
}

impl<C, A, M, G> Pixel for Gray<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Alpha = M;
    type Gamma = G;
}

impl<C, A, M, G> Iterator for Gray<C, A, M, G>
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
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: Gray<C, A, Straight, G>) -> Self {
        let value = Premultiplied::encode(c.value(), c.alpha());
        Gray::with_alpha(value, c.alpha())
    }
}

impl<C, A, G> From<Gray<C, A, Premultiplied, G>> for Gray<C, A, Straight, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: Gray<C, A, Premultiplied, G>) -> Self {
        let value = Premultiplied::decode(c.value(), c.alpha());
        Gray::with_alpha(value, c.alpha())
    }
}

impl<C, A, M, G> From<u8> for Gray<C, A, M, G>
where
    C: Channel + From<Ch8>,
    A: AChannel<Chan = C> + From<Opaque<C>>,
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
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self.components[0] = self.value() * rhs.value();
        self.alpha = self.alpha * rhs.alpha;
        self
    }
}

/// [Gray](struct.Gray.html) 8-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma
/// [pixel](trait.Pixel.html) format.
pub type Gray8 = Gray<Ch8, Opaque<Ch8>, Straight, Linear>;
/// [Gray](struct.Gray.html) 16-bit [opaque](alpha/struct.Opaque.html) (no
/// alpha) [linear](gamma/struct.Linear.html) gamma
/// [pixel](trait.Pixel.html) format.
pub type Gray16 = Gray<Ch16, Opaque<Ch16>, Straight, Linear>;
/// [Gray](struct.Gray.html) 32-bit [opaque](alpha/struct.Opaque.html) (no
/// alpha) [linear](gamma/struct.Linear.html) gamma
/// [pixel](trait.Pixel.html) format.
pub type Gray32 = Gray<Ch32, Opaque<Ch32>, Straight, Linear>;

type Graya<C, A> = Gray<C, A, Straight, Linear>;
/// [Gray](struct.Gray.html) 8-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Graya8 = Graya<Ch8, Translucent<Ch8>>;
/// [Gray](struct.Gray.html) 16-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Graya16 = Graya<Ch16, Translucent<Ch16>>;
/// [Gray](struct.Gray.html) 32-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Graya32 = Graya<Ch32, Translucent<Ch32>>;

type Grayap<C, A> = Gray<C, A, Premultiplied, Linear>;
/// [Gray](struct.Gray.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Graya8p = Grayap<Ch8, Translucent<Ch8>>;
/// [Gray](struct.Gray.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Graya16p = Grayap<Ch16, Translucent<Ch16>>;
/// [Gray](struct.Gray.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Graya32p = Grayap<Ch32, Translucent<Ch32>>;

type SGray<C, A> = Gray<C, A, Straight, Srgb>;
/// [Gray](struct.Gray.html) 8-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SGray8 = SGray<Ch8, Opaque<Ch8>>;
/// [Gray](struct.Gray.html) 16-bit [opaque](alpha/struct.Opaque.html) (no
/// alpha) [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html)
/// format.
pub type SGray16 = SGray<Ch16, Opaque<Ch16>>;
/// [Gray](struct.Gray.html) 32-bit [opaque](alpha/struct.Opaque.html) (no
/// alpha) [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html)
/// format.
pub type SGray32 = SGray<Ch32, Opaque<Ch32>>;

type SGraya<C, A> = Gray<C, A, Straight, Srgb>;
/// [Gray](struct.Gray.html) 8-bit [straight](alpha/struct.Straight.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SGraya8 = SGraya<Ch8, Translucent<Ch8>>;
/// [Gray](struct.Gray.html) 16-bit [straight](alpha/struct.Straight.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SGraya16 = SGraya<Ch16, Translucent<Ch16>>;
/// [Gray](struct.Gray.html) 32-bit [straight](alpha/struct.Straight.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SGraya32 = SGraya<Ch32, Translucent<Ch32>>;

type SGrayap<C, A> = Gray<C, A, Premultiplied, Srgb>;
/// [Gray](struct.Gray.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SGraya8p = SGrayap<Ch8, Translucent<Ch8>>;
/// [Gray](struct.Gray.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SGraya16p = SGrayap<Ch16, Translucent<Ch16>>;
/// [Gray](struct.Gray.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](trait.Pixel.html) format.
pub type SGraya32p = SGrayap<Ch32, Translucent<Ch32>>;

#[cfg(test)]
mod test {
    use super::super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<SGray8>(), 1);
        assert_eq!(std::mem::size_of::<SGray16>(), 2);
        assert_eq!(std::mem::size_of::<SGray32>(), 4);
        assert_eq!(std::mem::size_of::<SGraya8>(), 2);
        assert_eq!(std::mem::size_of::<SGraya16>(), 4);
        assert_eq!(std::mem::size_of::<SGraya32>(), 8);
    }
}
