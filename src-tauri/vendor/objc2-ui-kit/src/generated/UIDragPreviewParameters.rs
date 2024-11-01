//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIPreviewParameters")]
    pub struct UIDragPreviewParameters;

    #[cfg(feature = "UIPreviewParameters")]
    unsafe impl ClassType for UIDragPreviewParameters {
        #[inherits(NSObject)]
        type Super = UIPreviewParameters;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "UIPreviewParameters")]
unsafe impl NSCopying for UIDragPreviewParameters {}

#[cfg(feature = "UIPreviewParameters")]
unsafe impl NSObjectProtocol for UIDragPreviewParameters {}

extern_methods!(
    #[cfg(feature = "UIPreviewParameters")]
    unsafe impl UIDragPreviewParameters {}
);

extern_methods!(
    /// Methods declared on superclass `UIPreviewParameters`
    #[cfg(feature = "UIPreviewParameters")]
    unsafe impl UIDragPreviewParameters {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithTextLineRects:)]
        pub unsafe fn initWithTextLineRects(
            this: Allocated<Self>,
            text_line_rects: &NSArray<NSValue>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIPreviewParameters")]
    unsafe impl UIDragPreviewParameters {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
