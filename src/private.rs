// private.rs     Private sealed trait
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::alpha::{self, Opaque, Translucent};
use crate::gamma;
use crate::Channel;

/// Sealed trait to prevent outside crates from implementing traits
pub trait Sealed {}

impl<C> Sealed for Opaque<C> {}

impl<C: Channel> Sealed for Translucent<C> {}

impl Sealed for alpha::Straight {}

impl Sealed for alpha::Premultiplied {}

impl Sealed for gamma::Linear {}

impl Sealed for gamma::Srgb {}
