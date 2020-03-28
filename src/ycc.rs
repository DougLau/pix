// ycc.rs       YCbCr color model.
//
// Copyright (c) 2019-2020  Jeron Aldaron Lau
// Copyright (c) 2020  Douglas P Lau
//
use crate::alpha::{Premultiplied, Straight};
use crate::channel::{Ch16, Ch32, Ch8};
use crate::gamma::Linear;
use crate::model::{Channels, ColorModel};
use crate::el::{Pix3, Pix4, Pixel};
use std::any::TypeId;

/// YCbCr [color model] used in JPEG format.
///
/// The components are *y*, *cb* and *cr*, with optional *alpha*.
///
/// [channel]: trait.Channel.html
/// [color model]: trait.ColorModel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct YCbCr {}

impl YCbCr {
    /// Get the *y* component.
    pub fn y<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }

    /// Get the *Cb* component.
    pub fn cb<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two()
    }

    /// Get the *Cr* component.
    pub fn cr<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.three()
    }
}

impl ColorModel for YCbCr {
    /// Get the *alpha* component.
    fn alpha<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.four()
    }

    /// Convert into channels shared by pixel types
    fn into_channels<S, D>(src: S) -> Channels<S::Chan>
    where
        S: Pixel<Model = Self>,
        D: Pixel,
    {
        if TypeId::of::<S::Model>() == TypeId::of::<D::Model>() {
            Channels::new(
                [Self::y(src), Self::cb(src), Self::cr(src), Self::alpha(src)],
                3,
            )
        } else {
            Channels::new(Self::into_rgba(src), 3)
        }
    }

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> [P::Chan; 4]
    where
        P: Pixel<Model = Self>,
    {
        let y = Self::y(p).into();
        let cb = Self::cb(p).into();
        let cr = Self::cr(p).into();

        let r = y + (cr - 0.5) * 1.402;
        let g = y - (cb - 0.5) * 0.344136 - (cr - 0.5) * 0.714136;
        let b = y + (cb - 0.5) * 1.772;
        [r.into(), g.into(), b.into(), Self::alpha(p)]
    }

    /// Convert from channels shared by pixel types
    fn from_channels<S: Pixel, D: Pixel>(channels: Channels<D::Chan>) -> D {
        if TypeId::of::<S::Model>() == TypeId::of::<D::Model>() {
            debug_assert_eq!(channels.alpha_idx(), 3);
            D::from_channels::<D::Chan>(channels.into_array())
        } else {
            debug_assert_eq!(channels.alpha_idx(), 3);
            Self::from_rgba(channels.into_array())
        }
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P: Pixel>(rgba: [P::Chan; 4]) -> P {
        let red = rgba[0].into();
        let green = rgba[1].into();
        let blue = rgba[2].into();
        let alpha = rgba[3];

        let y = (0.299 * red) + (0.587 * green) + (0.114 * blue);
        let cb = 0.5 - (0.168736 * red) - (0.331264 * green) + (0.5 * blue);
        let cr = 0.5 + (0.5 * red) - (0.418688 * green) - (0.081312 * blue);

        P::from_channels::<P::Chan>([y.into(), cb.into(), cr.into(), alpha])
    }
}

/// [YCbCr](struct.YCbCr.html) 8-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCr8 = Pix3<Ch8, YCbCr, Straight, Linear>;
/// [YCbCr](struct.YCbCr.html) 16-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCr16 = Pix3<Ch16, YCbCr, Straight, Linear>;
/// [YCbCr](struct.YCbCr.html) 32-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCr32 = Pix3<Ch32, YCbCr, Straight, Linear>;

/// [YCbCr](struct.YCbCr.html) 8-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCra8 = Pix4<Ch8, YCbCr, Straight, Linear>;
/// [YCbCr](struct.YCbCr.html) 16-bit
/// [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCra16 = Pix4<Ch16, YCbCr, Straight, Linear>;
/// [YCbCr](struct.YCbCr.html) 32-bit
/// [straight](alpha/struct.Straight.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCra32 = Pix4<Ch32, YCbCr, Straight, Linear>;

/// [YCbCr](struct.YCbCr.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCra8p = Pix4<Ch8, YCbCr, Premultiplied, Linear>;
/// [YCbCr](struct.YCbCr.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCra16p = Pix4<Ch16, YCbCr, Premultiplied, Linear>;
/// [YCbCr](struct.YCbCr.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type YCbCra32p = Pix4<Ch32, YCbCr, Premultiplied, Linear>;

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
