//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIContentUnavailableConfigurationState;

    unsafe impl ClassType for UIContentUnavailableConfigurationState {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCoding for UIContentUnavailableConfigurationState {}

unsafe impl NSCopying for UIContentUnavailableConfigurationState {}

unsafe impl NSObjectProtocol for UIContentUnavailableConfigurationState {}

unsafe impl NSSecureCoding for UIContentUnavailableConfigurationState {}

#[cfg(feature = "UIConfigurationState")]
unsafe impl UIConfigurationState for UIContentUnavailableConfigurationState {}

extern_methods!(
    unsafe impl UIContentUnavailableConfigurationState {
        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Init initWithTraitCollection:)]
        pub unsafe fn initWithTraitCollection(
            this: Allocated<Self>,
            trait_collection: &UITraitCollection,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Other traitCollection)]
        pub unsafe fn traitCollection(&self) -> Retained<UITraitCollection>;

        #[cfg(feature = "UITraitCollection")]
        #[method(setTraitCollection:)]
        pub unsafe fn setTraitCollection(&self, trait_collection: &UITraitCollection);

        #[method_id(@__retain_semantics Other searchText)]
        pub unsafe fn searchText(&self) -> Option<Retained<NSString>>;

        #[method(setSearchText:)]
        pub unsafe fn setSearchText(&self, search_text: Option<&NSString>);
    }
);
