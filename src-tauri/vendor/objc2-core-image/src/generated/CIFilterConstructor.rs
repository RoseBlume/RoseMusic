//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait CIFilterConstructor {
        #[cfg(feature = "CIFilter")]
        #[method_id(@__retain_semantics Other filterWithName:)]
        unsafe fn filterWithName(&self, name: &NSString) -> Option<Retained<CIFilter>>;
    }

    unsafe impl ProtocolType for dyn CIFilterConstructor {}
);
