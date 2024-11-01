//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIStoryboardSegue")]
    #[deprecated = "Access destinationViewController.popoverPresentationController from your segue's performHandler or override of -perform"]
    pub struct UIStoryboardPopoverSegue;

    #[cfg(feature = "UIStoryboardSegue")]
    unsafe impl ClassType for UIStoryboardPopoverSegue {
        #[inherits(NSObject)]
        type Super = UIStoryboardSegue;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "UIStoryboardSegue")]
unsafe impl NSObjectProtocol for UIStoryboardPopoverSegue {}

extern_methods!(
    #[cfg(feature = "UIStoryboardSegue")]
    unsafe impl UIStoryboardPopoverSegue {
        #[cfg(feature = "UIPopoverController")]
        #[deprecated = "Access destinationViewController.popoverPresentationController from your segue's performHandler or override of -perform"]
        #[method_id(@__retain_semantics Other popoverController)]
        pub unsafe fn popoverController(&self) -> Retained<UIPopoverController>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIStoryboardSegue`
    #[cfg(feature = "UIStoryboardSegue")]
    unsafe impl UIStoryboardPopoverSegue {
        #[cfg(all(
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other segueWithIdentifier:source:destination:performHandler:)]
        pub unsafe fn segueWithIdentifier_source_destination_performHandler(
            identifier: Option<&NSString>,
            source: &UIViewController,
            destination: &UIViewController,
            perform_handler: &block2::Block<dyn Fn()>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[method_id(@__retain_semantics Init initWithIdentifier:source:destination:)]
        pub unsafe fn initWithIdentifier_source_destination(
            this: Allocated<Self>,
            identifier: Option<&NSString>,
            source: &UIViewController,
            destination: &UIViewController,
        ) -> Retained<Self>;

        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIStoryboardSegue")]
    unsafe impl UIStoryboardPopoverSegue {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
