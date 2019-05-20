// alpha.rs     Alpha channel handling.
//
// Copyright (c) 2019  Douglas P Lau
//
use crate::{Channel, Ch8, Ch16, Ch32};
use std::marker::PhantomData;

/// [Channel](trait.Channel.html) for defining the opacity of pixels.
///
/// It is the inverse of translucency.
pub trait Alpha: Copy + Default {

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
    where C: Channel + From<H>, H: Channel
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

impl<C, A> From<Translucent<A>> for Opaque<C> where C: Channel, A: Channel {
    /// Convert from a `Translucent` value.
    fn from(_: Translucent<A>) -> Self {
        Opaque::default()
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
    where C: Channel + From<H>, H: Channel
{
    fn from(value: H) -> Self {
        let value = value.into();
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
    where C: Channel, A: Channel
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

/// Mode for handling associated alpha.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlphaMode {
    /// Each `Channel` is associated, or premultiplied, with alpha
    Associated,
    /// Each `Channel` is separated from alpha (not premultiplied)
    Separated,
}

impl AlphaMode {
    /// Encode a `Channel` value using the alpha mode.
    pub fn encode<C>(self, c: C, a: C) -> C
        where C: Channel
    {
        match self {
            AlphaMode::Associated => c * a,
            AlphaMode::Separated => c,
        }
    }
    /// Decode a `Channel` value using the alpha mode.
    pub fn decode<C>(self, c: C, a: C) -> C
        where C: Channel
    {
        match self {
            AlphaMode::Associated => c / a,
            AlphaMode::Separated=> c,
        }
    }
}
