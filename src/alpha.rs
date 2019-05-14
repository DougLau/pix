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

    /// Channel type
    type Chan: Channel;

    /// Get the alpha channel value.
    ///
    /// [Channel::MIN](trait.Channel.html#associatedconstant.MIN) is fully
    /// transparent, and
    /// [Channel::MAX](trait.Channel.html#associatedconstant.MAX) is fully
    /// opaque.
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

/// Convert an Opaque to another Opaque with a different Channel
macro_rules! from_impl_opaque {
    ( $s:tt, $d:tt ) => {
        impl From<Opaque<$s>> for Opaque<$d> {
            fn from(_: Opaque<$s>) -> Self { Opaque::default() }
        }
    };
}

from_impl_opaque!(Ch8, Ch16);
from_impl_opaque!(Ch8, Ch32);
from_impl_opaque!(Ch16, Ch8);
from_impl_opaque!(Ch16, Ch32);
from_impl_opaque!(Ch32, Ch8);
from_impl_opaque!(Ch32, Ch16);

impl<C, A> From<Translucent<A>> for Opaque<C> where C: Channel, A: Channel {
    /// Convert from a translucent value.
    fn from(_: Translucent<A>) -> Self {
        Opaque::default()
    }
}

impl<C: Channel> Alpha for Opaque<C> {
    type Chan = C;

    /// Get the alpha channel value.
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

impl<C: Channel> Translucent<C> {
    /// Create a new translucent alpha value.
    pub fn new(value: C) -> Self {
        Translucent { value }
    }
}

impl<C, H> From<H> for Translucent<C>
    where C: Channel, C: From<H>, H: Channel
{
    /// Convert from a channel value.
    fn from(value: H) -> Self {
        let value = value.into();
        Translucent { value }
    }
}

impl<C> From<u8> for Translucent<C> where C: Channel, C: From<u8> {
    fn from(value: u8) -> Self {
        Translucent::new(value.into())
    }
}
impl<C> From<u16> for Translucent<C> where C: Channel, C: From<u16> {
    fn from(value: u16) -> Self {
        Translucent::new(value.into())
    }
}

/// Convert a Translucent to another Translucent with a different Channel
macro_rules! from_impl_translucent {
    ( $s:tt, $d:tt ) => {
        impl From<Translucent<$s>> for Translucent<$d> {
            fn from(t: Translucent<$s>) -> Self {
                Translucent::new(t.value.into())
            }
        }
    };
}

from_impl_translucent!(Ch8, Ch16);
from_impl_translucent!(Ch8, Ch32);
from_impl_translucent!(Ch16, Ch8);
from_impl_translucent!(Ch16, Ch32);
from_impl_translucent!(Ch32, Ch8);
from_impl_translucent!(Ch32, Ch16);

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
    /// [Channel::MIN](trait.Channel.html#associatedconstant.MIN) is fully
    /// transparent, and
    /// [Channel::MAX](trait.Channel.html#associatedconstant.MAX) is fully
    /// opaque.
    fn value(&self) -> C {
        self.value
    }
}
