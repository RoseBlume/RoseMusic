//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UNNotificationCategoryOptions(pub NSUInteger);
bitflags::bitflags! {
    impl UNNotificationCategoryOptions: NSUInteger {
        const UNNotificationCategoryOptionCustomDismissAction = 1<<0;
        const UNNotificationCategoryOptionAllowInCarPlay = 1<<1;
        const UNNotificationCategoryOptionHiddenPreviewsShowTitle = 1<<2;
        const UNNotificationCategoryOptionHiddenPreviewsShowSubtitle = 1<<3;
#[deprecated = "Announcement option is ignored"]
        const UNNotificationCategoryOptionAllowAnnouncement = 1<<4;
    }
}

unsafe impl Encode for UNNotificationCategoryOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UNNotificationCategoryOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub static UNNotificationCategoryOptionNone: UNNotificationCategoryOptions =
    UNNotificationCategoryOptions(0);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotificationCategory;

    unsafe impl ClassType for UNNotificationCategory {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for UNNotificationCategory {}

unsafe impl NSCopying for UNNotificationCategory {}

unsafe impl NSObjectProtocol for UNNotificationCategory {}

unsafe impl NSSecureCoding for UNNotificationCategory {}

extern_methods!(
    unsafe impl UNNotificationCategory {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[cfg(feature = "UNNotificationAction")]
        #[method_id(@__retain_semantics Other actions)]
        pub unsafe fn actions(&self) -> Retained<NSArray<UNNotificationAction>>;

        #[method_id(@__retain_semantics Other intentIdentifiers)]
        pub unsafe fn intentIdentifiers(&self) -> Retained<NSArray<NSString>>;

        #[method(options)]
        pub unsafe fn options(&self) -> UNNotificationCategoryOptions;

        #[method_id(@__retain_semantics Other hiddenPreviewsBodyPlaceholder)]
        pub unsafe fn hiddenPreviewsBodyPlaceholder(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other categorySummaryFormat)]
        pub unsafe fn categorySummaryFormat(&self) -> Retained<NSString>;

        #[cfg(feature = "UNNotificationAction")]
        #[method_id(@__retain_semantics Other categoryWithIdentifier:actions:intentIdentifiers:options:)]
        pub unsafe fn categoryWithIdentifier_actions_intentIdentifiers_options(
            identifier: &NSString,
            actions: &NSArray<UNNotificationAction>,
            intent_identifiers: &NSArray<NSString>,
            options: UNNotificationCategoryOptions,
        ) -> Retained<Self>;

        #[cfg(feature = "UNNotificationAction")]
        #[method_id(@__retain_semantics Other categoryWithIdentifier:actions:intentIdentifiers:hiddenPreviewsBodyPlaceholder:options:)]
        pub unsafe fn categoryWithIdentifier_actions_intentIdentifiers_hiddenPreviewsBodyPlaceholder_options(
            identifier: &NSString,
            actions: &NSArray<UNNotificationAction>,
            intent_identifiers: &NSArray<NSString>,
            hidden_previews_body_placeholder: &NSString,
            options: UNNotificationCategoryOptions,
        ) -> Retained<Self>;

        #[cfg(feature = "UNNotificationAction")]
        #[method_id(@__retain_semantics Other categoryWithIdentifier:actions:intentIdentifiers:hiddenPreviewsBodyPlaceholder:categorySummaryFormat:options:)]
        pub unsafe fn categoryWithIdentifier_actions_intentIdentifiers_hiddenPreviewsBodyPlaceholder_categorySummaryFormat_options(
            identifier: &NSString,
            actions: &NSArray<UNNotificationAction>,
            intent_identifiers: &NSArray<NSString>,
            hidden_previews_body_placeholder: Option<&NSString>,
            category_summary_format: Option<&NSString>,
            options: UNNotificationCategoryOptions,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNNotificationCategory {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
