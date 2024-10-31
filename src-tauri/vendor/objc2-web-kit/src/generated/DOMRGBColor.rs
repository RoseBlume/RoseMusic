//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMRGBColor;

    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl ClassType for DOMRGBColor {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMRGBColor {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMRGBColor {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMRGBColor {
        #[cfg(all(feature = "DOMCSSPrimitiveValue", feature = "DOMCSSValue"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other red)]
        pub unsafe fn red(&self) -> Option<Retained<DOMCSSPrimitiveValue>>;

        #[cfg(all(feature = "DOMCSSPrimitiveValue", feature = "DOMCSSValue"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other green)]
        pub unsafe fn green(&self) -> Option<Retained<DOMCSSPrimitiveValue>>;

        #[cfg(all(feature = "DOMCSSPrimitiveValue", feature = "DOMCSSValue"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other blue)]
        pub unsafe fn blue(&self) -> Option<Retained<DOMCSSPrimitiveValue>>;

        #[cfg(all(feature = "DOMCSSPrimitiveValue", feature = "DOMCSSValue"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other alpha)]
        pub unsafe fn alpha(&self) -> Option<Retained<DOMCSSPrimitiveValue>>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Retained<NSColor>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMRGBColor {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMRGBColor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);