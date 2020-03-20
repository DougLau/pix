// private.rs     Private sealed trait
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::alpha::{self, AChannel, Opaque, Translucent};
use crate::gamma;
use crate::{Ch16, Ch32, Ch8, Channel, Gray, Hsl, Hsv, Hwb, Mask, Rgb, YCbCr};

/// Sealed trait to prevent outside crates from implementing traits
pub trait Sealed {}

impl<C: Channel> Sealed for Opaque<C> {}

impl<C: Channel> Sealed for Translucent<C> {}

impl Sealed for alpha::Straight {}

impl Sealed for alpha::Premultiplied {}

impl Sealed for gamma::Linear {}

impl Sealed for gamma::Srgb {}

impl Sealed for Ch8 {}

impl Sealed for Ch16 {}

impl Sealed for Ch32 {}

impl Sealed for u8 {}

impl Sealed for u16 {}

impl Sealed for f32 {}

impl Sealed for f64 {}

impl<C: Channel> Sealed for Mask<C> {}

impl<C, A, M, G> Sealed for Gray<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
}

impl<C, A, M, G> Sealed for Rgb<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
}

impl<C, A, M, G> Sealed for Hsl<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
}

impl<C, A, M, G> Sealed for Hsv<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
}

impl<C, A, M, G> Sealed for Hwb<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
}

impl<C, A, M, G> Sealed for YCbCr<C, A, M, G>
where
    C: Channel,
    A: AChannel<Chan = C>,
    M: alpha::Mode,
    G: gamma::Mode,
{
}
