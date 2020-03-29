// gray.rs      Grayscale color model.
//
// Copyright (c) 2018-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
use crate::alpha::{Premultiplied, Straight};
use crate::channel::{Ch16, Ch32, Ch8, Channel};
use crate::el::{Pix1, Pix2, Pixel};
use crate::gamma::{Linear, Srgb};
use crate::model::{Channels, ColorModel};
use std::any::TypeId;

/// Gray [color model].
///
/// The component is *gray*, with optional *alpha*.  It ranges from *black* to
/// *white*.  With [sRGB] gamma it is *luma*, but with [linear] gamma it is
/// *relative luminance*.
///
/// [color model]: trait.ColorModel.html
/// [linear]: gamma/struct.Linear.html
/// [sRGB]: gamma/struct.Srgb.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Gray {}

impl Gray {
    /// Get the *luma* / *relative luminance* component.
    ///
    /// # Example: Gray Value
    /// ```
    /// # use pix::*;
    /// # use pix::channel::Ch16;
    /// let p = Gray16::new(0x4000);
    /// assert_eq!(Gray::value(p), Ch16::new(0x4000));
    /// ```
    pub fn value<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.one()
    }
}

impl ColorModel for Gray {
    /// Get the *alpha* component.
    fn alpha<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.two()
    }

    /// Convert into channels shared by pixel types
    fn into_channels<S, D>(src: S) -> Channels<S::Chan>
    where
        S: Pixel<Model = Self>,
        D: Pixel,
    {
        if TypeId::of::<S::Model>() == TypeId::of::<D::Model>() {
            let min = S::Chan::MIN;
            Channels::new([Self::value(src), Self::alpha(src), min, min], 1)
        } else {
            Channels::new(Self::into_rgba(src), 3)
        }
    }

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> [P::Chan; 4]
    where
        P: Pixel<Model = Self>,
    {
        let value = Self::value(p);
        [value, value, value, Self::alpha(p)]
    }

    /// Convert from channels shared by pixel types
    fn from_channels<S: Pixel, D: Pixel>(channels: Channels<D::Chan>) -> D {
        if TypeId::of::<S::Model>() == TypeId::of::<D::Model>() {
            debug_assert_eq!(channels.alpha_idx(), 1);
            D::from_channels::<D::Chan>(channels.into_array())
        } else {
            debug_assert_eq!(channels.alpha_idx(), 3);
            Self::from_rgba::<D>(channels.into_array())
        }
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P: Pixel>(rgba: [P::Chan; 4]) -> P {
        const RED_COEF: f32 = 0.2126;
        const GREEN_COEF: f32 = 0.7152;
        const BLUE_COEF: f32 = 0.0722;

        let red = rgba[0].into() * RED_COEF;
        let green = rgba[1].into() * GREEN_COEF;
        let blue = rgba[2].into() * BLUE_COEF;
        let value = P::Chan::from(red + green + blue);
        let alpha = rgba[3];
        let min = P::Chan::MIN;
        P::from_channels::<P::Chan>([value, alpha, min, min])
    }
}

/// [Gray](struct.Gray.html) 8-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma
/// [pixel](el/trait.Pixel.html) format.
pub type Gray8 = Pix1<Ch8, Gray, Straight, Linear>;
/// [Gray](struct.Gray.html) 16-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma
/// [pixel](el/trait.Pixel.html) format.
pub type Gray16 = Pix1<Ch16, Gray, Straight, Linear>;
/// [Gray](struct.Gray.html) 32-bit opaque (no *alpha* channel)
/// [linear](gamma/struct.Linear.html) gamma
/// [pixel](el/trait.Pixel.html) format.
pub type Gray32 = Pix1<Ch32, Gray, Straight, Linear>;

/// [Gray](struct.Gray.html) 8-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Graya8 = Pix2<Ch8, Gray, Straight, Linear>;
/// [Gray](struct.Gray.html) 16-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Graya16 = Pix2<Ch16, Gray, Straight, Linear>;
/// [Gray](struct.Gray.html) 32-bit [straight](alpha/struct.Straight.html)
/// alpha [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Graya32 = Pix2<Ch32, Gray, Straight, Linear>;

/// [Gray](struct.Gray.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Graya8p = Pix2<Ch8, Gray, Premultiplied, Linear>;
/// [Gray](struct.Gray.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Graya16p = Pix2<Ch16, Gray, Premultiplied, Linear>;
/// [Gray](struct.Gray.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [linear](gamma/struct.Linear.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type Graya32p = Pix2<Ch32, Gray, Premultiplied, Linear>;

/// [Gray](struct.Gray.html) 8-bit opaque (no *alpha* channel)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SGray8 = Pix1<Ch8, Gray, Straight, Srgb>;
/// [Gray](struct.Gray.html) 16-bit opaque (no *alpha* channel)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SGray16 = Pix1<Ch16, Gray, Straight, Srgb>;
/// [Gray](struct.Gray.html) 32-bit opaque (no *alpha* channel)
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SGray32 = Pix1<Ch32, Gray, Straight, Srgb>;

/// [Gray](struct.Gray.html) 8-bit [straight](alpha/struct.Straight.html)
/// alpha [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type SGraya8 = Pix2<Ch8, Gray, Straight, Srgb>;
/// [Gray](struct.Gray.html) 16-bit [straight](alpha/struct.Straight.html)
/// alpha [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type SGraya16 = Pix2<Ch16, Gray, Straight, Srgb>;
/// [Gray](struct.Gray.html) 32-bit [straight](alpha/struct.Straight.html)
/// alpha [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html)
/// format.
pub type SGraya32 = Pix2<Ch32, Gray, Straight, Srgb>;

/// [Gray](struct.Gray.html) 8-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SGraya8p = Pix2<Ch8, Gray, Premultiplied, Srgb>;
/// [Gray](struct.Gray.html) 16-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SGraya16p = Pix2<Ch16, Gray, Premultiplied, Srgb>;
/// [Gray](struct.Gray.html) 32-bit
/// [premultiplied](alpha/struct.Premultiplied.html) alpha
/// [sRGB](gamma/struct.Srgb.html) gamma [pixel](el/trait.Pixel.html) format.
pub type SGraya32p = Pix2<Ch32, Gray, Premultiplied, Srgb>;

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

    #[test]
    fn rgb_to_gray() {
        assert_eq!(
            SGray8::new(0x7B),
            SRgb16::new(0x4321, 0x9085, 0x5543).convert(),
        );
        assert_eq!(
            SGray8::new(0x46),
            SRgb16::new(0x5768, 0x4091, 0x5000).convert(),
        );
    }

    #[test]
    fn gray_to_rgb() {
        assert_eq!(SRgb8::new(0x45, 0x45, 0x45), SGray8::new(0x45).convert());
        assert_eq!(
            SRgb8::new(0xDA, 0xDA, 0xDA),
            SGraya8::new(0xDA, 0x33).convert(),
        );
        assert_eq!(SRgb8::new(0xBA, 0xBA, 0xBA), Gray8::new(0x7D).convert());
    }

    #[test]
    fn mask_to_gray() {
        assert_eq!(SGraya8::new(0xFF, 0xAB), Mask16::new(0xABCD).convert());
        assert_eq!(SGraya8::new(0xFF, 0x98), Mask16::new(0x9876).convert());
    }

    #[test]
    fn gray_to_mask() {
        assert_eq!(Mask16::new(0x9494), SGraya8::new(0x67, 0x94).convert());
        assert_eq!(Mask16::new(0xA2A2), SGraya8::new(0xBA, 0xA2).convert());
        assert_eq!(Mask8::new(0x80), SGraya32::new(0.75, 0.5).convert());
    }
}
