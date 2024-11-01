//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAccessibilityCustomRotorSearchDirection(pub NSInteger);
impl NSAccessibilityCustomRotorSearchDirection {
    #[doc(alias = "NSAccessibilityCustomRotorSearchDirectionPrevious")]
    pub const Previous: Self = Self(0);
    #[doc(alias = "NSAccessibilityCustomRotorSearchDirectionNext")]
    pub const Next: Self = Self(1);
}

unsafe impl Encode for NSAccessibilityCustomRotorSearchDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSAccessibilityCustomRotorSearchDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAccessibilityCustomRotorType(pub NSInteger);
impl NSAccessibilityCustomRotorType {
    #[doc(alias = "NSAccessibilityCustomRotorTypeCustom")]
    pub const Custom: Self = Self(0);
    #[doc(alias = "NSAccessibilityCustomRotorTypeAny")]
    pub const Any: Self = Self(1);
    #[doc(alias = "NSAccessibilityCustomRotorTypeAnnotation")]
    pub const Annotation: Self = Self(2);
    #[doc(alias = "NSAccessibilityCustomRotorTypeBoldText")]
    pub const BoldText: Self = Self(3);
    #[doc(alias = "NSAccessibilityCustomRotorTypeHeading")]
    pub const Heading: Self = Self(4);
    #[doc(alias = "NSAccessibilityCustomRotorTypeHeadingLevel1")]
    pub const HeadingLevel1: Self = Self(5);
    #[doc(alias = "NSAccessibilityCustomRotorTypeHeadingLevel2")]
    pub const HeadingLevel2: Self = Self(6);
    #[doc(alias = "NSAccessibilityCustomRotorTypeHeadingLevel3")]
    pub const HeadingLevel3: Self = Self(7);
    #[doc(alias = "NSAccessibilityCustomRotorTypeHeadingLevel4")]
    pub const HeadingLevel4: Self = Self(8);
    #[doc(alias = "NSAccessibilityCustomRotorTypeHeadingLevel5")]
    pub const HeadingLevel5: Self = Self(9);
    #[doc(alias = "NSAccessibilityCustomRotorTypeHeadingLevel6")]
    pub const HeadingLevel6: Self = Self(10);
    #[doc(alias = "NSAccessibilityCustomRotorTypeImage")]
    pub const Image: Self = Self(11);
    #[doc(alias = "NSAccessibilityCustomRotorTypeItalicText")]
    pub const ItalicText: Self = Self(12);
    #[doc(alias = "NSAccessibilityCustomRotorTypeLandmark")]
    pub const Landmark: Self = Self(13);
    #[doc(alias = "NSAccessibilityCustomRotorTypeLink")]
    pub const Link: Self = Self(14);
    #[doc(alias = "NSAccessibilityCustomRotorTypeList")]
    pub const List: Self = Self(15);
    #[doc(alias = "NSAccessibilityCustomRotorTypeMisspelledWord")]
    pub const MisspelledWord: Self = Self(16);
    #[doc(alias = "NSAccessibilityCustomRotorTypeTable")]
    pub const Table: Self = Self(17);
    #[doc(alias = "NSAccessibilityCustomRotorTypeTextField")]
    pub const TextField: Self = Self(18);
    #[doc(alias = "NSAccessibilityCustomRotorTypeUnderlinedText")]
    pub const UnderlinedText: Self = Self(19);
    #[doc(alias = "NSAccessibilityCustomRotorTypeVisitedLink")]
    pub const VisitedLink: Self = Self(20);
    #[doc(alias = "NSAccessibilityCustomRotorTypeAudiograph")]
    pub const Audiograph: Self = Self(21);
}

unsafe impl Encode for NSAccessibilityCustomRotorType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSAccessibilityCustomRotorType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAccessibilityCustomRotor;

    unsafe impl ClassType for NSAccessibilityCustomRotor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSAccessibilityCustomRotor {}

extern_methods!(
    unsafe impl NSAccessibilityCustomRotor {
        #[method_id(@__retain_semantics Init initWithLabel:itemSearchDelegate:)]
        pub unsafe fn initWithLabel_itemSearchDelegate(
            this: Allocated<Self>,
            label: &NSString,
            item_search_delegate: &ProtocolObject<dyn NSAccessibilityCustomRotorItemSearchDelegate>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithRotorType:itemSearchDelegate:)]
        pub unsafe fn initWithRotorType_itemSearchDelegate(
            this: Allocated<Self>,
            rotor_type: NSAccessibilityCustomRotorType,
            item_search_delegate: &ProtocolObject<dyn NSAccessibilityCustomRotorItemSearchDelegate>,
        ) -> Retained<Self>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> NSAccessibilityCustomRotorType;

        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: NSAccessibilityCustomRotorType);

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Retained<NSString>;

        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: &NSString);

        #[method_id(@__retain_semantics Other itemSearchDelegate)]
        pub unsafe fn itemSearchDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSAccessibilityCustomRotorItemSearchDelegate>>>;

        #[method(setItemSearchDelegate:)]
        pub unsafe fn setItemSearchDelegate(
            &self,
            item_search_delegate: Option<
                &ProtocolObject<dyn NSAccessibilityCustomRotorItemSearchDelegate>,
            >,
        );

        #[cfg(feature = "NSAccessibilityProtocols")]
        #[method_id(@__retain_semantics Other itemLoadingDelegate)]
        pub unsafe fn itemLoadingDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSAccessibilityElementLoading>>>;

        #[cfg(feature = "NSAccessibilityProtocols")]
        #[method(setItemLoadingDelegate:)]
        pub unsafe fn setItemLoadingDelegate(
            &self,
            item_loading_delegate: Option<&ProtocolObject<dyn NSAccessibilityElementLoading>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAccessibilityCustomRotor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAccessibilityCustomRotorSearchParameters;

    unsafe impl ClassType for NSAccessibilityCustomRotorSearchParameters {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSAccessibilityCustomRotorSearchParameters {}

extern_methods!(
    unsafe impl NSAccessibilityCustomRotorSearchParameters {
        #[method_id(@__retain_semantics Other currentItem)]
        pub unsafe fn currentItem(&self) -> Option<Retained<NSAccessibilityCustomRotorItemResult>>;

        #[method(setCurrentItem:)]
        pub unsafe fn setCurrentItem(
            &self,
            current_item: Option<&NSAccessibilityCustomRotorItemResult>,
        );

        #[method(searchDirection)]
        pub unsafe fn searchDirection(&self) -> NSAccessibilityCustomRotorSearchDirection;

        #[method(setSearchDirection:)]
        pub unsafe fn setSearchDirection(
            &self,
            search_direction: NSAccessibilityCustomRotorSearchDirection,
        );

        #[method_id(@__retain_semantics Other filterString)]
        pub unsafe fn filterString(&self) -> Retained<NSString>;

        #[method(setFilterString:)]
        pub unsafe fn setFilterString(&self, filter_string: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAccessibilityCustomRotorSearchParameters {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAccessibilityCustomRotorItemResult;

    unsafe impl ClassType for NSAccessibilityCustomRotorItemResult {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSAccessibilityCustomRotorItemResult {}

extern_methods!(
    unsafe impl NSAccessibilityCustomRotorItemResult {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSAccessibilityProtocols")]
        #[method_id(@__retain_semantics Init initWithTargetElement:)]
        pub unsafe fn initWithTargetElement(
            this: Allocated<Self>,
            target_element: &ProtocolObject<dyn NSAccessibilityElementProtocol>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[method_id(@__retain_semantics Init initWithItemLoadingToken:customLabel:)]
        pub unsafe fn initWithItemLoadingToken_customLabel(
            this: Allocated<Self>,
            item_loading_token: &NSAccessibilityLoadingToken,
            custom_label: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "NSAccessibilityProtocols")]
        #[method_id(@__retain_semantics Other targetElement)]
        pub unsafe fn targetElement(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSAccessibilityElementProtocol>>>;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[method_id(@__retain_semantics Other itemLoadingToken)]
        pub unsafe fn itemLoadingToken(&self) -> Option<Retained<NSAccessibilityLoadingToken>>;

        #[method(targetRange)]
        pub unsafe fn targetRange(&self) -> NSRange;

        #[method(setTargetRange:)]
        pub unsafe fn setTargetRange(&self, target_range: NSRange);

        #[method_id(@__retain_semantics Other customLabel)]
        pub unsafe fn customLabel(&self) -> Option<Retained<NSString>>;

        #[method(setCustomLabel:)]
        pub unsafe fn setCustomLabel(&self, custom_label: Option<&NSString>);
    }
);

extern_protocol!(
    pub unsafe trait NSAccessibilityCustomRotorItemSearchDelegate: NSObjectProtocol {
        #[method_id(@__retain_semantics Other rotor:resultForSearchParameters:)]
        unsafe fn rotor_resultForSearchParameters(
            &self,
            rotor: &NSAccessibilityCustomRotor,
            search_parameters: &NSAccessibilityCustomRotorSearchParameters,
        ) -> Option<Retained<NSAccessibilityCustomRotorItemResult>>;
    }

    unsafe impl ProtocolType for dyn NSAccessibilityCustomRotorItemSearchDelegate {}
);
