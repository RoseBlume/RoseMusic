//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGestureRecognizer")]
    pub struct UITapGestureRecognizer;

    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl ClassType for UITapGestureRecognizer {
        #[inherits(NSObject)]
        type Super = UIGestureRecognizer;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "UIGestureRecognizer")]
unsafe impl NSObjectProtocol for UITapGestureRecognizer {}

extern_methods!(
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UITapGestureRecognizer {
        #[method(numberOfTapsRequired)]
        pub fn numberOfTapsRequired(&self) -> NSUInteger;

        #[method(setNumberOfTapsRequired:)]
        pub fn setNumberOfTapsRequired(&self, number_of_taps_required: NSUInteger);

        #[method(numberOfTouchesRequired)]
        pub fn numberOfTouchesRequired(&self) -> NSUInteger;

        #[method(setNumberOfTouchesRequired:)]
        pub fn setNumberOfTouchesRequired(&self, number_of_touches_required: NSUInteger);

        #[cfg(feature = "UIEvent")]
        #[method(buttonMaskRequired)]
        pub fn buttonMaskRequired(&self) -> UIEventButtonMask;

        #[cfg(feature = "UIEvent")]
        #[method(setButtonMaskRequired:)]
        pub fn setButtonMaskRequired(&self, button_mask_required: UIEventButtonMask);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGestureRecognizer`
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UITapGestureRecognizer {
        #[method_id(@__retain_semantics Init initWithTarget:action:)]
        pub unsafe fn initWithTarget_action(
            this: Allocated<Self>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UITapGestureRecognizer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
