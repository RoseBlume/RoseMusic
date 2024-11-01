//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTouchPhase(pub NSUInteger);
bitflags::bitflags! {
    impl NSTouchPhase: NSUInteger {
        #[doc(alias = "NSTouchPhaseBegan")]
        const Began = 1<<0;
        #[doc(alias = "NSTouchPhaseMoved")]
        const Moved = 1<<1;
        #[doc(alias = "NSTouchPhaseStationary")]
        const Stationary = 1<<2;
        #[doc(alias = "NSTouchPhaseEnded")]
        const Ended = 1<<3;
        #[doc(alias = "NSTouchPhaseCancelled")]
        const Cancelled = 1<<4;
        #[doc(alias = "NSTouchPhaseTouching")]
        const Touching = NSTouchPhase::Began.0|NSTouchPhase::Moved.0|NSTouchPhase::Stationary.0;
        #[doc(alias = "NSTouchPhaseAny")]
        const Any = NSUIntegerMax as _;
    }
}

unsafe impl Encode for NSTouchPhase {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTouchPhase {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTouchType(pub NSInteger);
impl NSTouchType {
    #[doc(alias = "NSTouchTypeDirect")]
    pub const Direct: Self = Self(0);
    #[doc(alias = "NSTouchTypeIndirect")]
    pub const Indirect: Self = Self(1);
}

unsafe impl Encode for NSTouchType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTouchType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTouchTypeMask(pub NSUInteger);
bitflags::bitflags! {
    impl NSTouchTypeMask: NSUInteger {
        #[doc(alias = "NSTouchTypeMaskDirect")]
        const Direct = 1<<NSTouchType::Direct.0;
        #[doc(alias = "NSTouchTypeMaskIndirect")]
        const Indirect = 1<<NSTouchType::Indirect.0;
    }
}

unsafe impl Encode for NSTouchTypeMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTouchTypeMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// TODO: pub fn NSTouchTypeMaskFromType(r#type: NSTouchType,) -> NSTouchTypeMask;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTouch;

    unsafe impl ClassType for NSTouch {
        type Super = NSObject;
        type Mutability = Immutable;
    }
);

unsafe impl Send for NSTouch {}

unsafe impl Sync for NSTouch {}

unsafe impl NSCopying for NSTouch {}

unsafe impl NSObjectProtocol for NSTouch {}

extern_methods!(
    unsafe impl NSTouch {
        #[method_id(@__retain_semantics Other identity)]
        pub unsafe fn identity(&self) -> Retained<TodoProtocols>;

        #[method(phase)]
        pub unsafe fn phase(&self) -> NSTouchPhase;

        #[method(normalizedPosition)]
        pub unsafe fn normalizedPosition(&self) -> NSPoint;

        #[method(isResting)]
        pub unsafe fn isResting(&self) -> bool;

        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Retained<AnyObject>>;

        #[method(deviceSize)]
        pub unsafe fn deviceSize(&self) -> NSSize;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTouch {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSTouchBar
    unsafe impl NSTouch {
        #[method(type)]
        pub unsafe fn r#type(&self) -> NSTouchType;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(locationInView:)]
        pub unsafe fn locationInView(&self, view: Option<&NSView>) -> NSPoint;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(previousLocationInView:)]
        pub unsafe fn previousLocationInView(&self, view: Option<&NSView>) -> NSPoint;
    }
);
