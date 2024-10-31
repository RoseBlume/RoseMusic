//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait UISearchSuggestion: NSObjectProtocol + IsMainThreadOnly {
        #[method_id(@__retain_semantics Other localizedSuggestion)]
        unsafe fn localizedSuggestion(&self) -> Option<Retained<NSString>>;

        #[optional]
        #[method_id(@__retain_semantics Other localizedDescription)]
        unsafe fn localizedDescription(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "UIImage")]
        #[optional]
        #[method_id(@__retain_semantics Other iconImage)]
        unsafe fn iconImage(&self) -> Option<Retained<UIImage>>;

        #[method_id(@__retain_semantics Other localizedAttributedSuggestion)]
        unsafe fn localizedAttributedSuggestion(&self) -> Option<Retained<NSAttributedString>>;

        #[method_id(@__retain_semantics Other representedObject)]
        unsafe fn representedObject(&self) -> Option<Retained<AnyObject>>;

        #[method(setRepresentedObject:)]
        unsafe fn setRepresentedObject(&self, represented_object: Option<&AnyObject>);
    }

    unsafe impl ProtocolType for dyn UISearchSuggestion {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISearchSuggestionItem;

    unsafe impl ClassType for UISearchSuggestionItem {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UISearchSuggestionItem {}

unsafe impl UISearchSuggestion for UISearchSuggestionItem {}

extern_methods!(
    unsafe impl UISearchSuggestionItem {
        #[method_id(@__retain_semantics Other suggestionWithLocalizedSuggestion:)]
        pub unsafe fn suggestionWithLocalizedSuggestion(
            suggestion: &NSString,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other suggestionWithLocalizedSuggestion:descriptionString:)]
        pub unsafe fn suggestionWithLocalizedSuggestion_descriptionString(
            suggestion: &NSString,
            description: Option<&NSString>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other suggestionWithLocalizedSuggestion:descriptionString:iconImage:)]
        pub unsafe fn suggestionWithLocalizedSuggestion_descriptionString_iconImage(
            suggestion: &NSString,
            description: Option<&NSString>,
            icon_image: Option<&UIImage>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other suggestionWithLocalizedAttributedSuggestion:)]
        pub unsafe fn suggestionWithLocalizedAttributedSuggestion(
            suggestion: &NSAttributedString,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other suggestionWithLocalizedAttributedSuggestion:descriptionString:)]
        pub unsafe fn suggestionWithLocalizedAttributedSuggestion_descriptionString(
            suggestion: &NSAttributedString,
            description: Option<&NSString>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other suggestionWithLocalizedAttributedSuggestion:descriptionString:iconImage:)]
        pub unsafe fn suggestionWithLocalizedAttributedSuggestion_descriptionString_iconImage(
            suggestion: &NSAttributedString,
            description: Option<&NSString>,
            icon_image: Option<&UIImage>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLocalizedSuggestion:)]
        pub unsafe fn initWithLocalizedSuggestion(
            this: Allocated<Self>,
            suggestion: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLocalizedSuggestion:localizedDescription:)]
        pub unsafe fn initWithLocalizedSuggestion_localizedDescription(
            this: Allocated<Self>,
            suggestion: &NSString,
            description: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Init initWithLocalizedSuggestion:localizedDescription:iconImage:)]
        pub unsafe fn initWithLocalizedSuggestion_localizedDescription_iconImage(
            this: Allocated<Self>,
            suggestion: &NSString,
            description: Option<&NSString>,
            icon_image: Option<&UIImage>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLocalizedAttributedSuggestion:)]
        pub unsafe fn initWithLocalizedAttributedSuggestion(
            this: Allocated<Self>,
            suggestion: &NSAttributedString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLocalizedAttributedSuggestion:localizedDescription:)]
        pub unsafe fn initWithLocalizedAttributedSuggestion_localizedDescription(
            this: Allocated<Self>,
            suggestion: &NSAttributedString,
            description: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Init initWithLocalizedAttributedSuggestion:localizedDescription:iconImage:)]
        pub unsafe fn initWithLocalizedAttributedSuggestion_localizedDescription_iconImage(
            this: Allocated<Self>,
            suggestion: &NSAttributedString,
            description: Option<&NSString>,
            icon_image: Option<&UIImage>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other localizedAttributedSuggestion)]
        pub unsafe fn localizedAttributedSuggestion(&self) -> Option<Retained<NSAttributedString>>;

        #[method_id(@__retain_semantics Other localizedSuggestion)]
        pub unsafe fn localizedSuggestion(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other localizedDescription)]
        pub unsafe fn localizedDescription(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other iconImage)]
        pub unsafe fn iconImage(&self) -> Option<Retained<UIImage>>;

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Retained<AnyObject>>;

        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, represented_object: Option<&AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UISearchSuggestionItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
