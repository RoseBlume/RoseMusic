//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIBarMetrics(pub NSInteger);
impl UIBarMetrics {
    #[doc(alias = "UIBarMetricsDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "UIBarMetricsCompact")]
    pub const Compact: Self = Self(1);
    #[doc(alias = "UIBarMetricsDefaultPrompt")]
    pub const DefaultPrompt: Self = Self(101);
    #[doc(alias = "UIBarMetricsCompactPrompt")]
    pub const CompactPrompt: Self = Self(102);
    #[deprecated]
    #[doc(alias = "UIBarMetricsLandscapePhone")]
    pub const LandscapePhone: Self = Self(UIBarMetrics::Compact.0);
    #[deprecated]
    #[doc(alias = "UIBarMetricsLandscapePhonePrompt")]
    pub const LandscapePhonePrompt: Self = Self(UIBarMetrics::CompactPrompt.0);
}

unsafe impl Encode for UIBarMetrics {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIBarMetrics {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIBarPosition(pub NSInteger);
impl UIBarPosition {
    #[doc(alias = "UIBarPositionAny")]
    pub const Any: Self = Self(0);
    #[doc(alias = "UIBarPositionBottom")]
    pub const Bottom: Self = Self(1);
    #[doc(alias = "UIBarPositionTop")]
    pub const Top: Self = Self(2);
    #[doc(alias = "UIBarPositionTopAttached")]
    pub const TopAttached: Self = Self(3);
}

unsafe impl Encode for UIBarPosition {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIBarPosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait UIBarPositioning: NSObjectProtocol + IsMainThreadOnly {
        #[method(barPosition)]
        unsafe fn barPosition(&self) -> UIBarPosition;
    }

    unsafe impl ProtocolType for dyn UIBarPositioning {}
);

extern_protocol!(
    pub unsafe trait UIBarPositioningDelegate: NSObjectProtocol + IsMainThreadOnly {
        #[optional]
        #[method(positionForBar:)]
        unsafe fn positionForBar(
            &self,
            bar: &ProtocolObject<dyn UIBarPositioning>,
        ) -> UIBarPosition;
    }

    unsafe impl ProtocolType for dyn UIBarPositioningDelegate {}
);
