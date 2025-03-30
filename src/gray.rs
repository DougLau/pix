// gray.rs      Grayscale color model.
//
// Copyright (c) 2018-2024  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
//! [`Gray`] color model and types.
//!
//! [`gray`]: https://en.wikipedia.org/wiki/Grayscale
use crate::ColorModel;
use crate::chan::{
    Ch8, Ch16, Ch32, Channel, Linear, Premultiplied, Srgb, Straight,
};
use crate::el::{Pix, PixRgba, Pixel};
use std::ops::Range;

/// Gray [color model].
///
/// The components are *[value]* and optional *[alpha]*.  *Value* ranges from
/// *black* to *white*.  With [sRGB] gamma it is *luma*, but with [linear]
/// gamma it is *relative luminance*.
///
/// [alpha]: ../el/trait.Pixel.html#method.alpha
/// [color model]: ../trait.ColorModel.html
/// [linear]: ../chan/struct.Linear.html
/// [sRGB]: ../chan/struct.Srgb.html
/// [value]: #method.value
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Gray {}

impl Gray {
    /// Get the *luma* / *relative luminance* component.
    ///
    /// # Example: Get Value
    /// ```
    /// use pix::chan::Ch16;
    /// use pix::gray::{Gray, Gray16};
    ///
    /// let p = Gray16::new(0x4000);
    /// assert_eq!(Gray::value(p), Ch16::new(0x4000));
    /// ```
    pub fn value<P>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.get::<0>()
    }

    /// Get a mutable reference to the *luma* / *relative luminance* component.
    ///
    /// # Example: Modify Value
    /// ```
    /// use pix::chan::Ch8;
    /// use pix::gray::{Gray, Gray8};
    ///
    /// let mut p = Gray8::new(0x40);
    /// *Gray::value_mut(&mut p) = 0x50.into();
    /// assert_eq!(Gray::value(p), Ch8::new(0x50));
    /// ```
    pub fn value_mut<P>(p: &mut P) -> &mut P::Chan
    where
        P: Pixel<Model = Self>,
    {
        p.get_mut::<0>()
    }
}

impl ColorModel for Gray {
    const CIRCULAR: Range<usize> = 0..0;
    const LINEAR: Range<usize> = 0..1;
    const ALPHA: usize = 1;

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> PixRgba<P>
    where
        P: Pixel<Model = Self>,
    {
        let value = Self::value(p);
        PixRgba::<P>::new::<P::Chan>(value, value, value, p.alpha())
    }

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P>(rgba: PixRgba<P>) -> P
    where
        P: Pixel<Model = Self>,
    {
        const RED_COEF: f32 = 0.212_6;
        const GREEN_COEF: f32 = 0.715_2;
        const BLUE_COEF: f32 = 0.072_2;

        let chan = rgba.channels();
        let red = chan[0].to_f32() * RED_COEF;
        let green = chan[1].to_f32() * GREEN_COEF;
        let blue = chan[2].to_f32() * BLUE_COEF;
        let value = P::Chan::from(red + green + blue);
        let alpha = chan[3];
        P::from_channels(&[value, alpha])
    }
}

/// [Gray](struct.Gray.html) 8-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Gray8 = Pix<1, Ch8, Gray, Straight, Linear>;

/// [Gray](struct.Gray.html) 16-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Gray16 = Pix<1, Ch16, Gray, Straight, Linear>;

/// [Gray](struct.Gray.html) 32-bit opaque (no *alpha* channel)
/// [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Gray32 = Pix<1, Ch32, Gray, Straight, Linear>;

/// [Gray](struct.Gray.html) 8-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Graya8 = Pix<2, Ch8, Gray, Straight, Linear>;

/// [Gray](struct.Gray.html) 16-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Graya16 = Pix<2, Ch16, Gray, Straight, Linear>;

/// [Gray](struct.Gray.html) 32-bit [straight](../chan/struct.Straight.html)
/// alpha [linear](../chan/struct.Linear.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type Graya32 = Pix<2, Ch32, Gray, Straight, Linear>;

/// [Gray](struct.Gray.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Graya8p = Pix<2, Ch8, Gray, Premultiplied, Linear>;

/// [Gray](struct.Gray.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Graya16p = Pix<2, Ch16, Gray, Premultiplied, Linear>;

/// [Gray](struct.Gray.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [linear](../chan/struct.Linear.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type Graya32p = Pix<2, Ch32, Gray, Premultiplied, Linear>;

/// [Gray](struct.Gray.html) 8-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SGray8 = Pix<1, Ch8, Gray, Straight, Srgb>;

/// [Gray](struct.Gray.html) 16-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SGray16 = Pix<1, Ch16, Gray, Straight, Srgb>;

/// [Gray](struct.Gray.html) 32-bit opaque (no *alpha* channel)
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SGray32 = Pix<1, Ch32, Gray, Straight, Srgb>;

/// [Gray](struct.Gray.html) 8-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type SGraya8 = Pix<2, Ch8, Gray, Straight, Srgb>;

/// [Gray](struct.Gray.html) 16-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type SGraya16 = Pix<2, Ch16, Gray, Straight, Srgb>;

/// [Gray](struct.Gray.html) 32-bit [straight](../chan/struct.Straight.html)
/// alpha [sRGB](../chan/struct.Srgb.html) gamma
/// [pixel](../el/trait.Pixel.html) format.
pub type SGraya32 = Pix<2, Ch32, Gray, Straight, Srgb>;

/// [Gray](struct.Gray.html) 8-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SGraya8p = Pix<2, Ch8, Gray, Premultiplied, Srgb>;

/// [Gray](struct.Gray.html) 16-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SGraya16p = Pix<2, Ch16, Gray, Premultiplied, Srgb>;

/// [Gray](struct.Gray.html) 32-bit
/// [premultiplied](../chan/struct.Premultiplied.html) alpha
/// [sRGB](../chan/struct.Srgb.html) gamma [pixel](../el/trait.Pixel.html)
/// format.
pub type SGraya32p = Pix<2, Ch32, Gray, Premultiplied, Srgb>;

#[cfg(test)]
mod test {
    use crate::el::Pixel;
    use crate::gray::*;
    use crate::matte::*;
    use crate::rgb::*;

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
    fn matte_to_gray() {
        assert_eq!(SGraya8::new(0xFF, 0xAB), Matte16::new(0xABCD).convert());
        assert_eq!(SGraya8::new(0xFF, 0x98), Matte16::new(0x9876).convert());
    }

    #[test]
    fn gray_to_matte() {
        assert_eq!(Matte16::new(0x9494), SGraya8::new(0x67, 0x94).convert());
        assert_eq!(Matte16::new(0xA2A2), SGraya8::new(0xBA, 0xA2).convert());
        assert_eq!(Matte8::new(0x80), SGraya32::new(0.75, 0.5).convert());
    }
}
