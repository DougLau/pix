// ops.rs       Compositing operations
//
// Copyright (c) 2020  Douglas P Lau
//
//! Porter-Duff compositing operations
use crate::chan::Premultiplied;
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
    fn composite_slice<P>(dst: &mut [P], src: &[P])
    where
        P: Pixel;
}

/// Source compositing (copy source to destination)
pub struct Src;

/// Source Over compositing (standard alpha blending)
pub struct SrcOver;

impl PorterDuff for Src {
    fn composite_color<P>(dst: &mut [P], clr: P)
    where
        P: Pixel,
    {
        P::composite_color(dst, &clr, |d, s, _a1| *d = *s);
    }

    fn composite_slice<P>(dst: &mut [P], src: &[P])
    where
        P: Pixel,
    {
        P::composite_slice(dst, src, |d, s, _a1| *d = *s);
    }
}

impl PorterDuff for SrcOver {
    fn composite_color<P>(dst: &mut [P], clr: P)
    where
        P: Pixel,
    {
        if TypeId::of::<P::Alpha>() == TypeId::of::<Premultiplied>() {
            P::composite_color(dst, &clr, |d, s, a1| *d = *s + *d * *a1);
        } else {
            todo!();
        }
    }

    fn composite_slice<P>(dst: &mut [P], src: &[P])
    where
        P: Pixel,
    {
        if TypeId::of::<P::Alpha>() == TypeId::of::<Premultiplied>() {
            P::composite_slice(dst, src, |d, s, a1| *d = *s + *d * *a1);
        } else {
            todo!();
        }
    }
}
