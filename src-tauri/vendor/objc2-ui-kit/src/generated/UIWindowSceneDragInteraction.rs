//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIWindowSceneDragInteraction;

    unsafe impl ClassType for UIWindowSceneDragInteraction {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIWindowSceneDragInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UIWindowSceneDragInteraction {}

extern_methods!(
    unsafe impl UIWindowSceneDragInteraction {
        #[cfg(feature = "UIGestureRecognizer")]
        #[method_id(@__retain_semantics Other gestureForFailureRelationships)]
        pub unsafe fn gestureForFailureRelationships(&self) -> Retained<UIGestureRecognizer>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIWindowSceneDragInteraction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
