//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIMenuElement")]
    pub struct UIDeferredMenuElement;

    #[cfg(feature = "UIMenuElement")]
    unsafe impl ClassType for UIDeferredMenuElement {
        #[inherits(NSObject)]
        type Super = UIMenuElement;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "UIMenuElement")]
unsafe impl NSCoding for UIDeferredMenuElement {}

#[cfg(feature = "UIMenuElement")]
unsafe impl NSCopying for UIDeferredMenuElement {}

#[cfg(feature = "UIMenuElement")]
unsafe impl NSObjectProtocol for UIDeferredMenuElement {}

#[cfg(feature = "UIMenuElement")]
unsafe impl NSSecureCoding for UIDeferredMenuElement {}

extern_methods!(
    #[cfg(feature = "UIMenuElement")]
    unsafe impl UIDeferredMenuElement {
        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other elementWithProvider:)]
        pub unsafe fn elementWithProvider(
            element_provider: &block2::Block<
                dyn Fn(NonNull<block2::Block<dyn Fn(NonNull<NSArray<UIMenuElement>>)>>),
            >,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other elementWithUncachedProvider:)]
        pub unsafe fn elementWithUncachedProvider(
            element_provider: &block2::Block<
                dyn Fn(NonNull<block2::Block<dyn Fn(NonNull<NSArray<UIMenuElement>>)>>),
            >,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIMenuElement`
    #[cfg(feature = "UIMenuElement")]
    unsafe impl UIDeferredMenuElement {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
