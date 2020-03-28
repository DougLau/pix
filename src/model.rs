// model.rs     Color models
//
// Copyright (c) 2020  Douglas P Lau
//
//! Module for color model items
use crate::alpha::Mode as _;
use crate::channel::Channel;
use crate::gamma::Mode as _;
use crate::el::Pixel;
use crate::private::Sealed;
use std::any::TypeId;
use std::fmt::Debug;

/// Channels making up a color.
///
/// All channels before *alpha_idx* will be adjusted by *alpha*/*gamma* during
/// conversion; *alpha* and later channels will not.
#[derive(Debug)]
pub struct Channels<C: Channel> {
    channels: [C; 4],
    alpha_idx: usize,
}

/// Model for pixel colors.
///
/// Existing color models are [Rgb], [Gray], [Hsv], [Hsl], [Hwb], [YCbCr] and
/// [Mask].
///
/// It is possible to convert from a color model to any other, using
/// [into_channels] and [from_channels].  For usage of this, see the `Pixel`
/// [convert] method.
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
///
/// [convert]: el/trait.Pixel.html#method.convert
/// [from_channels]: trait.ColorModel.html#method.from_channels
/// [gray]: struct.Gray.html
/// [hsl]: struct.Hsl.html
/// [hsv]: struct.Hsv.html
/// [hwb]: struct.Hwb.html
/// [into_channels]: trait.ColorModel.html#method.into_channels
/// [mask]: struct.Mask.html
/// [rgb]: struct.Rgb.html
/// [ycbcr]: struct.YCbCr.html
pub trait ColorModel: Clone + Copy + Debug + Default + PartialEq + Sealed {
    /// Get the *alpha* component.
    fn alpha<P: Pixel>(p: P) -> P::Chan
    where
        P: Pixel<Model = Self>;

    /// Convert into channels shared by pixel types
    fn into_channels<S, D>(src: S) -> Channels<S::Chan>
    where
        S: Pixel<Model = Self>,
        D: Pixel;

    /// Convert into *red*, *green*, *blue* and *alpha* components
    fn into_rgba<P>(p: P) -> [P::Chan; 4]
    where
        P: Pixel<Model = Self>;

    /// Convert from channels shared by pixel types
    fn from_channels<S: Pixel, D: Pixel>(channels: Channels<D::Chan>) -> D;

    /// Convert from *red*, *green*, *blue* and *alpha* components
    fn from_rgba<P: Pixel>(rgba: [P::Chan; 4]) -> P;
}

impl<C: Channel> Channels<C> {
    /// Create new channels
    pub fn new(channels: [C; 4], alpha_idx: usize) -> Self {
        Channels { channels, alpha_idx }
    }
    /// Get alpha index
    pub fn alpha_idx(&self) -> usize {
        self.alpha_idx
    }
    /// Convert channels into an array
    pub fn into_array(self) -> [C; 4] {
        self.channels
    }
    /// Convert to destination bit depth
    fn into_bit_depth<D>(self) -> Channels<D>
    where
        D: Channel + From<C>,
    {
        let chan = [
            D::from(self.channels[0]),
            D::from(self.channels[1]),
            D::from(self.channels[2]),
            D::from(self.channels[3]),
        ];
        Channels::<D>::new(chan, self.alpha_idx)
    }
    /// Convert channels from source to destination pixel format
    pub fn convert<S, D>(self) -> Channels<D::Chan>
    where
        S: Pixel,
        D: Pixel,
        D::Chan: From<C>,
    {
        let mut dst = self.into_bit_depth::<D::Chan>();
        if TypeId::of::<S::Alpha>() != TypeId::of::<D::Alpha>()
            || TypeId::of::<S::Gamma>() != TypeId::of::<D::Gamma>()
        {
            dst.convert_alpha_gamma::<S, D>();
        }
        dst
    }
    /// Convert *alpha*/*gamma* between two pixel formats
    fn convert_alpha_gamma<S, D>(&mut self)
    where
        S: Pixel,
        D: Pixel,
    {
        let (channels, later) = self.channels.split_at_mut(self.alpha_idx);
        let alpha = later[0];
        // Convert to linear gamma
        channels
            .iter_mut()
            .for_each(|c| *c = S::Gamma::to_linear(*c));
        if TypeId::of::<S::Alpha>() != TypeId::of::<D::Alpha>() {
            for c in channels.iter_mut() {
                // Decode source alpha
                *c = S::Alpha::decode(*c, alpha);
                // Encode destination alpha
                *c = D::Alpha::encode(*c, alpha);
            }
        }
        // Convert to destination gamma
        channels
            .iter_mut()
            .for_each(|c| *c = D::Gamma::from_linear(*c));
    }
}
