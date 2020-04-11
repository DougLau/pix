// ops.rs       Compositing operations
//
// Copyright (c) 2020  Douglas P Lau
//
//! Porter-Duff compositing operations
use crate::chan::Channel;

/// Porter-Duff compositing operation.
///
/// This trait is *sealed*, and cannot be implemented outside of this crate.
pub trait PorterDuff: Copy + Clone {
    /// Composite a destination and source
    ///
    /// * `dst` Destination channel
    /// * `da1` One minus destination *alpha*
    /// * `src` Source channel
    /// * `sa1` One minus source *alpha*
    fn composite<C: Channel>(dst: &mut C, da1: C, src: &C, sa1: C);
}

/// Source only (ignore destination)
#[derive(Clone, Copy)]
pub struct Src;

/// Destination only (ignore source)
#[derive(Clone, Copy)]
pub struct Dest;

/// Source Over compositing (standard *alpha* blending)
#[derive(Clone, Copy)]
pub struct SrcOver;

/// Destination Over compositing (*alpha* blending behind destination)
#[derive(Clone, Copy)]
pub struct DestOver;

/// Source Out compositing (remove destination from source)
#[derive(Clone, Copy)]
pub struct SrcOut;

/// Destination Out compositing (remove source from destination)
#[derive(Clone, Copy)]
pub struct DestOut;

/// Source In compositing (mask source with destination *alpha*)
#[derive(Clone, Copy)]
pub struct SrcIn;

/// Destination In compositing (mask destination with source *alpha*)
#[derive(Clone, Copy)]
pub struct DestIn;

/// Source Atop compositing (overlay and mask source atop destination)
#[derive(Clone, Copy)]
pub struct SrcAtop;

/// Destination Atop compositing (overlay and mask destination atop source)
#[derive(Clone, Copy)]
pub struct DestAtop;

/// Xor compositing (source or destination with no overlap)
#[derive(Clone, Copy)]
pub struct Xor;

impl PorterDuff for Src {
    fn composite<C: Channel>(dst: &mut C, _da1: C, src: &C, _sa1: C) {
        *dst = *src;
    }
}

impl PorterDuff for Dest {
    fn composite<C: Channel>(_dst: &mut C, _da1: C, _src: &C, _sa1: C) {
        // leave _dst as is
    }
}

impl PorterDuff for SrcOver {
    fn composite<C: Channel>(dst: &mut C, _da1: C, src: &C, sa1: C) {
        *dst = *src + *dst * sa1;
    }
}

impl PorterDuff for DestOver {
    fn composite<C: Channel>(dst: &mut C, da1: C, src: &C, _sa1: C) {
        *dst = *src * da1 + *dst;
    }
}

impl PorterDuff for SrcOut {
    fn composite<C: Channel>(dst: &mut C, da1: C, src: &C, _sa1: C) {
        *dst = *src * da1;
    }
}

impl PorterDuff for DestOut {
    fn composite<C: Channel>(dst: &mut C, _da1: C, _src: &C, sa1: C) {
        *dst = *dst * sa1;
    }
}

impl PorterDuff for SrcIn {
    fn composite<C: Channel>(dst: &mut C, da1: C, src: &C, _sa1: C) {
        let da = C::MAX - da1;
        *dst = *src * da;
    }
}

impl PorterDuff for DestIn {
    fn composite<C: Channel>(dst: &mut C, _da1: C, _src: &C, sa1: C) {
        let sa = C::MAX - sa1;
        *dst = *dst * sa;
    }
}

impl PorterDuff for SrcAtop {
    fn composite<C: Channel>(dst: &mut C, da1: C, src: &C, sa1: C) {
        let da = C::MAX - da1;
        *dst = *src * da + *dst * sa1;
    }
}

impl PorterDuff for DestAtop {
    fn composite<C: Channel>(dst: &mut C, da1: C, src: &C, sa1: C) {
        let sa = C::MAX - sa1;
        *dst = *src * da1 + *dst * sa;
    }
}

impl PorterDuff for Xor {
    fn composite<C: Channel>(dst: &mut C, da1: C, src: &C, sa1: C) {
        *dst = *src * da1 + *dst * sa1;
    }
}
