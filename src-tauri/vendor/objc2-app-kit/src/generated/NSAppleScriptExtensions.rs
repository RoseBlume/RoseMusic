//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_category!(
    /// Category "NSExtensions" on [`NSAppleScript`].
    #[doc(alias = "NSExtensions")]
    pub unsafe trait NSAppleScriptNSExtensions {
        #[method_id(@__retain_semantics Other richTextSource)]
        unsafe fn richTextSource(&self) -> Option<Retained<NSAttributedString>>;
    }

    unsafe impl NSAppleScriptNSExtensions for NSAppleScript {}
);
