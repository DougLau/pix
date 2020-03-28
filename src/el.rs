// el.rs        Pixel format.
//
// Copyright (c) 2018-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
//! Module for `pix::el` items
use crate::alpha;
use crate::channel::{Ch16, Ch8, Channel};
use crate::gamma;
use crate::model::ColorModel;
use crate::private::Sealed;
use std::fmt::Debug;
use std::marker::PhantomData;

/// Pixel [channel], [color model], [alpha mode] and [gamma mode].
///
/// A pixel can be converted to another format using the [convert] method.
///
/// [alpha mode]: ../alpha/trait.Mode.html
/// [channel]: ../channel/trait.Channel.html
/// [color model]: ../trait.ColorModel.html
/// [convert]: trait.Pixel.html#method.convert
/// [gamma mode]: ../gamma/trait.Mode.html
///
/// ### Type Alias Naming Scheme
///
/// * _Gamma_: `S` for [sRGB] gamma encoding; [linear] if omitted.
/// * _Color model_: [`Rgb`] / [`Gray`] / [`Hsv`] / [`Hsl`] / [`Hwb`] /
///                  [`YCbCr`] / [`Mask`].
/// * _Alpha_: `a` to include alpha channel enabling translucent pixels.
/// * _Bit depth_: `8` / `16` / `32` for 8-bit integer, 16-bit integer and
///   32-bit floating-point [channels].
/// * _Alpha mode_: `p` for [premultiplied]; [straight] if omitted.
///
/// [channels]: ../channel/trait.Channel.html
/// [`gray`]: ../struct.GrayModel.html
/// [`hsl`]: ../struct.HslModel.html
/// [`hsv`]: ../struct.HsvModel.html
/// [`hwb`]: ../struct.HwbModel.html
/// [linear]: ../gamma/struct.Linear.html
/// [`mask`]: ../struct.MaskModel.html
/// [premultiplied]: ../alpha/struct.Premultiplied.html
/// [`Rgb`]: ../struct.RgbModel.html
/// [sRGB]: ../gamma/struct.Srgb.html
/// [straight]: ../alpha/struct.Straight.html
/// [`YCbCr`]: ../struct.YCbCrModel.html
///
/// ### Type Aliases
///
/// * Opaque, linear gamma:
///   [Rgb8](../type.Rgb8.html),
///   [Gray8](../type.Gray8.html),
///   [Hsv8](../type.Hsv8.html),
///   [Hsl8](../type.Hsl8.html),
///   [Rgb16](../type.Rgb16.html),
///   *etc.*
/// * Opaque, sRGB gamma:
///   [SRgb8](../type.SRgb8.html),
///   [SGray8](../type.SGray8.html),
///   [SRgb16](../type.SRgb16.html),
///   *etc.*
/// * Translucent (straight alpha), linear gamma:
///   [Rgba8](../type.Rgba8.html),
///   [Graya8](../type.Graya8.html),
///   [Hsva8](../type.Hsva8.html),
///   [Hsla8](../type.Hsla8.html),
///   [Rgba16](../type.Rgba16.html),
///   *etc.*
/// * Translucent (premultiplied alpha), linear gamma:
///   [Rgba8p](../type.Rgba8p.html),
///   [Graya8p](../type.Graya8p.html),
///   [Hsva8p](../type.Hsva8p.html),
///   [Hsla8p](../type.Hsla8p.html),
///   [Rgba16p](../type.Rgba16p.html),
///   *etc.*
/// * Translucent (straight alpha), sRGB gamma:
///   [SRgba8](../type.SRgba8.html),
///   [SGraya8](../type.SGraya8.html),
///   [SRgba16](../type.SRgba16.html),
///   *etc.*
/// * Translucent (premultiplied alpha), sRGB gamma:
///   [SRgba8p](../type.SRgba8p.html),
///   [SGraya8p](../type.SGraya8p.html),
///   [SRgba16p](../type.SRgba16p.html),
///   *etc.*
/// * Alpha mask:
///   [Mask8](../type.Mask8.html),
///   [Mask16](../type.Mask16.html),
///   [Mask32](../type.Mask32.html)
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
pub trait Pixel: Clone + Copy + Debug + Default + PartialEq + Sealed {
    /// Channel type
    type Chan: Channel;

    /// Color model
    type Model: ColorModel;

    /// Alpha mode
    type Alpha: alpha::Mode;

    /// Gamma mode
    type Gamma: gamma::Mode;

    /// Make a pixel from an array of channels.
    fn from_channels<H>(ch: [H; 4]) -> Self
    where
        H: Copy,
        Self::Chan: From<H>;

    /// Get the first channel.
    fn one(self) -> Self::Chan {
        Self::Chan::MAX
    }

    /// Get the second channel.
    fn two(self) -> Self::Chan {
        Self::Chan::MAX
    }

    /// Get the third channel.
    fn three(self) -> Self::Chan {
        Self::Chan::MAX
    }

    /// Get the fourth channel.
    fn four(self) -> Self::Chan {
        Self::Chan::MAX
    }

    /// Convert a pixel to another format
    ///
    /// * `D` Destination format.
    fn convert<D>(self) -> D
    where
        D: Pixel,
        D::Chan: From<Self::Chan>,
    {
        let channels = Self::Model::into_channels::<Self, D>(self);
        let channels = channels.convert::<Self, D>();
        D::Model::from_channels::<Self, D>(channels)
    }
}

/// [Pixel] with one [channel] in its [color model].
///
/// [channel]: ../channel/trait.Channel.html
/// [color model]: ../trait.ColorModel.html
/// [pixel]: trait.Pixel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Pix1<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    one: C,
    _model: PhantomData<M>,
    _alpha: PhantomData<A>,
    _gamma: PhantomData<G>,
}

impl<C, M, A, G> Pix1<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    /// Create a one-channel color.
    ///
    /// ## Example
    /// ```
    /// # use pix::*;
    /// let opaque_gray = Gray8::new(128);
    /// ```
    pub fn new<H>(one: H) -> Self
    where
        C: From<H>,
    {
        let one = C::from(one);
        Pix1 {
            one,
            _model: PhantomData,
            _alpha: PhantomData,
            _gamma: PhantomData,
        }
    }
}

impl<C, M, A, G> Pixel for Pix1<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    type Chan = C;
    type Model = M;
    type Alpha = A;
    type Gamma = G;

    fn from_channels<H>(ch: [H; 4]) -> Self
    where
        H: Copy,
        Self::Chan: From<H>,
    {
        let one = Self::Chan::from(ch[0]);
        Self::new(one)
    }

    fn one(self) -> C {
        self.one
    }
}

impl<C, M, A, G> Iterator for Pix1<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

impl<C, M, A, G> From<u8> for Pix1<C, M, A, G>
where
    C: Channel + From<Ch8>,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    /// Convert from a `u8` value.
    fn from(c: u8) -> Self {
        Pix1::new(Ch8::new(c))
    }
}

impl<C, M, A, G> From<u16> for Pix1<C, M, A, G>
where
    C: Channel + From<Ch16>,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    /// Convert from a `u16` value.
    fn from(c: u16) -> Self {
        Pix1::new(Ch16::new(c))
    }
}

/// [Pixel] with two [channel]s in its [color model].
///
/// [channel]: ../channel/trait.Channel.html
/// [color model]: ../trait.ColorModel.html
/// [pixel]: trait.Pixel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Pix2<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    one: C,
    two: C,
    _model: PhantomData<M>,
    _alpha: PhantomData<A>,
    _gamma: PhantomData<G>,
}

impl<C, M, A, G> Pix2<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    /// Create a two-channel color.
    ///
    /// ## Example
    /// ```
    /// # use pix::*;
    /// let translucent_gray = Graya8::new(128, 200);
    /// ```
    pub fn new<H>(one: H, two: H) -> Self
    where
        C: From<H>,
    {
        let one = C::from(one);
        let two = C::from(two);
        Pix2 {
            one,
            two,
            _model: PhantomData,
            _alpha: PhantomData,
            _gamma: PhantomData,
        }
    }
}

impl<C, M, A, G> Pixel for Pix2<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    type Chan = C;
    type Model = M;
    type Alpha = A;
    type Gamma = G;

    fn from_channels<H>(ch: [H; 4]) -> Self
    where
        H: Copy,
        Self::Chan: From<H>,
    {
        let one = ch[0];
        let two = ch[1];
        Self::new(one, two)
    }

    fn one(self) -> C {
        self.one
    }

    fn two(self) -> C {
        self.two
    }
}

impl<C, M, A, G> Iterator for Pix2<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

/// [Pixel] with three [channel]s in its [color model].
///
/// [channel]: ../channel/trait.Channel.html
/// [color model]: ../trait.ColorModel.html
/// [pixel]: trait.Pixel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Pix3<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    one: C,
    two: C,
    three: C,
    _model: PhantomData<M>,
    _alpha: PhantomData<A>,
    _gamma: PhantomData<G>,
}

impl<C, M, A, G> Pix3<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    /// Create a three-channel color.
    ///
    /// ## Example
    /// ```
    /// # use pix::*;
    /// let rgb = Rgb8::new(128, 200, 255);
    /// ```
    pub fn new<H>(one: H, two: H, three: H) -> Self
    where
        C: From<H>,
    {
        let one = C::from(one);
        let two = C::from(two);
        let three = C::from(three);
        Pix3 {
            one,
            two,
            three,
            _model: PhantomData,
            _alpha: PhantomData,
            _gamma: PhantomData,
        }
    }
}

impl<C, M, A, G> Pixel for Pix3<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    type Chan = C;
    type Model = M;
    type Alpha = A;
    type Gamma = G;

    fn from_channels<H>(ch: [H; 4]) -> Self
    where
        H: Copy,
        Self::Chan: From<H>,
    {
        let one = ch[0];
        let two = ch[1];
        let three = ch[2];
        Self::new(one, two, three)
    }

    fn one(self) -> C {
        self.one
    }

    fn two(self) -> C {
        self.two
    }

    fn three(self) -> C {
        self.three
    }
}

impl<C, M, A, G> Iterator for Pix3<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

/// [Pixel] with four [channel]s in its [color model].
///
/// [channel]: ../channel/trait.Channel.html
/// [color model]: ../trait.ColorModel.html
/// [pixel]: trait.Pixel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Pix4<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    one: C,
    two: C,
    three: C,
    four: C,
    _model: PhantomData<M>,
    _alpha: PhantomData<A>,
    _gamma: PhantomData<G>,
}

impl<C, M, A, G> Pix4<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    /// Create a four-channel color.
    ///
    /// ## Example
    /// ```
    /// # use pix::*;
    /// let rgba = Rgba8::new(128, 200, 255, 128);
    /// ```
    pub fn new<H>(one: H, two: H, three: H, four: H) -> Self
    where
        C: From<H>,
    {
        let one = C::from(one);
        let two = C::from(two);
        let three = C::from(three);
        let four = C::from(four);
        Pix4 {
            one,
            two,
            three,
            four,
            _model: PhantomData,
            _alpha: PhantomData,
            _gamma: PhantomData,
        }
    }
}

impl<C, M, A, G> Pixel for Pix4<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    type Chan = C;
    type Model = M;
    type Alpha = A;
    type Gamma = G;

    fn from_channels<H>(ch: [H; 4]) -> Self
    where
        H: Copy,
        Self::Chan: From<H>,
    {
        let one = ch[0];
        let two = ch[1];
        let three = ch[2];
        let four = ch[3];
        Self::new(one, two, three, four)
    }

    fn one(self) -> C {
        self.one
    }

    fn two(self) -> C {
        self.two
    }

    fn three(self) -> C {
        self.three
    }

    fn four(self) -> C {
        self.four
    }
}

impl<C, M, A, G> Iterator for Pix4<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: alpha::Mode,
    G: gamma::Mode,
{
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

#[cfg(test)]
mod test {
    use super::super::*;
    use super::*;
    #[test]
    fn gray_to_rgb() {
        assert_eq!(SRgb8::new(0xD9, 0xD9, 0xD9), SGray8::new(0xD9).convert(),);
        assert_eq!(
            SRgb8::new(0x33, 0x33, 0x33),
            SGray16::new(0x337F).convert(),
        );
        assert_eq!(SRgb8::new(0x40, 0x40, 0x40), SGray32::new(0.25).convert(),);
        assert_eq!(
            SRgb16::new(0x2929, 0x2929, 0x2929),
            SGray8::new(0x29).convert(),
        );
        assert_eq!(
            SRgb16::new(0x5593, 0x5593, 0x5593),
            SGray16::new(0x5593).convert(),
        );
        assert_eq!(
            SRgb16::new(0xFFFF, 0xFFFF, 0xFFFF),
            SGray32::new(1.0).convert(),
        );
        assert_eq!(
            SRgb32::new(0.5019608, 0.5019608, 0.5019608),
            SGray8::new(0x80).convert(),
        );
        assert_eq!(
            SRgb32::new(0.75001144, 0.75001144, 0.75001144),
            SGray16::new(0xC000).convert(),
        );
        assert_eq!(SRgb32::new(0.33, 0.33, 0.33), SGray32::new(0.33).convert(),);
    }
    #[test]
    fn linear_to_srgb() {
        assert_eq!(
            SRgb8::new(0xEF, 0x8C, 0xC7),
            Rgb8::new(0xDC, 0x43, 0x91).convert()
        );
        assert_eq!(
            SRgb8::new(0x66, 0xF4, 0xB5),
            Rgb16::new(0x2205, 0xE699, 0x7654).convert()
        );
        assert_eq!(
            SRgb8::new(0xBC, 0x89, 0xE0),
            Rgb32::new(0.5, 0.25, 0.75).convert()
        );
    }
    #[test]
    fn srgb_to_linear() {
        assert_eq!(
            Rgb8::new(0xDC, 0x43, 0x92),
            SRgb8::new(0xEF, 0x8C, 0xC7).convert(),
        );
        assert_eq!(
            Rgb8::new(0x22, 0xE7, 0x76),
            SRgb16::new(0x6673, 0xF453, 0xB593).convert(),
        );
        assert_eq!(
            Rgb8::new(0x37, 0x0D, 0x85),
            SRgb32::new(0.5, 0.25, 0.75).convert(),
        );
    }
    #[test]
    fn straight_to_premultiplied() {
        assert_eq!(
            Rgba8p::new(0x10, 0x20, 0x40, 0x80),
            Rgba8::new(0x20, 0x40, 0x80, 0x80).convert(),
        );
        assert_eq!(
            Rgba8p::new(0x04, 0x10, 0x20, 0x40),
            Rgba16::new(0x1000, 0x4000, 0x8000, 0x4000).convert(),
        );
        assert_eq!(
            Rgba8p::new(0x60, 0xBF, 0x8F, 0xBF),
            Rgba32::new(0.5, 1.0, 0.75, 0.75).convert(),
        );
    }
    #[test]
    fn premultiplied_to_straight() {
        assert_eq!(
            Rgba8::new(0x40, 0x80, 0xFF, 0x80),
            Rgba8p::new(0x20, 0x40, 0x80, 0x80).convert(),
        );
        assert_eq!(
            Rgba8::new(0x40, 0xFF, 0x80, 0x40),
            Rgba16p::new(0x1000, 0x4000, 0x2000, 0x4000).convert(),
        );
        assert_eq!(
            Rgba8::new(0xAB, 0x55, 0xFF, 0xBF),
            Rgba32p::new(0.5, 0.25, 0.75, 0.75).convert(),
        );
    }
    #[test]
    fn straight_to_premultiplied_srgb() {
        assert_eq!(
            SRgba8p::new(0x16, 0x2A, 0x5C, 0x80),
            SRgba8::new(0x20, 0x40, 0x80, 0x80).convert(),
        );
        assert_eq!(
            SRgba8p::new(0x0D, 0x1C, 0x40, 0x40),
            SRgba16::new(0x2000, 0x4000, 0x8000, 0x4000).convert(),
        );
        assert_eq!(
            SRgba8p::new(0x70, 0xE0, 0xA7, 0xBF),
            SRgba32::new(0.5, 1.0, 0.75, 0.75).convert(),
        );
    }
}
