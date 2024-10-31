//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLOrigin {
    pub x: NSUInteger,
    pub y: NSUInteger,
    pub z: NSUInteger,
}

unsafe impl Encode for MTLOrigin {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <NSUInteger>::ENCODING,
            <NSUInteger>::ENCODING,
            <NSUInteger>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLOrigin {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// TODO: pub fn MTLOriginMake(x: NSUInteger,y: NSUInteger,z: NSUInteger,) -> MTLOrigin;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLSize {
    pub width: NSUInteger,
    pub height: NSUInteger,
    pub depth: NSUInteger,
}

unsafe impl Encode for MTLSize {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <NSUInteger>::ENCODING,
            <NSUInteger>::ENCODING,
            <NSUInteger>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// TODO: pub fn MTLSizeMake(width: NSUInteger,height: NSUInteger,depth: NSUInteger,) -> MTLSize;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLRegion {
    pub origin: MTLOrigin,
    pub size: MTLSize,
}

unsafe impl Encode for MTLRegion {
    const ENCODING: Encoding = Encoding::Struct("?", &[<MTLOrigin>::ENCODING, <MTLSize>::ENCODING]);
}

unsafe impl RefEncode for MTLRegion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// TODO: pub fn MTLRegionMake1D(x: NSUInteger,width: NSUInteger,) -> MTLRegion;

// TODO: pub fn MTLRegionMake2D(x: NSUInteger,y: NSUInteger,width: NSUInteger,height: NSUInteger,) -> MTLRegion;

// TODO: pub fn MTLRegionMake3D(x: NSUInteger,y: NSUInteger,z: NSUInteger,width: NSUInteger,height: NSUInteger,depth: NSUInteger,) -> MTLRegion;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLSamplePosition {
    pub x: c_float,
    pub y: c_float,
}

unsafe impl Encode for MTLSamplePosition {
    const ENCODING: Encoding = Encoding::Struct("?", &[<c_float>::ENCODING, <c_float>::ENCODING]);
}

unsafe impl RefEncode for MTLSamplePosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// TODO: pub fn MTLSamplePositionMake(x: c_float,y: c_float,) -> MTLSamplePosition;

pub type MTLCoordinate2D = MTLSamplePosition;

// TODO: pub fn MTLCoordinate2DMake(x: c_float,y: c_float,) -> MTLCoordinate2D;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLResourceID {
    _impl: u64,
}

unsafe impl Encode for MTLResourceID {
    const ENCODING: Encoding = Encoding::Struct("MTLResourceID", &[<u64>::ENCODING]);
}

unsafe impl RefEncode for MTLResourceID {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
