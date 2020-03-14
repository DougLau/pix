// hsv.rs       HSV color model.
//
// Copyright (c) 2018-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::alpha::{
    self, AChannel, Mode as _, Opaque, Premultiplied, Straight, Translucent,
};
use crate::gamma::{self, Linear};
use crate::{Ch16, Ch32, Ch8, Channel, ColorModel, Pixel};
use std::marker::PhantomData;
use std::ops::Mul;

/// HSV hexcone [color model].
///
/// The components are *hue*, *saturation* and *value*, with optional *[alpha]*.
///
/// [alpha]: alpha/trait.AChannel.html
/// [channel]: trait.Channel.html
/// [color model]: trait.ColorModel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    components: [C; 3],
    alpha: A,
    mode: PhantomData<M>,
    gamma: PhantomData<G>,
}

impl<C, A, M, G> ColorModel for Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Chan = C;

    /// Get all non-alpha components
    fn components(&self) -> &[Self::Chan] {
        &self.components
    }

    /// Get the *alpha* component
    fn alpha(self) -> Self::Chan {
        self.alpha.value()
    }

    /// Convert to *red*, *green*, *blue* and *alpha* components
    fn to_rgba(self) -> [Self::Chan; 4] {
        // Convert HSV to Linear RGBA
        // FIXME
        /*let fh = self.hue() * 6.0; // Get range 0-6
        let h = fh as i8; // int 0-6
        let s = self.saturation();
        let v = self.value();
        let alpha = self.alpha();

        if s == 0 {
            // if saturation is 0, then it's gray
            return [v, v, v, alpha];
        }

        let f = fh - (h as f32); // get difference from rounding to 0-6
        let p = v * (1.0 - s);
        let q = v * (1.0 - s * f);
        let t = v * (1.0 - s * (1.0 - f));

        let (red, green, blue) = match h {
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            5 => (v, p, q),
            _ => (v, t, p), // 0 or 6
        };

        [red, green, blue, alpha]*/
        todo!()
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self {
        // Linear RGBA to HSV
        // FIXME
        /*let red = rgba[0];
        let green = rgba[1];
        let blue = rgba[2];
        let alpha = rgba[3];

        let max = red.max(green).max(blue);
        let min = red.min(green).min(blue);
        let delta = max - min;

        let hue = if delta != 0.0 {
            let mut hue = if red == max {
                (green - blue) / delta
            } else if green == max {
                2.0 + (blue - red) / delta
            } else {
                4.0 + (red - green) / delta
            };

            if hue < 0.0 {
                hue += 6.0;
            }
            hue / 6.0
        } else {
            0.0
        };

        let sat = if max == 0.0 { 0.0 } else { (max - min) / max };
        let val = max;

        Hsv::with_alpha(hue, sat, val, alpha)*/
        todo!()
    }

    /// Get channel-wise difference
    fn difference(self, rhs: Self) -> Self {
        // FIXME: Hue circles around, use nearest middle point
        let h = if self.hue() > rhs.hue() {
            self.hue() - rhs.hue()
        } else {
            rhs.hue() - self.hue()
        };

        let s = if self.saturation() > rhs.saturation() {
            self.saturation() - rhs.saturation()
        } else {
            rhs.saturation() - self.saturation()
        };
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
        Hsv::with_alpha(h, s, v, a)
    }

    // FIXME
    /// Check if all `Channel`s are within threshold
    fn within_threshold(self, rhs: Self) -> bool {
        todo!()
        /*self.red() <= rhs.red()
            && self.green() <= rhs.green()
            && self.blue() <= rhs.blue()
            && self.alpha() <= rhs.alpha()*/
    }
}

impl<C, A, M, G> Pixel for Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Alpha = M;
    type Gamma = G;
}

impl<C, A, M, G> Iterator for Hsv<C, A, M, G>
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

impl<C, M, G> From<Hsv<C, Translucent<C>, M, G>> for Hsv<C, Opaque<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Hsv<C, Translucent<C>, M, G>) -> Self {
        Hsv::new(c.hue(), c.saturation(), c.value())
    }
}

impl<C, M, G> From<Hsv<C, Opaque<C>, M, G>> for Hsv<C, Translucent<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: Hsv<C, Opaque<C>, M, G>) -> Self {
        Hsv::with_alpha(c.hue(), c.saturation(), c.value(), C::MAX)
    }
}

impl<C, A, G> From<Hsv<C, A, Straight, G>> for Hsv<C, A, Premultiplied, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: Hsv<C, A, Straight, G>) -> Self {
        let hue = c.hue();
        // FIXME: Saturation encoded or not?
        let saturation = c.saturation();
        let value = Premultiplied::encode(c.value(), c.alpha());
        Hsv::with_alpha(hue, saturation, value, c.alpha())
    }
}

impl<C, A, G> From<Hsv<C, A, Premultiplied, G>> for Hsv<C, A, Straight, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: Hsv<C, A, Premultiplied, G>) -> Self {
        let hue = c.hue();
        // FIXME: Saturation decoded or not?
        let saturation = c.saturation();
        let value = Premultiplied::decode(c.value(), c.alpha());
        Hsv::with_alpha(hue, saturation, value, c.alpha)
    }
}

impl<C, A, M, G> From<i32> for Hsv<C, A, M, G>
where
    C: Channel + From<Ch8>,
    A: AChannel<Chan = C> + From<Translucent<Ch8>>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Get an `Hsv` from an `i32`
    fn from(c: i32) -> Self {
        let hue = Ch8::from(c as u8);
        let saturation = Ch8::from((c >> 8) as u8);
        let value = Ch8::from((c >> 16) as u8);
        let alpha = Ch8::from((c >> 24) as u8);
        Hsv::with_alpha(hue, saturation, value, Translucent::new(alpha))
    }
}

impl<C, A, M, G> From<Hsv<C, A, M, G>> for i32
where
    C: Channel,
    Ch8: From<C>,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Get an `i32` from an `Hsv`
    fn from(c: Hsv<C, A, M, G>) -> i32 {
        let hue: u8 = Ch8::from(c.hue()).into();
        let hue = i32::from(hue);
        let saturation: u8 = Ch8::from(c.saturation()).into();
        let saturation = i32::from(saturation) << 8;
        let value: u8 = Ch8::from(c.value()).into();
        let value = i32::from(value) << 16;
        let alpha: u8 = Ch8::from(c.alpha()).into();
        let alpha = i32::from(alpha) << 24;
        hue | saturation | value | alpha
    }
}

// FIXME
/*impl<C, A, G> Mul<Self> for Hsv<C, A, Straight, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    G: gamma::Mode,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let red = self.red() * rhs.red();
        let green = self.green() * rhs.green();
        let blue = self.blue() * rhs.blue();
        let components = [red, green, blue];
        let alpha = self.alpha * rhs.alpha;
        Hsv {
            components,
            alpha,
            mode: PhantomData,
            gamma: PhantomData,
        }
    }
}*/

// FIXME
/*impl<C, A, G> Mul<Self> for Hsv<C, A, Premultiplied, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let this: Hsv<C, A, Straight, G> = self.into();
        let other: Hsv<C, A, Straight, G> = rhs.into();

        (this * other).into()
    }
}*/

impl<C, A, M, G> Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Create an [Opaque](alpha/struct.Opaque.html) color by specifying *hue*,
    /// *saturation* and *value* values.
    pub fn new<H>(hue: H, saturation: H, value: H) -> Self
    where
        C: From<H>,
        A: From<Opaque<C>>,
    {
        Self::with_alpha(hue, saturation, value, Opaque::default())
    }
    /// Create a [Translucent](alpha/struct.Translucent.html) color by
    /// specifying *hue*, *saturation*, *value* and *alpha* values.
    pub fn with_alpha<H, B>(hue: H, saturation: H, value: H, alpha: B) -> Self
    where
        C: From<H>,
        A: From<B>,
    {
        let hue = C::from(hue);
        let saturation = C::from(saturation);
        let value = C::from(value);
        let components = [hue, saturation, value];
        let alpha = A::from(alpha);
        Hsv {
            components,
            alpha,
            mode: PhantomData,
            gamma: PhantomData,
        }
    }
    /// Get the hue component.
    pub fn hue(self) -> C {
        self.components[0]
    }
    /// Get the saturation component.
    pub fn saturation(self) -> C {
        self.components[1]
    }
    /// Get the value component.
    pub fn value(self) -> C {
        self.components[2]
    }
}

/// [Hsv](struct.Hsv.html) 8-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsv8 = Hsv<Ch8, Opaque<Ch8>, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 16-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsv16 = Hsv<Ch16, Opaque<Ch16>, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 32-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsv32 = Hsv<Ch32, Opaque<Ch32>, Straight, Linear>;

/// [Hsv](struct.Hsv.html) 8-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva8 = Hsv<Ch8, Translucent<Ch8>, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 16-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva16 = Hsv<Ch16, Translucent<Ch16>, Straight, Linear>;
/// [Hsv](struct.Hsv.html) 32-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva32 = Hsv<Ch32, Translucent<Ch32>, Straight, Linear>;

/// [Hsv](struct.Hsv.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva8p = Hsv<Ch8, Translucent<Ch8>, Premultiplied, Linear>;
/// [Hsv](struct.Hsv.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva16p = Hsv<Ch16, Translucent<Ch16>, Premultiplied, Linear>;
/// [Hsv](struct.Hsv.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type Hsva32p = Hsv<Ch32, Translucent<Ch32>, Premultiplied, Linear>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<Hsv8>(), 3);
        assert_eq!(std::mem::size_of::<Hsv16>(), 6);
        assert_eq!(std::mem::size_of::<Hsv32>(), 12);
        assert_eq!(std::mem::size_of::<Hsva8>(), 4);
        assert_eq!(std::mem::size_of::<Hsva16>(), 8);
        assert_eq!(std::mem::size_of::<Hsva32>(), 16);
    }

    #[test]
    fn check_mul() {
        // FIXME
        /*let a = SRgba8::with_alpha(0xFF, 0xFF, 0xFF, 0xFF);
        let b = SRgba8::with_alpha(0x00, 0x00, 0x00, 0x00);
        assert_eq!(a * b, b);

        let a = SRgba8::with_alpha(0xFF, 0xFF, 0xFF, 0xFF);
        let b = SRgba8::with_alpha(0x80, 0x80, 0x80, 0x80);
        assert_eq!(a * b, b);

        let a = SRgba8::with_alpha(0xFF, 0xF0, 0x00, 0x70);
        let b = SRgba8::with_alpha(0x80, 0x00, 0x60, 0xFF);
        assert_eq!(a * b, SRgba8::with_alpha(0x80, 0x00, 0x00, 0x70));

        let a = SRgba8::with_alpha(0xFF, 0x00, 0x80, 0xFF);
        let b = SRgba8::with_alpha(0xFF, 0xFF, 0xFF, 0x10);
        assert_eq!(a * b, SRgba8::with_alpha(0xFF, 0x00, 0x80, 0x10));*/
    }
}
