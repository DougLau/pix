// ops.rs       Compositing operations
//
// Copyright (c) 2020  Douglas P Lau
//
//! Porter-Duff compositing operations
use crate::el::Pixel;
use crate::private::Sealed;

/// Porter-Duff compositing operation.
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
pub trait PorterDuff: Sealed {
    /// Composite source color to destination pixel slice
    fn composite_color<P>(dst: &mut [P], clr: P)
    where
        P: Pixel;

    /// Composite source and destination pixel slices
    fn composite<D, S>(dst: &mut [D], src: &[S])
    where
        D: Pixel,
        S: Pixel,
        D::Chan: From<S::Chan>;
}

/// Source compositing (copy source to destination)
pub struct Source;

impl PorterDuff for Source {
    fn composite_color<P>(dst: &mut [P], clr: P)
    where
        P: Pixel,
    {
        for d in dst.iter_mut() {
            *d = clr;
        }
    }

    fn composite<D, S>(dst: &mut [D], src: &[S])
    where
        D: Pixel,
        S: Pixel,
        D::Chan: From<S::Chan>
    {
        for (d, s) in dst.iter_mut().zip(src) {
            *d = s.convert();
        }
    }
}
