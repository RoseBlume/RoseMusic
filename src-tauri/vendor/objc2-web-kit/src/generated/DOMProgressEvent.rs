//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMProgressEvent;

    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMProgressEvent {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMEvent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMProgressEvent {}

#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMProgressEvent {}

extern_methods!(
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMProgressEvent {
        #[deprecated]
        #[method(lengthComputable)]
        pub unsafe fn lengthComputable(&self) -> bool;

        #[deprecated]
        #[method(loaded)]
        pub unsafe fn loaded(&self) -> c_ulonglong;

        #[deprecated]
        #[method(total)]
        pub unsafe fn total(&self) -> c_ulonglong;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMProgressEvent {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMProgressEvent {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);