//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextAlternatives;

    unsafe impl ClassType for NSTextAlternatives {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NSTextAlternatives {}

unsafe impl NSObjectProtocol for NSTextAlternatives {}

unsafe impl NSSecureCoding for NSTextAlternatives {}

extern_methods!(
    unsafe impl NSTextAlternatives {
        #[method_id(@__retain_semantics Init initWithPrimaryString:alternativeStrings:)]
        pub unsafe fn initWithPrimaryString_alternativeStrings(
            this: Allocated<Self>,
            primary_string: &NSString,
            alternative_strings: &NSArray<NSString>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other primaryString)]
        pub unsafe fn primaryString(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other alternativeStrings)]
        pub unsafe fn alternativeStrings(&self) -> Retained<NSArray<NSString>>;

        #[method(noteSelectedAlternativeString:)]
        pub unsafe fn noteSelectedAlternativeString(&self, alternative_string: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextAlternatives {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static NSTextAlternativesSelectedAlternativeStringNotification: &'static NSNotificationName;
}