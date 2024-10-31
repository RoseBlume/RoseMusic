//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSRegularExpressionOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSRegularExpressionOptions: NSUInteger {
        const NSRegularExpressionCaseInsensitive = 1<<0;
        const NSRegularExpressionAllowCommentsAndWhitespace = 1<<1;
        const NSRegularExpressionIgnoreMetacharacters = 1<<2;
        const NSRegularExpressionDotMatchesLineSeparators = 1<<3;
        const NSRegularExpressionAnchorsMatchLines = 1<<4;
        const NSRegularExpressionUseUnixLineSeparators = 1<<5;
        const NSRegularExpressionUseUnicodeWordBoundaries = 1<<6;
    }
}

unsafe impl Encode for NSRegularExpressionOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSRegularExpressionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSRegularExpression;

    unsafe impl ClassType for NSRegularExpression {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSRegularExpression {}

unsafe impl Sync for NSRegularExpression {}

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSRegularExpression {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSRegularExpression {}

unsafe impl NSObjectProtocol for NSRegularExpression {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSRegularExpression {}

extern_methods!(
    unsafe impl NSRegularExpression {
        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method_id(@__retain_semantics Other regularExpressionWithPattern:options:error:_)]
        pub unsafe fn regularExpressionWithPattern_options_error(
            pattern: &NSString,
            options: NSRegularExpressionOptions,
        ) -> Result<Retained<NSRegularExpression>, Retained<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initWithPattern:options:error:_)]
        pub unsafe fn initWithPattern_options_error(
            this: Allocated<Self>,
            pattern: &NSString,
            options: NSRegularExpressionOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other pattern)]
        pub unsafe fn pattern(&self) -> Retained<NSString>;

        #[method(options)]
        pub unsafe fn options(&self) -> NSRegularExpressionOptions;

        #[method(numberOfCaptureGroups)]
        pub unsafe fn numberOfCaptureGroups(&self) -> NSUInteger;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other escapedPatternForString:)]
        pub unsafe fn escapedPatternForString(string: &NSString) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSRegularExpression {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSMatchingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSMatchingOptions: NSUInteger {
        const NSMatchingReportProgress = 1<<0;
        const NSMatchingReportCompletion = 1<<1;
        const NSMatchingAnchored = 1<<2;
        const NSMatchingWithTransparentBounds = 1<<3;
        const NSMatchingWithoutAnchoringBounds = 1<<4;
    }
}

unsafe impl Encode for NSMatchingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSMatchingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSMatchingFlags(pub NSUInteger);
bitflags::bitflags! {
    impl NSMatchingFlags: NSUInteger {
        const NSMatchingProgress = 1<<0;
        const NSMatchingCompleted = 1<<1;
        const NSMatchingHitEnd = 1<<2;
        const NSMatchingRequiredEnd = 1<<3;
        const NSMatchingInternalError = 1<<4;
    }
}

unsafe impl Encode for NSMatchingFlags {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSMatchingFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSMatching
    unsafe impl NSRegularExpression {
        #[cfg(all(
            feature = "NSRange",
            feature = "NSString",
            feature = "NSTextCheckingResult",
            feature = "block2"
        ))]
        #[method(enumerateMatchesInString:options:range:usingBlock:)]
        pub unsafe fn enumerateMatchesInString_options_range_usingBlock(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
            block: &block2::Block<
                dyn Fn(*mut NSTextCheckingResult, NSMatchingFlags, NonNull<Bool>) + '_,
            >,
        );

        #[cfg(all(
            feature = "NSArray",
            feature = "NSRange",
            feature = "NSString",
            feature = "NSTextCheckingResult"
        ))]
        #[method_id(@__retain_semantics Other matchesInString:options:range:)]
        pub unsafe fn matchesInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> Retained<NSArray<NSTextCheckingResult>>;

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method(numberOfMatchesInString:options:range:)]
        pub unsafe fn numberOfMatchesInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> NSUInteger;

        #[cfg(all(
            feature = "NSRange",
            feature = "NSString",
            feature = "NSTextCheckingResult"
        ))]
        #[method_id(@__retain_semantics Other firstMatchInString:options:range:)]
        pub unsafe fn firstMatchInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> Option<Retained<NSTextCheckingResult>>;

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method(rangeOfFirstMatchInString:options:range:)]
        pub unsafe fn rangeOfFirstMatchInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> NSRange;
    }
);

extern_methods!(
    /// NSReplacement
    unsafe impl NSRegularExpression {
        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method_id(@__retain_semantics Other stringByReplacingMatchesInString:options:range:withTemplate:)]
        pub unsafe fn stringByReplacingMatchesInString_options_range_withTemplate(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
            templ: &NSString,
        ) -> Retained<NSString>;

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method(replaceMatchesInString:options:range:withTemplate:)]
        pub unsafe fn replaceMatchesInString_options_range_withTemplate(
            &self,
            string: &NSMutableString,
            options: NSMatchingOptions,
            range: NSRange,
            templ: &NSString,
        ) -> NSUInteger;

        #[cfg(all(feature = "NSString", feature = "NSTextCheckingResult"))]
        #[method_id(@__retain_semantics Other replacementStringForResult:inString:offset:template:)]
        pub unsafe fn replacementStringForResult_inString_offset_template(
            &self,
            result: &NSTextCheckingResult,
            string: &NSString,
            offset: NSInteger,
            templ: &NSString,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other escapedTemplateForString:)]
        pub unsafe fn escapedTemplateForString(string: &NSString) -> Retained<NSString>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDataDetector;

    unsafe impl ClassType for NSDataDetector {
        #[inherits(NSObject)]
        type Super = NSRegularExpression;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSDataDetector {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSDataDetector {}

unsafe impl NSObjectProtocol for NSDataDetector {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSDataDetector {}

extern_methods!(
    unsafe impl NSDataDetector {
        #[cfg(all(feature = "NSError", feature = "NSTextCheckingResult"))]
        #[method_id(@__retain_semantics Other dataDetectorWithTypes:error:_)]
        pub unsafe fn dataDetectorWithTypes_error(
            checking_types: NSTextCheckingTypes,
        ) -> Result<Retained<NSDataDetector>, Retained<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSTextCheckingResult"))]
        #[method_id(@__retain_semantics Init initWithTypes:error:_)]
        pub unsafe fn initWithTypes_error(
            this: Allocated<Self>,
            checking_types: NSTextCheckingTypes,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "NSTextCheckingResult")]
        #[method(checkingTypes)]
        pub unsafe fn checkingTypes(&self) -> NSTextCheckingTypes;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSRegularExpression`
    unsafe impl NSDataDetector {
        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initWithPattern:options:error:_)]
        pub unsafe fn initWithPattern_options_error(
            this: Allocated<Self>,
            pattern: &NSString,
            options: NSRegularExpressionOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSDataDetector {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);