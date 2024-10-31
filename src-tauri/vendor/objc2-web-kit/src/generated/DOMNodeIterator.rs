//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMNodeIterator;

    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl ClassType for DOMNodeIterator {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMNodeIterator {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMNodeIterator {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMNodeIterator {
        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other root)]
        pub unsafe fn root(&self) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method(whatToShow)]
        pub unsafe fn whatToShow(&self) -> c_uint;

        #[cfg(feature = "DOMNodeFilter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other filter)]
        pub unsafe fn filter(&self) -> Option<Retained<ProtocolObject<dyn DOMNodeFilter>>>;

        #[deprecated]
        #[method(expandEntityReferences)]
        pub unsafe fn expandEntityReferences(&self) -> bool;

        #[cfg(feature = "DOMNode")]
        #[method_id(@__retain_semantics Other referenceNode)]
        pub unsafe fn referenceNode(&self) -> Option<Retained<DOMNode>>;

        #[method(pointerBeforeReferenceNode)]
        pub unsafe fn pointerBeforeReferenceNode(&self) -> bool;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other nextNode)]
        pub unsafe fn nextNode(&self) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other previousNode)]
        pub unsafe fn previousNode(&self) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method(detach)]
        pub unsafe fn detach(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMNodeIterator {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMNodeIterator {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);