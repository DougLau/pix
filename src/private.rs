// private.rs     Private sealed trait
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::alpha::{self, Alpha, Opaque, Translucent};
use crate::gamma;
use crate::{Channel, Gray, Mask, Rgb};

/// Sealed trait to prevent outside crates from implementing traits
pub trait Sealed {}

impl<C: Channel> Sealed for Translucent<C> {}

impl<C> Sealed for Opaque<C> {}

impl Sealed for alpha::Straight {}

impl Sealed for alpha::Premultiplied {}

impl<C: Channel, A: Alpha, M: alpha::Mode, G: gamma::Mode> Sealed
    for Gray<C, A, M, G>
{
}

impl<A: Alpha> Sealed for Mask<A> {}

impl<C: Channel, A: Alpha, M: alpha::Mode, G: gamma::Mode> Sealed
    for Rgb<C, A, M, G>
{
}

impl Sealed for gamma::Linear {}

impl Sealed for gamma::Srgb {}
