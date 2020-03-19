// ycc.rs       YCbCr color model.
//
// Copyright (c) 2019-2020  Jeron Aldaron Lau
// Copyright (c) 2020  Douglas P Lau
//
use crate::alpha::{
    self, AChannel, Mode as _, Opaque, Premultiplied, Straight, Translucent,
};
use crate::gamma::{self, Linear};
use crate::{Ch16, Ch32, Ch8, Channel, ColorModel, Pixel};
use std::marker::PhantomData;

/// `YCbCr` [color model] used in JPEG format.
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

impl<C, A, M, G> YCbCr<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
    /// Create a `YCbCr` color.
    pub fn new<H, B>(y: H, cb: H, cr: H, alpha: B) -> Self
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
    /// Get the *y* component.
    pub fn y(self) -> C {
        self.components[0]
    }
    /// Get the *Cb* component.
    pub fn cb(self) -> C {
        self.components[1]
    }
    /// Get the *Cr* component.
    pub fn cr(self) -> C {
        self.components[2]
    }
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
        let y = self.y().into();
        let cb = self.cb().into();
        let cr = self.cr().into();

        let r = y + (cr - 0.5) * 1.402;
        let g = y - (cb - 0.5) * 0.344136 - (cr - 0.5) * 0.714136;
        let b = y + (cb - 0.5) * 1.772;
        [r.into(), g.into(), b.into(), self.alpha()]
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn with_rgba(rgba: [Self::Chan; 4]) -> Self {
        let red = rgba[0].into();
        let green = rgba[1].into();
        let blue = rgba[2].into();
        let alpha = rgba[3];

        let y = (0.299 * red) + (0.587 * green) + (0.114 * blue);
        let cb = 0.5 - (0.168736 * red) - (0.331264 * green) + (0.5 * blue);
        let cr = 0.5 + (0.5 * red) - (0.418688 * green) - (0.081312 * blue);

        YCbCr::new(y, cb, cr, alpha)
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
        YCbCr::new(c.y(), c.cb(), c.cr(), ())
    }
}

impl<C, M, G> From<YCbCr<C, Opaque<C>, M, G>> for YCbCr<C, Translucent<C>, M, G>
where
    C: Channel,
    M: alpha::Mode,
    G: gamma::Mode,
{
    fn from(c: YCbCr<C, Opaque<C>, M, G>) -> Self {
        YCbCr::new(c.y(), c.cb(), c.cr(), ())
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
        YCbCr::new(y, cb, cr, c.alpha())
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
        YCbCr::new(y, cb, cr, c.alpha())
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
pub type YCbCra8 = YCbCr<Ch8, Translucent<Ch8>, Straight, Linear>;
/// [YCbCr](struct.YCbCr.html) 16-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type YCbCra16 = YCbCr<Ch16, Translucent<Ch16>, Straight, Linear>;
/// [YCbCr](struct.YCbCr.html) 32-bit [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type YCbCra32 = YCbCr<Ch32, Translucent<Ch32>, Straight, Linear>;

/// [YCbCr](struct.YCbCr.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type YCbCra8p = YCbCr<Ch8, Translucent<Ch8>, Premultiplied, Linear>;
/// [YCbCr](struct.YCbCr.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type YCbCra16p = YCbCr<Ch16, Translucent<Ch16>, Premultiplied, Linear>;
/// [YCbCr](struct.YCbCr.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](trait.Pixel.html) format.
pub type YCbCra32p = YCbCr<Ch32, Translucent<Ch32>, Premultiplied, Linear>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<YCbCr8>(), 3);
        assert_eq!(std::mem::size_of::<YCbCr16>(), 6);
        assert_eq!(std::mem::size_of::<YCbCr32>(), 12);
        assert_eq!(std::mem::size_of::<YCbCra8>(), 4);
        assert_eq!(std::mem::size_of::<YCbCra16>(), 8);
        assert_eq!(std::mem::size_of::<YCbCra32>(), 16);
    }
}
