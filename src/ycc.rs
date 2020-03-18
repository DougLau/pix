// ycc.rs       YCbCr color model.
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

/// YCbCr (ITU601 / ITU-T 709 / ITU-T T.871) [color model].
///
/// Video cameras and JPEGS use this format.
///
/// The components are *y*, *cb* and *cr*, with optional *[alpha]*.
///
/// [alpha]: alpha/trait.AChannel.html
/// [channel]: trait.Channel.html
/// [color model]: trait.ColorModel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct YCbCr<C, A, M, G>
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

impl<C, A, M, G> ColorModel for YCbCr<C, A, M, G>
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

    /// Get the *alpha* component
    fn alpha(self) -> Self::Chan {
        self.alpha.value()
    }

    /// Convert to *red*, *green*, *blue* and *alpha* components
    fn to_rgba(self) -> [Self::Chan; 4] {
        // Convert YCbCr to RGBA
        // FIXME
        /*let y = self.y();
        let cb = self.cb();
        let cr = self.cr();

        let r = y + 1.402 * (cr - 0.5);
        let g = y - 0.344136 * (cb - 0.5) - 0.714136 * (cr - 0.5);
        let b = y + 1.772 * (cb - 0.5);
        [r as u8, g as u8, b as u8, 255]*/
        todo!()
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self {
        // Convert RGBA to YCbCr
        // FIXME
        /*let red = rgba[0];
        let green = rgba[1];
        let blue = rgba[2];
        let alpha = rgba[3];

        let y = (0.299 * red) + (0.587 * green) + (0.114 * blue);
        let cb = 0.5 - (0.168736 * red) - (0.331264 * green) + (0.5 * blue);
        let cr = 0.5 + (0.5 * red) - (0.418688 * green) - (0.081312 * blue);

        YCbCr::new(y, cb, cr, alpha)*/
        todo!()
    }
}

impl<C, A, M, G> Pixel for YCbCr<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    type Alpha = M;
    type Gamma = G;
}

impl<C, A, M, G> Iterator for YCbCr<C, A, M, G>
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

impl<C, M, G> From<YCbCr<C, Translucent<C>, M, G>> for YCbCr<C, Opaque<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: YCbCr<C, Translucent<C>, M, G>) -> Self {
        YCbCr::new(c.y(), c.cb(), c.cr())
    }
}

impl<C, M, G> From<YCbCr<C, Opaque<C>, M, G>> for YCbCr<C, Translucent<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: YCbCr<C, Opaque<C>, M, G>) -> Self {
        YCbCr::with_alpha(c.y(), c.cb(), c.cr(), C::MAX)
    }
}

impl<C, A, G> From<YCbCr<C, A, Straight, G>> for YCbCr<C, A, Premultiplied, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: YCbCr<C, A, Straight, G>) -> Self {
        let y = Premultiplied::encode(c.y(), c.alpha());
        let cb = c.cb();
        let cr = c.cr();
        YCbCr::with_alpha(y, cb, cr, c.alpha())
    }
}

impl<C, A, G> From<YCbCr<C, A, Premultiplied, G>> for YCbCr<C, A, Straight, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    fn from(c: YCbCr<C, A, Premultiplied, G>) -> Self {
        let y = Premultiplied::decode(c.y(), c.alpha());
        let cb = c.cb();
        let cr = c.cr();
        YCbCr::with_alpha(y, cb, cr, c.alpha())
    }
}

impl<C, A, M, G> From<i32> for YCbCr<C, A, M, G>
where
    C: Channel + From<Ch8>,
    A: AChannel<Chan = C> + From<Translucent<Ch8>>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Get an `YCbCr` from an `i32`
    fn from(c: i32) -> Self {
        let y = Ch8::from(c as u8);
        let cb = Ch8::from((c >> 8) as u8);
        let cr = Ch8::from((c >> 16) as u8);
        let alpha = Ch8::from((c >> 24) as u8);
        YCbCr::with_alpha(y, cb, cr, Translucent::new(alpha))
    }
}

impl<C, A, M, G> From<YCbCr<C, A, M, G>> for i32
where
    C: Channel,
    Ch8: From<C>,
    A: AChannel<Chan = C> + From<C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Get an `i32` from an `YCbCr`
    fn from(c: YCbCr<C, A, M, G>) -> i32 {
        let y: u8 = Ch8::from(c.y()).into();
        let y = i32::from(y);
        let cb: u8 = Ch8::from(c.cb()).into();
        let cb = i32::from(cb) << 8;
        let cr: u8 = Ch8::from(c.cr()).into();
        let cr = i32::from(cr) << 16;
        let alpha: u8 = Ch8::from(c.alpha()).into();
        let alpha = i32::from(alpha) << 24;
        y | cb | cr | alpha
    }
}

// FIXME
/*impl<C, A, G> Mul<Self> for YCbCr<C, A, Straight, G>
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
        YCbCr {
            components,
            alpha,
            mode: PhantomData,
            gamma: PhantomData,
        }
    }
}*/

// FIXME
/*impl<C, A, G> Mul<Self> for YCbCr<C, A, Premultiplied, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
    G: gamma::Mode,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let this: YCbCr<C, A, Straight, G> = self.into();
        let other: YCbCr<C, A, Straight, G> = rhs.into();

        (this * other).into()
    }
}*/

impl<C, A, M, G> YCbCr<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Create an [Opaque](alpha/struct.Opaque.html) color by specifying *y*,
    /// *cb* and *cr* values.
    pub fn new<H>(y: H, cb: H, cr: H) -> Self
    where
        C: From<H>,
        A: From<Opaque<C>>,
    {
        Self::with_alpha(y, cb, cr, Opaque::default())
    }
    /// Create a [Translucent](alpha/struct.Translucent.html) color by
    /// specifying *y*, *cb*, *cr* and *alpha* values.
    pub fn with_alpha<H, B>(y: H, cb: H, cr: H, alpha: B) -> Self
    where
        C: From<H>,
        A: From<B>,
    {
        let y = C::from(y);
        let cb = C::from(cb);
        let cr = C::from(cr);
        let components = [y, cb, cr];
        let alpha = A::from(alpha);
        YCbCr {
            components,
            alpha,
            mode: PhantomData,
            gamma: PhantomData,
        }
    }
    /// Get the y component.
    pub fn y(self) -> C {
        self.components[0]
    }
    /// Get the cb component.
    pub fn cb(self) -> C {
        self.components[1]
    }
    /// Get the cr component.
    pub fn cr(self) -> C {
        self.components[2]
    }
}

/// [YCbCr](struct.YCbCr.html) 8-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type YCbCr8 = YCbCr<Ch8, Opaque<Ch8>, Straight, Linear>;
/// [YCbCr](struct.YCbCr.html) 16-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type YCbCr16 = YCbCr<Ch16, Opaque<Ch16>, Straight, Linear>;
/// [YCbCr](struct.YCbCr.html) 32-bit [opaque](alpha/struct.Opaque.html) (no alpha)
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type YCbCr32 = YCbCr<Ch32, Opaque<Ch32>, Straight, Linear>;

/// [YCbCr](struct.YCbCr.html) 8-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type YCbCrAlpha8 = YCbCr<Ch8, Translucent<Ch8>, Straight, Linear>;
/// [YCbCr](struct.YCbCr.html) 16-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type YCbCrAlpha16 = YCbCr<Ch16, Translucent<Ch16>, Straight, Linear>;
/// [YCbCr](struct.YCbCr.html) 32-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type YCbCrAlpha32 = YCbCr<Ch32, Translucent<Ch32>, Straight, Linear>;

/// [YCbCr](struct.YCbCr.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type YCbCrAlpha8p = YCbCr<Ch8, Translucent<Ch8>, Premultiplied, Linear>;
/// [YCbCr](struct.YCbCr.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type YCbCrAlpha16p = YCbCr<Ch16, Translucent<Ch16>, Premultiplied, Linear>;
/// [YCbCr](struct.YCbCr.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type YCbCrAlpha32p = YCbCr<Ch32, Translucent<Ch32>, Premultiplied, Linear>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<YCbCr8>(), 3);
        assert_eq!(std::mem::size_of::<YCbCr16>(), 6);
        assert_eq!(std::mem::size_of::<YCbCr32>(), 12);
        assert_eq!(std::mem::size_of::<YCbCrAlpha8>(), 4);
        assert_eq!(std::mem::size_of::<YCbCrAlpha16>(), 8);
        assert_eq!(std::mem::size_of::<YCbCrAlpha32>(), 16);
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
