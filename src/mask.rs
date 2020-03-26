// mask.rs      Alpha mask color model.
//
// Copyright (c) 2019-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::alpha::Straight;
use crate::channel::{Ch16, Ch32, Ch8, Channel};
use crate::gamma::Linear;
use crate::model::{Channels, ColorModel};
use crate::el::{Pix1, Pixel};

/// Mask [color model].
///
/// The component is *alpha* only.
///
/// [color model]: trait.ColorModel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MaskModel {}

impl ColorModel for MaskModel {
    /// Get the *alpha* component.
    fn alpha<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }

    /// Convert into channels shared by pixel types
    fn into_channels<S, D>(src: S) -> Channels<S::Chan>
    where
        S: Pixel<Model = Self>,
        D: Pixel,
    {
        Channels::new(Self::into_rgba(src), 3)
    }

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> [P::Chan; 4]
    where
        P: Pixel<Model = Self>,
    {
        let max = P::Chan::MAX;
        [max, max, max, Self::alpha(p)]
    }

    /// Convert from channels shared by pixel types
    fn from_channels<S: Pixel, D: Pixel>(channels: Channels<D::Chan>) -> D {
        debug_assert_eq!(channels.alpha_idx(), 3);
        Self::from_rgba::<D>(channels.into_array())
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P: Pixel>(rgba: [P::Chan; 4]) -> P {
        let min = P::Chan::MIN;
        let chan = [rgba[3], min, min, min];
        P::from_channels::<P::Chan>(chan)
    }
}

/// [Mask](struct.MaskModel.html) 8-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Mask8 = Pix1<Ch8, MaskModel, Straight, Linear>;

/// [Mask](struct.MaskModel.html) 16-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Mask16 = Pix1<Ch16, MaskModel, Straight, Linear>;

/// [Mask](struct.MaskModel.html) 32-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Mask32 = Pix1<Ch32, MaskModel, Straight, Linear>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<Mask8>(), 1);
        assert_eq!(std::mem::size_of::<Mask16>(), 2);
        assert_eq!(std::mem::size_of::<Mask32>(), 4);
    }
}
