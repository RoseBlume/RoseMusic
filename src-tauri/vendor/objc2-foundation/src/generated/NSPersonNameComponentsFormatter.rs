//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPersonNameComponentsFormatterStyle(pub NSInteger);
impl NSPersonNameComponentsFormatterStyle {
    #[doc(alias = "NSPersonNameComponentsFormatterStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSPersonNameComponentsFormatterStyleShort")]
    pub const Short: Self = Self(1);
    #[doc(alias = "NSPersonNameComponentsFormatterStyleMedium")]
    pub const Medium: Self = Self(2);
    #[doc(alias = "NSPersonNameComponentsFormatterStyleLong")]
    pub const Long: Self = Self(3);
    #[doc(alias = "NSPersonNameComponentsFormatterStyleAbbreviated")]
    pub const Abbreviated: Self = Self(4);
}

unsafe impl Encode for NSPersonNameComponentsFormatterStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPersonNameComponentsFormatterStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPersonNameComponentsFormatterOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSPersonNameComponentsFormatterOptions: NSUInteger {
        const NSPersonNameComponentsFormatterPhonetic = 1<<1;
    }
}

unsafe impl Encode for NSPersonNameComponentsFormatterOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPersonNameComponentsFormatterOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSFormatter")]
    pub struct NSPersonNameComponentsFormatter;

    #[cfg(feature = "NSFormatter")]
    unsafe impl ClassType for NSPersonNameComponentsFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSFormatter")]
unsafe impl Send for NSPersonNameComponentsFormatter {}

#[cfg(feature = "NSFormatter")]
unsafe impl Sync for NSPersonNameComponentsFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCoding for NSPersonNameComponentsFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCopying for NSPersonNameComponentsFormatter {}

#[cfg(feature = "NSFormatter")]
unsafe impl NSObjectProtocol for NSPersonNameComponentsFormatter {}

extern_methods!(
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSPersonNameComponentsFormatter {
        #[method(style)]
        pub unsafe fn style(&self) -> NSPersonNameComponentsFormatterStyle;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: NSPersonNameComponentsFormatterStyle);

        #[method(isPhonetic)]
        pub unsafe fn isPhonetic(&self) -> bool;

        #[method(setPhonetic:)]
        pub unsafe fn setPhonetic(&self, phonetic: bool);

        #[cfg(feature = "NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Retained<NSLocale>;

        #[cfg(feature = "NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[cfg(all(feature = "NSPersonNameComponents", feature = "NSString"))]
        #[method_id(@__retain_semantics Other localizedStringFromPersonNameComponents:style:options:)]
        pub unsafe fn localizedStringFromPersonNameComponents_style_options(
            components: &NSPersonNameComponents,
            name_format_style: NSPersonNameComponentsFormatterStyle,
            name_options: NSPersonNameComponentsFormatterOptions,
        ) -> Retained<NSString>;

        #[cfg(all(feature = "NSPersonNameComponents", feature = "NSString"))]
        #[method_id(@__retain_semantics Other stringFromPersonNameComponents:)]
        pub unsafe fn stringFromPersonNameComponents(
            &self,
            components: &NSPersonNameComponents,
        ) -> Retained<NSString>;

        #[cfg(all(feature = "NSAttributedString", feature = "NSPersonNameComponents"))]
        #[method_id(@__retain_semantics Other annotatedStringFromPersonNameComponents:)]
        pub unsafe fn annotatedStringFromPersonNameComponents(
            &self,
            components: &NSPersonNameComponents,
        ) -> Retained<NSAttributedString>;

        #[cfg(all(feature = "NSPersonNameComponents", feature = "NSString"))]
        #[method_id(@__retain_semantics Other personNameComponentsFromString:)]
        pub unsafe fn personNameComponentsFromString(
            &self,
            string: &NSString,
        ) -> Option<Retained<NSPersonNameComponents>>;

        #[cfg(feature = "NSString")]
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Retained<AnyObject>>>,
            string: &NSString,
            error: Option<&mut Option<Retained<NSString>>>,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSPersonNameComponentsFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSPersonNameComponentKey: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSPersonNameComponentGivenName: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSPersonNameComponentFamilyName: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSPersonNameComponentMiddleName: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSPersonNameComponentPrefix: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSPersonNameComponentSuffix: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSPersonNameComponentNickname: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSPersonNameComponentDelimiter: &'static NSString;
}
