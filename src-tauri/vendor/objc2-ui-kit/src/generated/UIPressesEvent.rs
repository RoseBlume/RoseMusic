//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIEvent")]
    pub struct UIPressesEvent;

    #[cfg(feature = "UIEvent")]
    unsafe impl ClassType for UIPressesEvent {
        #[inherits(NSObject)]
        type Super = UIEvent;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "UIEvent")]
unsafe impl NSObjectProtocol for UIPressesEvent {}

extern_methods!(
    #[cfg(feature = "UIEvent")]
    unsafe impl UIPressesEvent {
        #[cfg(feature = "UIPress")]
        #[method_id(@__retain_semantics Other allPresses)]
        pub unsafe fn allPresses(&self) -> Retained<NSSet<UIPress>>;

        #[cfg(all(feature = "UIGestureRecognizer", feature = "UIPress"))]
        #[method_id(@__retain_semantics Other pressesForGestureRecognizer:)]
        pub unsafe fn pressesForGestureRecognizer(
            &self,
            gesture: &UIGestureRecognizer,
        ) -> Retained<NSSet<UIPress>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIEvent")]
    unsafe impl UIPressesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);