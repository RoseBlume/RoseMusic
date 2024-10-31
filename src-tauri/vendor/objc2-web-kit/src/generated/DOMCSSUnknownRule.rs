//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMCSSRule",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMCSSUnknownRule;

    #[cfg(all(
        feature = "DOMCSSRule",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMCSSUnknownRule {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMCSSRule;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "DOMCSSRule",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMCSSUnknownRule {}

#[cfg(all(
    feature = "DOMCSSRule",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMCSSUnknownRule {}

extern_methods!(
    #[cfg(all(
        feature = "DOMCSSRule",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCSSUnknownRule {}
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMCSSRule",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCSSUnknownRule {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMCSSRule",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCSSUnknownRule {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);