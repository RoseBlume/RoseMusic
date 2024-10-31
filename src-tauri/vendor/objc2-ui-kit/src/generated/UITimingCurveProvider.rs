//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITimingCurveType(pub NSInteger);
impl UITimingCurveType {
    #[doc(alias = "UITimingCurveTypeBuiltin")]
    pub const Builtin: Self = Self(0);
    #[doc(alias = "UITimingCurveTypeCubic")]
    pub const Cubic: Self = Self(1);
    #[doc(alias = "UITimingCurveTypeSpring")]
    pub const Spring: Self = Self(2);
    #[doc(alias = "UITimingCurveTypeComposed")]
    pub const Composed: Self = Self(3);
}

unsafe impl Encode for UITimingCurveType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITimingCurveType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait UITimingCurveProvider: NSCoding + NSCopying + IsMainThreadOnly {
        #[method(timingCurveType)]
        unsafe fn timingCurveType(&self) -> UITimingCurveType;

        #[cfg(feature = "UITimingParameters")]
        #[method_id(@__retain_semantics Other cubicTimingParameters)]
        unsafe fn cubicTimingParameters(&self) -> Option<Retained<UICubicTimingParameters>>;

        #[cfg(feature = "UITimingParameters")]
        #[method_id(@__retain_semantics Other springTimingParameters)]
        unsafe fn springTimingParameters(&self) -> Option<Retained<UISpringTimingParameters>>;
    }

    unsafe impl ProtocolType for dyn UITimingCurveProvider {}
);
