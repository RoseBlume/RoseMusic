//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMHTMLOptionsCollection;

    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl ClassType for DOMHTMLOptionsCollection {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMHTMLOptionsCollection {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMHTMLOptionsCollection {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMHTMLOptionsCollection {
        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> c_int;

        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selected_index: c_int);

        #[deprecated]
        #[method(length)]
        pub unsafe fn length(&self) -> c_uint;

        #[deprecated]
        #[method(setLength:)]
        pub unsafe fn setLength(&self, length: c_uint);

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other namedItem:)]
        pub unsafe fn namedItem(&self, name: Option<&NSString>) -> Option<Retained<DOMNode>>;

        #[cfg(all(
            feature = "DOMElement",
            feature = "DOMHTMLElement",
            feature = "DOMHTMLOptionElement",
            feature = "DOMNode"
        ))]
        #[method(add:index:)]
        pub unsafe fn add_index(&self, option: Option<&DOMHTMLOptionElement>, index: c_uint);

        #[method(remove:)]
        pub unsafe fn remove(&self, index: c_uint);

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other item:)]
        pub unsafe fn item(&self, index: c_uint) -> Option<Retained<DOMNode>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMHTMLOptionsCollection {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMHTMLOptionsCollection {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);