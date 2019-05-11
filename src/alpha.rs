// alpha.rs     Alpha channel handling.
//
// Copyright (c) 2019  Douglas P Lau
//
use crate::channel::Channel;
use std::marker::PhantomData;

/// [Channel](trait.Channel.html) for defining the opacity of pixels.
///
/// It is the inverse of translucency.
pub trait Alpha: Copy + Default {

    /// Channel type
    type Chan;

    /// Get the alpha channel value.
    ///
    /// *Zero* is fully transparent, and *one* is fully opaque.
    fn value(&self) -> Self::Chan;
}

/// [Alpha](trait.Alpha.html) channel for fully opaque pixels and
/// [Raster](struct.Raster.html)s.
///
/// Pixel [Format](trait.Format.html)s with opaque alpha channels take less
/// memory than those with [translucent](struct.Translucent.html) ones.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Opaque<C> {
    value: PhantomData<C>,
}

impl<C: Channel> From<C> for Opaque<C> {
    /// Convert from a channel value.
    fn from(_: C) -> Self {
        Opaque::default()
    }
}

impl<C: Channel> Alpha for Opaque<C> {
    type Chan = C;

    /// Get the alpha channel value.
    ///
    /// *Zero* is fully transparent, and *one* is fully opaque.
    fn value(&self) -> C {
        C::MAX
    }
}

impl<C, A> From<Translucent<A>> for Opaque<C>
    where C: Channel, C: From<Translucent<A>>, A: Channel
{
    /// Convert from a translucent value.
    fn from(_: Translucent<A>) -> Self {
        Opaque::default()
    }
}

/// [Alpha](trait.Alpha.html) channel for translucent or transparent pixels and
/// [Raster](struct.Raster.html)s.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Translucent<C: Channel> {
    value: C,
}

impl<C: Channel> Translucent<C> {
    /// Create a new translucent alpha value.
    pub fn new(value: C) -> Self {
        Translucent { value }
    }
}

impl<C: Channel, H: Channel> From<H> for Translucent<C>
    where C: From<H>
{
    /// Convert from a channel value.
    fn from(value: H) -> Self {
        let value = value.into();
        Translucent { value }
    }
}

impl<C, A> From<Opaque<A>> for Translucent<C>
    where C: Channel, A: Channel
{
    /// Convert from an opaque value.
    fn from(_: Opaque<A>) -> Self {
        Self::new(C::MAX)
    }
}

impl<C: Channel> Alpha for Translucent<C> {
    type Chan = C;

    /// Get the alpha channel value.
    ///
    /// *Zero* is fully transparent, and *one* is fully opaque.
    fn value(&self) -> C {
        self.value
    }
}
