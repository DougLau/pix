// ops.rs       Compositing operations
//
// Copyright (c) 2020  Douglas P Lau
//
//! Porter-Duff compositing operations
use crate::chan::{Channel, Premultiplied};
use crate::clr::ColorModel;
use crate::el::Pixel;
use crate::private::Sealed;
use std::any::TypeId;

/// Porter-Duff compositing operation.
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
pub trait PorterDuff: Sealed {
    /// Composite source color to destination pixel slice
    fn composite_color<P>(dst: &mut [P], clr: P)
    where
        P: Pixel;

    /// Composite source and destination pixel slices
    fn composite<P>(dst: &mut [P], src: &[P])
    where
        P: Pixel;
}

/// Source compositing (copy source to destination)
pub struct Source;

/// Source Over compositing (standard alpha blending)
pub struct SourceOver;

impl PorterDuff for Source {
    fn composite_color<P>(dst: &mut [P], clr: P)
    where
        P: Pixel,
    {
        for d in dst.iter_mut() {
            *d = clr;
        }
    }

    fn composite<P>(dst: &mut [P], src: &[P])
    where
        P: Pixel,
    {
        for (d, s) in dst.iter_mut().zip(src) {
            *d = *s;
        }
    }
}

impl PorterDuff for SourceOver {
    fn composite_color<P>(dst: &mut [P], clr: P)
    where
        P: Pixel,
    {
        if TypeId::of::<P::Alpha>() == TypeId::of::<Premultiplied>() {
            for d in dst.iter_mut() {
                source_over_premultiplied(d, &clr);
            }
        } else {
            for d in dst.iter_mut() {
                source_over_straight(d, &clr);
            }
        }
    }

    fn composite<P>(dst: &mut [P], src: &[P])
    where
        P: Pixel,
    {
        if TypeId::of::<P::Alpha>() == TypeId::of::<Premultiplied>() {
            for (d, s) in dst.iter_mut().zip(src) {
                source_over_premultiplied(d, s);
            }
        } else {
            for (d, s) in dst.iter_mut().zip(src) {
                source_over_straight(d, s);
            }
        }
    }
}

fn source_over_premultiplied<P: Pixel>(dst: &mut P, src: &P) {
    let one_minus_src_a = P::Chan::MAX - src.alpha();
    let s_chan = &src.channels()[P::Model::LINEAR];
    let d_chan = &mut dst.channels_mut()[P::Model::LINEAR];
    d_chan.iter_mut()
        .zip(s_chan)
        .for_each(|(d, s)| *d = *s + *d * one_minus_src_a);
    // FIXME: composite circular channels
    let sa = &src.channels()[P::Model::ALPHA];
    let da = &mut dst.channels_mut()[P::Model::ALPHA];
    *da = *sa + *da * one_minus_src_a;
}

fn source_over_straight<P: Pixel>(_dst: &mut P, _src: &P) {
    todo!();
}
