//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

pub const NSOpenStepUnicodeReservedBase: c_uint = 0xF400;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCharacterSet;

    unsafe impl ClassType for NSCharacterSet {
        type Super = NSObject;
        type Mutability = ImmutableWithMutableSubclass<NSMutableCharacterSet>;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSCharacterSet {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSCharacterSet {}

#[cfg(feature = "NSObject")]
unsafe impl NSMutableCopying for NSCharacterSet {}

unsafe impl NSObjectProtocol for NSCharacterSet {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSCharacterSet {}

extern_methods!(
    unsafe impl NSCharacterSet {
        #[method_id(@__retain_semantics Other controlCharacterSet)]
        pub unsafe fn controlCharacterSet() -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other whitespaceCharacterSet)]
        pub unsafe fn whitespaceCharacterSet() -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other whitespaceAndNewlineCharacterSet)]
        pub unsafe fn whitespaceAndNewlineCharacterSet() -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other decimalDigitCharacterSet)]
        pub unsafe fn decimalDigitCharacterSet() -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other letterCharacterSet)]
        pub unsafe fn letterCharacterSet() -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other lowercaseLetterCharacterSet)]
        pub unsafe fn lowercaseLetterCharacterSet() -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other uppercaseLetterCharacterSet)]
        pub unsafe fn uppercaseLetterCharacterSet() -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other nonBaseCharacterSet)]
        pub unsafe fn nonBaseCharacterSet() -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other alphanumericCharacterSet)]
        pub unsafe fn alphanumericCharacterSet() -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other decomposableCharacterSet)]
        pub unsafe fn decomposableCharacterSet() -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other illegalCharacterSet)]
        pub unsafe fn illegalCharacterSet() -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other punctuationCharacterSet)]
        pub unsafe fn punctuationCharacterSet() -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other capitalizedLetterCharacterSet)]
        pub unsafe fn capitalizedLetterCharacterSet() -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other symbolCharacterSet)]
        pub unsafe fn symbolCharacterSet() -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other newlineCharacterSet)]
        pub unsafe fn newlineCharacterSet() -> Retained<NSCharacterSet>;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Other characterSetWithRange:)]
        pub unsafe fn characterSetWithRange(a_range: NSRange) -> Retained<NSCharacterSet>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other characterSetWithCharactersInString:)]
        pub unsafe fn characterSetWithCharactersInString(
            a_string: &NSString,
        ) -> Retained<NSCharacterSet>;

        #[cfg(feature = "NSData")]
        #[method_id(@__retain_semantics Other characterSetWithBitmapRepresentation:)]
        pub unsafe fn characterSetWithBitmapRepresentation(
            data: &NSData,
        ) -> Retained<NSCharacterSet>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other characterSetWithContentsOfFile:)]
        pub unsafe fn characterSetWithContentsOfFile(
            f_name: &NSString,
        ) -> Option<Retained<NSCharacterSet>>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[method(characterIsMember:)]
        pub unsafe fn characterIsMember(&self, a_character: unichar) -> bool;

        #[cfg(feature = "NSData")]
        #[method_id(@__retain_semantics Other bitmapRepresentation)]
        pub unsafe fn bitmapRepresentation(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other invertedSet)]
        pub unsafe fn invertedSet(&self) -> Retained<NSCharacterSet>;

        #[method(longCharacterIsMember:)]
        pub unsafe fn longCharacterIsMember(&self, the_long_char: UTF32Char) -> bool;

        #[method(isSupersetOfSet:)]
        pub unsafe fn isSupersetOfSet(&self, the_other_set: &NSCharacterSet) -> bool;

        #[method(hasMemberInPlane:)]
        pub unsafe fn hasMemberInPlane(&self, the_plane: u8) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSCharacterSet {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMutableCharacterSet;

    unsafe impl ClassType for NSMutableCharacterSet {
        #[inherits(NSObject)]
        type Super = NSCharacterSet;
        type Mutability = MutableWithImmutableSuperclass<NSCharacterSet>;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSMutableCharacterSet {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSMutableCharacterSet {}

#[cfg(feature = "NSObject")]
unsafe impl NSMutableCopying for NSMutableCharacterSet {}

unsafe impl NSObjectProtocol for NSMutableCharacterSet {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSMutableCharacterSet {}

extern_methods!(
    unsafe impl NSMutableCharacterSet {
        #[cfg(feature = "NSRange")]
        #[method(addCharactersInRange:)]
        pub unsafe fn addCharactersInRange(&mut self, a_range: NSRange);

        #[cfg(feature = "NSRange")]
        #[method(removeCharactersInRange:)]
        pub unsafe fn removeCharactersInRange(&mut self, a_range: NSRange);

        #[cfg(feature = "NSString")]
        #[method(addCharactersInString:)]
        pub unsafe fn addCharactersInString(&mut self, a_string: &NSString);

        #[cfg(feature = "NSString")]
        #[method(removeCharactersInString:)]
        pub unsafe fn removeCharactersInString(&mut self, a_string: &NSString);

        #[method(formUnionWithCharacterSet:)]
        pub unsafe fn formUnionWithCharacterSet(&mut self, other_set: &NSCharacterSet);

        #[method(formIntersectionWithCharacterSet:)]
        pub unsafe fn formIntersectionWithCharacterSet(&mut self, other_set: &NSCharacterSet);

        #[method(invert)]
        pub unsafe fn invert(&mut self);

        #[method_id(@__retain_semantics Other controlCharacterSet)]
        pub unsafe fn controlCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[method_id(@__retain_semantics Other whitespaceCharacterSet)]
        pub unsafe fn whitespaceCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[method_id(@__retain_semantics Other whitespaceAndNewlineCharacterSet)]
        pub unsafe fn whitespaceAndNewlineCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[method_id(@__retain_semantics Other decimalDigitCharacterSet)]
        pub unsafe fn decimalDigitCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[method_id(@__retain_semantics Other letterCharacterSet)]
        pub unsafe fn letterCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[method_id(@__retain_semantics Other lowercaseLetterCharacterSet)]
        pub unsafe fn lowercaseLetterCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[method_id(@__retain_semantics Other uppercaseLetterCharacterSet)]
        pub unsafe fn uppercaseLetterCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[method_id(@__retain_semantics Other nonBaseCharacterSet)]
        pub unsafe fn nonBaseCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[method_id(@__retain_semantics Other alphanumericCharacterSet)]
        pub unsafe fn alphanumericCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[method_id(@__retain_semantics Other decomposableCharacterSet)]
        pub unsafe fn decomposableCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[method_id(@__retain_semantics Other illegalCharacterSet)]
        pub unsafe fn illegalCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[method_id(@__retain_semantics Other punctuationCharacterSet)]
        pub unsafe fn punctuationCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[method_id(@__retain_semantics Other capitalizedLetterCharacterSet)]
        pub unsafe fn capitalizedLetterCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[method_id(@__retain_semantics Other symbolCharacterSet)]
        pub unsafe fn symbolCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[method_id(@__retain_semantics Other newlineCharacterSet)]
        pub unsafe fn newlineCharacterSet() -> Retained<NSMutableCharacterSet>;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Other characterSetWithRange:)]
        pub unsafe fn characterSetWithRange(a_range: NSRange) -> Retained<NSMutableCharacterSet>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other characterSetWithCharactersInString:)]
        pub unsafe fn characterSetWithCharactersInString(
            a_string: &NSString,
        ) -> Retained<NSMutableCharacterSet>;

        #[cfg(feature = "NSData")]
        #[method_id(@__retain_semantics Other characterSetWithBitmapRepresentation:)]
        pub unsafe fn characterSetWithBitmapRepresentation(
            data: &NSData,
        ) -> Retained<NSMutableCharacterSet>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other characterSetWithContentsOfFile:)]
        pub unsafe fn characterSetWithContentsOfFile(
            f_name: &NSString,
        ) -> Option<Retained<NSMutableCharacterSet>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCharacterSet`
    unsafe impl NSMutableCharacterSet {
        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMutableCharacterSet {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
