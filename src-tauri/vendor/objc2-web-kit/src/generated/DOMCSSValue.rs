//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[deprecated]
pub const DOM_CSS_INHERIT: c_uint = 0;
#[deprecated]
pub const DOM_CSS_PRIMITIVE_VALUE: c_uint = 1;
#[deprecated]
pub const DOM_CSS_VALUE_LIST: c_uint = 2;
#[deprecated]
pub const DOM_CSS_CUSTOM: c_uint = 3;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMCSSValue;

    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl ClassType for DOMCSSValue {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMCSSValue {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMCSSValue {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMCSSValue {
        #[deprecated]
        #[method_id(@__retain_semantics Other cssText)]
        pub unsafe fn cssText(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setCssText:)]
        pub unsafe fn setCssText(&self, css_text: Option<&NSString>);

        #[deprecated]
        #[method(cssValueType)]
        pub unsafe fn cssValueType(&self) -> c_ushort;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMCSSValue {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMCSSValue {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);