//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    pub unsafe trait NSChangeSpelling {
        #[method(changeSpelling:)]
        unsafe fn changeSpelling(&self, sender: Option<&AnyObject>);
    }

    unsafe impl ProtocolType for dyn NSChangeSpelling {}
);

extern_protocol!(
    pub unsafe trait NSIgnoreMisspelledWords {
        #[method(ignoreSpelling:)]
        unsafe fn ignoreSpelling(&self, sender: Option<&AnyObject>);
    }

    unsafe impl ProtocolType for dyn NSIgnoreMisspelledWords {}
);
