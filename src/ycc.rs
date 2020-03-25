// ycc.rs       YCbCr color model.
//
// Copyright (c) 2019-2020  Jeron Aldaron Lau
// Copyright (c) 2020  Douglas P Lau
//
use crate::alpha::{
    self, AChannel, Opaque, Premultiplied, Straight, Translucent,
};
use crate::gamma::{self, Linear};
use crate::model::Channels;
use crate::{Ch16, Ch32, Ch8, Channel, ColorModel, Pixel};
use std::any::TypeId;
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
    y: C,
    cb: C,
    cr: C,
    alpha: A,
    mode: PhantomData<M>,
    gamma: PhantomData<G>,
}

impl<C, A, M, G> YCbCr<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C> + From<C>,
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
        let alpha = A::from(alpha);
        YCbCr {
            y,
            cb,
            cr,
            alpha,
            mode: PhantomData,
            gamma: PhantomData,
        }
    }

    /// Get the *y* component.
    pub fn y(self) -> C {
        self.y
    }

    /// Get the *Cb* component.
    pub fn cb(self) -> C {
        self.cb
    }

    /// Get the *Cr* component.
    pub fn cr(self) -> C {
        self.cr
    }

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba(self) -> [C; 4] {
        let y = self.y().into();
        let cb = self.cb().into();
        let cr = self.cr().into();

        let r = y + (cr - 0.5) * 1.402;
        let g = y - (cb - 0.5) * 0.344136 - (cr - 0.5) * 0.714136;
        let b = y + (cb - 0.5) * 1.772;
        [r.into(), g.into(), b.into(), self.alpha()]
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba(rgba: [C; 4]) -> Self {
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

impl<C, A, M, G> ColorModel for YCbCr<C, A, M, G>
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
    fn into_channels<R: ColorModel>(self) -> Channels<C> {
        if TypeId::of::<Self>() == TypeId::of::<R>() {
            Channels::new([
                self.y(),
                self.cb(),
                self.cr(),
                self.alpha(),
            ], 3)
        } else {
            Channels::new(self.into_rgba(), 3)
        }
    }

    /// Convert from channels shared by types
    fn from_channels<R: ColorModel>(channels: Channels<C>) -> Self {
        if TypeId::of::<Self>() == TypeId::of::<R>() {
            debug_assert_eq!(channels.alpha(), 3);
            let ch = channels.into_array();
            let y = ch[0];
            let cb = ch[1];
            let cr = ch[2];
            let alpha = ch[3];
            YCbCr::new(y, cb, cr, alpha)
        } else {
            debug_assert_eq!(channels.alpha(), 3);
            Self::from_rgba(channels.into_array())
        }
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
