// el.rs        Pixel format.
//
// Copyright (c) 2018-2020  Douglas P Lau
// Copyright (c) 2019-2020  Jeron Aldaron Lau
//
//! Module for `pix::el` items
use crate::chan::{Alpha, Channel, Gamma};
use crate::clr::{ColorModel, Rgb};
use crate::private::Sealed;
use std::any::TypeId;
use std::fmt::Debug;
use std::marker::PhantomData;

/// Pixel [channel], [color model], [alpha] and [gamma] mode.
///
/// A pixel can be converted to another format using the [convert] method.
///
/// [alpha]: ../chan/trait.Alpha.html
/// [channel]: ../chan/trait.Channel.html
/// [color model]: ../clr/trait.ColorModel.html
/// [convert]: #method.convert
/// [gamma]: ../chan/trait.Gamma.html
///
/// ### Type Alias Naming Scheme
///
/// * _Gamma_: `S` for [sRGB] gamma encoding; [linear] if omitted.
/// * _Color model_: [`Rgb`] / [`Bgr`] / [`Gray`] / [`Hsv`] / [`Hsl`] /
///                  [`Hwb`] / [`YCbCr`] / [`Mask`].
/// * _Alpha_: `a` to include alpha channel enabling translucent pixels.
/// * _Bit depth_: `8` / `16` / `32` for 8-bit integer, 16-bit integer and
///   32-bit floating-point [channels].
/// * _Alpha mode_: `p` for [premultiplied]; [straight] if omitted.
///
/// [`bgr`]: ../clr/struct.Bgr.html
/// [channels]: ../chan/trait.Channel.html
/// [`gray`]: ../clr/struct.Gray.html
/// [`hsl`]: ../clr/struct.Hsl.html
/// [`hsv`]: ../clr/struct.Hsv.html
/// [`hwb`]: ../clr/struct.Hwb.html
/// [linear]: ../chan/struct.Linear.html
/// [`mask`]: ../clr/struct.Mask.html
/// [premultiplied]: ../chan/struct.Premultiplied.html
/// [`Rgb`]: ../clr/struct.Rgb.html
/// [sRGB]: ../chan/struct.Srgb.html
/// [straight]: ../chan/struct.Straight.html
/// [`YCbCr`]: ../clr/struct.YCbCr.html
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
    type Alpha: Alpha;

    /// Gamma mode
    type Gamma: Gamma;

    /// Make a pixel from a slice of channels.
    fn from_channels(ch: &[Self::Chan]) -> Self;

    /// Convert from a pixel with a different bit depth.
    fn from_bit_depth<P>(p: P) -> Self
    where
        P: Pixel,
        Self::Chan: From<P::Chan>;

    /// Get the channels.
    fn channels(&self) -> &[Self::Chan];

    /// Get the channels mutably.
    fn channels_mut(&mut self) -> &mut [Self::Chan];

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

    /// Get the *alpha* channel.
    fn alpha(self) -> Self::Chan {
        let chan = self.channels();
        chan[Self::Model::ALPHA]
    }

    /// Convert a pixel to another format
    ///
    /// * `D` Destination format.
    fn convert<D>(self) -> D
    where
        D: Pixel,
        D::Chan: From<Self::Chan>,
    {
        if TypeId::of::<Self::Model>() == TypeId::of::<D::Model>() {
            convert_same_model::<D, Self>(self)
        } else {
            convert_thru_rgba::<D, Self>(self)
        }
    }

    /// Convert channels to linear gamma
    fn to_linear_gamma<C: Channel>(channels: &mut [C]) {
        channels[Self::Model::LINEAR]
            .iter_mut()
            .for_each(|c| *c = Self::Gamma::to_linear(*c));
    }

    /// Convert channels from linear gamma
    fn from_linear_gamma(channels: &mut [Self::Chan]) {
        channels[Self::Model::LINEAR]
            .iter_mut()
            .for_each(|c| *c = Self::Gamma::from_linear(*c));
    }
}

/// Rgba pixel type for color model conversions
pub type PixRgba<P> =
    Pix4<<P as Pixel>::Chan, Rgb, <P as Pixel>::Alpha, <P as Pixel>::Gamma>;

/// Convert a pixel to another format with the same color model.
///
/// * `D` Destination pixel format.
/// * `S` Source pixel format.
/// * `src` Source pixel.
fn convert_same_model<D, S>(src: S) -> D
where
    D: Pixel,
    S: Pixel,
    D::Chan: From<S::Chan>,
{
    let mut dst = D::from_bit_depth(src);
    if TypeId::of::<S::Alpha>() != TypeId::of::<D::Alpha>()
        || TypeId::of::<S::Gamma>() != TypeId::of::<D::Gamma>()
    {
        let mut channels = dst.channels_mut();
        convert_alpha_gamma::<D, S>(&mut channels);
    }
    dst
}

/// Convert *alpha* / *gamma* to another pixel format
fn convert_alpha_gamma<D, S>(channels: &mut [D::Chan])
where
    D: Pixel,
    S: Pixel,
{
    S::to_linear_gamma(channels);
    if TypeId::of::<S::Alpha>() != TypeId::of::<D::Alpha>() {
        let alpha = channels[D::Model::ALPHA];
        for c in channels[D::Model::LINEAR].iter_mut() {
            *c = S::Alpha::decode(*c, alpha);
            *c = D::Alpha::encode(*c, alpha);
        }
    }
    D::from_linear_gamma(channels);
}

/// Convert a pixel to another format thru RGBA.
///
/// * `D` Destination pixel format.
/// * `S` Source pixel format.
/// * `src` Source pixel.
fn convert_thru_rgba<D, S>(src: S) -> D
where
    D: Pixel,
    S: Pixel,
    D::Chan: From<S::Chan>,
{
    let rgba = S::Model::into_rgba::<S>(src);
    let rgba = convert_same_model::<PixRgba<D>, PixRgba<S>>(rgba);
    D::Model::from_rgba::<D>(rgba)
}

/// [Pixel] with one [channel] in its [color model].
///
/// [channel]: ../chan/trait.Channel.html
/// [color model]: ../clr/trait.ColorModel.html
/// [pixel]: trait.Pixel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Pix1<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: Alpha,
    G: Gamma,
{
    channels: [C; 1],
    _model: PhantomData<M>,
    _alpha: PhantomData<A>,
    _gamma: PhantomData<G>,
}

impl<C, M, A, G> Pix1<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: Alpha,
    G: Gamma,
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
        let channels = [C::from(one); 1];
        Pix1 {
            channels,
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
    A: Alpha,
    G: Gamma,
{
    type Chan = C;
    type Model = M;
    type Alpha = A;
    type Gamma = G;

    fn from_channels(ch: &[C]) -> Self {
        let one = ch[0].into();
        Self::new(one)
    }

    fn from_bit_depth<P>(p: P) -> Self
    where
        P: Pixel,
        Self::Chan: From<P::Chan>,
    {
        if TypeId::of::<Self::Model>() != TypeId::of::<P::Model>() {
            panic!("Invalid pixel conversion");
        }
        let one = Self::Chan::from(p.one());
        Self::new(one)
    }

    fn channels(&self) -> &[Self::Chan] {
        &self.channels
    }

    fn channels_mut(&mut self) -> &mut [Self::Chan] {
        &mut self.channels
    }

    fn one(self) -> C {
        self.channels[0]
    }
}

/// [Pixel] with two [channel]s in its [color model].
///
/// [channel]: ../chan/trait.Channel.html
/// [color model]: ../clr/trait.ColorModel.html
/// [pixel]: trait.Pixel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Pix2<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: Alpha,
    G: Gamma,
{
    channels: [C; 2],
    _model: PhantomData<M>,
    _alpha: PhantomData<A>,
    _gamma: PhantomData<G>,
}

impl<C, M, A, G> Pix2<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: Alpha,
    G: Gamma,
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
        let channels = [one, two];
        Pix2 {
            channels,
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
    A: Alpha,
    G: Gamma,
{
    type Chan = C;
    type Model = M;
    type Alpha = A;
    type Gamma = G;

    fn from_channels(ch: &[C]) -> Self {
        let one = ch[0].into();
        let two = ch[1].into();
        Self::new(one, two)
    }

    fn from_bit_depth<P>(p: P) -> Self
    where
        P: Pixel,
        Self::Chan: From<P::Chan>,
    {
        if TypeId::of::<Self::Model>() != TypeId::of::<P::Model>() {
            panic!("Invalid pixel conversion");
        }
        let one = Self::Chan::from(p.one());
        let two = Self::Chan::from(p.two());
        Self::new(one, two)
    }

    fn channels(&self) -> &[Self::Chan] {
        &self.channels
    }

    fn channels_mut(&mut self) -> &mut [Self::Chan] {
        &mut self.channels
    }

    fn one(self) -> C {
        self.channels[0]
    }

    fn two(self) -> C {
        self.channels[1]
    }
}

/// [Pixel] with three [channel]s in its [color model].
///
/// [channel]: ../chan/trait.Channel.html
/// [color model]: ../clr/trait.ColorModel.html
/// [pixel]: trait.Pixel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Pix3<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: Alpha,
    G: Gamma,
{
    channels: [C; 3],
    _model: PhantomData<M>,
    _alpha: PhantomData<A>,
    _gamma: PhantomData<G>,
}

impl<C, M, A, G> Pix3<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: Alpha,
    G: Gamma,
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
        let channels = [one, two, three];
        Pix3 {
            channels,
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
    A: Alpha,
    G: Gamma,
{
    type Chan = C;
    type Model = M;
    type Alpha = A;
    type Gamma = G;

    fn from_channels(ch: &[C]) -> Self {
        let one = ch[0].into();
        let two = ch[1].into();
        let three = ch[2].into();
        Self::new(one, two, three)
    }

    fn from_bit_depth<P>(p: P) -> Self
    where
        P: Pixel,
        Self::Chan: From<P::Chan>,
    {
        if TypeId::of::<Self::Model>() != TypeId::of::<P::Model>() {
            panic!("Invalid pixel conversion");
        }
        let one = Self::Chan::from(p.one());
        let two = Self::Chan::from(p.two());
        let three = Self::Chan::from(p.three());
        Self::new(one, two, three)
    }

    fn channels(&self) -> &[Self::Chan] {
        &self.channels
    }

    fn channels_mut(&mut self) -> &mut [Self::Chan] {
        &mut self.channels
    }

    fn one(self) -> C {
        self.channels[0]
    }

    fn two(self) -> C {
        self.channels[1]
    }

    fn three(self) -> C {
        self.channels[2]
    }
}

/// [Pixel] with four [channel]s in its [color model].
///
/// [channel]: ../chan/trait.Channel.html
/// [color model]: ../clr/trait.ColorModel.html
/// [pixel]: trait.Pixel.html
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Pix4<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: Alpha,
    G: Gamma,
{
    channels: [C; 4],
    _model: PhantomData<M>,
    _alpha: PhantomData<A>,
    _gamma: PhantomData<G>,
}

impl<C, M, A, G> Pix4<C, M, A, G>
where
    C: Channel,
    M: ColorModel,
    A: Alpha,
    G: Gamma,
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
        let channels = [one, two, three, four];
        Pix4 {
            channels,
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
    A: Alpha,
    G: Gamma,
{
    type Chan = C;
    type Model = M;
    type Alpha = A;
    type Gamma = G;

    fn from_channels(ch: &[C]) -> Self {
        let one = ch[0].into();
        let two = ch[1].into();
        let three = ch[2].into();
        let four = ch[3].into();
        Self::new(one, two, three, four)
    }

    fn from_bit_depth<P>(p: P) -> Self
    where
        P: Pixel,
        Self::Chan: From<P::Chan>,
    {
        if TypeId::of::<Self::Model>() != TypeId::of::<P::Model>() {
            panic!("Invalid pixel conversion");
        }
        let one = Self::Chan::from(p.one());
        let two = Self::Chan::from(p.two());
        let three = Self::Chan::from(p.three());
        let four = Self::Chan::from(p.four());
        Self::new(one, two, three, four)
    }

    fn channels(&self) -> &[Self::Chan] {
        &self.channels
    }

    fn channels_mut(&mut self) -> &mut [Self::Chan] {
        &mut self.channels
    }

    fn one(self) -> C {
        self.channels[0]
    }

    fn two(self) -> C {
        self.channels[1]
    }

    fn three(self) -> C {
        self.channels[2]
    }

    fn four(self) -> C {
        self.channels[3]
    }
}

#[cfg(test)]
mod test {
    use crate::el::*;
    use crate::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<Mask8>(), 1);
        assert_eq!(std::mem::size_of::<Mask16>(), 2);
        assert_eq!(std::mem::size_of::<Mask32>(), 4);
        assert_eq!(std::mem::size_of::<SGray8>(), 1);
        assert_eq!(std::mem::size_of::<SGray16>(), 2);
        assert_eq!(std::mem::size_of::<SGray32>(), 4);
        assert_eq!(std::mem::size_of::<SGraya8>(), 2);
        assert_eq!(std::mem::size_of::<SGraya16>(), 4);
        assert_eq!(std::mem::size_of::<SGraya32>(), 8);
        assert_eq!(std::mem::size_of::<Rgb8>(), 3);
        assert_eq!(std::mem::size_of::<Rgb16>(), 6);
        assert_eq!(std::mem::size_of::<Rgb32>(), 12);
        assert_eq!(std::mem::size_of::<Rgba8>(), 4);
        assert_eq!(std::mem::size_of::<Rgba16>(), 8);
        assert_eq!(std::mem::size_of::<Rgba32>(), 16);
    }

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
