// alpha.rs     Alpha channel handling.
//
// Copyright (c) 2019  Douglas P Lau
//
use crate::{Ch16, Ch32, Ch8, Channel};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Mul;

/// [Channel](trait.Channel.html) for defining the opacity of pixels.
///
/// It is the inverse of translucency.
pub trait Alpha:
    Copy + Debug + Default + PartialEq + Mul<Output = Self>
{
    /// `Channel` type
    type Chan: Channel;

    /// Get the alpha `Channel` value.
    ///
    /// [Channel::MIN](trait.Channel.html#associatedconstant.MIN) is fully
    /// transparent, and
    /// [Channel::MAX](trait.Channel.html#associatedconstant.MAX) is fully
    /// opaque.
    fn value(&self) -> Self::Chan;
}

/// [Alpha](trait.Alpha.html) `Channel` for fully opaque pixels and
/// [Raster](struct.Raster.html)s.
///
/// Pixel [Format](trait.Format.html)s with `Opaque` alpha channels take less
/// memory than those with [translucent](struct.Translucent.html) ones.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Opaque<C> {
    value: PhantomData<C>,
}

impl<C, H> From<H> for Opaque<C>
where
    C: Channel + From<H>,
    H: Channel,
{
    fn from(_value: H) -> Self {
        Opaque::default()
    }
}
impl<C: Channel> From<Opaque<C>> for Ch8 {
    fn from(_value: Opaque<C>) -> Self {
        Ch8::MAX
    }
}
impl<C: Channel> From<Opaque<C>> for Ch16 {
    fn from(_value: Opaque<C>) -> Self {
        Ch16::MAX
    }
}
impl<C: Channel> From<Opaque<C>> for Ch32 {
    fn from(_value: Opaque<C>) -> Self {
        Ch32::MAX
    }
}

impl<C, A> From<Translucent<A>> for Opaque<C>
where
    C: Channel,
    A: Channel,
{
    /// Convert from a `Translucent` value.
    fn from(_: Translucent<A>) -> Self {
        Opaque::default()
    }
}

impl<C: Channel> Mul<Self> for Opaque<C> {
    type Output = Self;
    fn mul(self, _rhs: Self) -> Self {
        self
    }
}

impl<C: Channel> Alpha for Opaque<C> {
    type Chan = C;

    /// Get the alpha `Channel` value.
    ///
    /// Always returns
    /// [Channel::MAX](trait.Channel.html#associatedconstant.MAX) (fully
    /// opaque).
    fn value(&self) -> C {
        C::MAX
    }
}

/// [Alpha](trait.Alpha.html) channel for translucent or transparent pixels and
/// [Raster](struct.Raster.html)s.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Translucent<C: Channel> {
    value: C,
}

impl<C, H> From<H> for Translucent<C>
where
    C: Channel + From<H>,
    H: Channel,
{
    fn from(value: H) -> Self {
        let value = value.into();
        Translucent { value }
    }
}
impl From<u8> for Translucent<Ch8> {
    fn from(value: u8) -> Self {
        Ch8::new(value).into()
    }
}
impl From<u16> for Translucent<Ch16> {
    fn from(value: u16) -> Self {
        Ch16::new(value).into()
    }
}
impl From<f32> for Translucent<Ch32> {
    fn from(value: f32) -> Self {
        Ch32::new(value).into()
    }
}

impl<C: Channel> Mul<Self> for Translucent<C> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let value = self.value * rhs.value;
        Translucent { value }
    }
}

impl<C: Channel> Translucent<C> {
    /// Create a new `Translucent` alpha value.
    pub fn new(value: C) -> Self {
        Translucent { value }
    }
}

impl<C, A> From<Opaque<A>> for Translucent<C>
where
    C: Channel,
    A: Channel,
{
    /// Convert from an `Opaque` value.
    fn from(_: Opaque<A>) -> Self {
        Self::new(C::MAX)
    }
}

impl<C: Channel> Alpha for Translucent<C> {
    type Chan = C;

    /// Get the alpha `Channel` value.
    ///
    /// [Channel::MIN](trait.Channel.html#associatedconstant.MIN) is fully
    /// transparent, and
    /// [Channel::MAX](trait.Channel.html#associatedconstant.MAX) is fully
    /// opaque.
    fn value(&self) -> C {
        self.value
    }
}

/// Each `Channel` is associated, or premultiplied, with alpha
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Associated;

/// Each `Channel` is separated from alpha (not premultiplied)
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Separated;

/// Unknown
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct UnknownAlpha;

/// Trait for handling associated versus separated alpha
pub trait AlphaMode: Copy + Clone + Debug + PartialEq + Default {
    const ID: AlphaModeID;

    /// Encode one `Channel` using the alpha mode.
    fn encode<C: Channel, A: Alpha<Chan = C>>(c: C, a: A) -> C;
    /// Decode one `Channel` using the alpha mode.
    fn decode<C: Channel, A: Alpha<Chan = C>>(c: C, a: A) -> C;
}

impl AlphaMode for Associated {
    const ID: AlphaModeID = AlphaModeID::Associated;

    /// Encode one `Channel` using the alpha mode.
    fn encode<C: Channel, A: Alpha<Chan = C>>(c: C, a: A) -> C {
        c * a.value()
    }
    /// Decode one `Channel` using the alpha mode.
    fn decode<C: Channel, A: Alpha<Chan = C>>(c: C, a: A) -> C {
        c / a.value()
    }
}

impl AlphaMode for Separated {
    const ID: AlphaModeID = AlphaModeID::Separated;

    /// Encode one `Channel` using the alpha mode.
    fn encode<C: Channel, A: Alpha<Chan = C>>(c: C, _a: A) -> C {
        c
    }
    /// Decode one `Channel` using the alpha mode.
    fn decode<C: Channel, A: Alpha<Chan = C>>(c: C, _a: A) -> C {
        c
    }
}

impl AlphaMode for UnknownAlpha {
    const ID: AlphaModeID = AlphaModeID::UnknownAlpha;

    /// Encode one `Channel` using the alpha mode.
    fn encode<C: Channel, A: Alpha<Chan = C>>(c: C, _a: A) -> C {
        c
    }
    /// Decode one `Channel` using the alpha mode.
    fn decode<C: Channel, A: Alpha<Chan = C>>(c: C, _a: A) -> C {
        c
    }
}

/// Mode for handling associated alpha.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlphaModeID {
    /// Each `Channel` is associated, or premultiplied, with alpha
    Associated,
    /// Each `Channel` is separated from alpha (not premultiplied)
    Separated,
    /// Unknown
    UnknownAlpha,
}
