//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTestComparisonOperation(pub NSUInteger);
impl NSTestComparisonOperation {
    pub const NSEqualToComparison: Self = Self(0);
    pub const NSLessThanOrEqualToComparison: Self = Self(1);
    pub const NSLessThanComparison: Self = Self(2);
    pub const NSGreaterThanOrEqualToComparison: Self = Self(3);
    pub const NSGreaterThanComparison: Self = Self(4);
    pub const NSBeginsWithComparison: Self = Self(5);
    pub const NSEndsWithComparison: Self = Self(6);
    pub const NSContainsComparison: Self = Self(7);
}

unsafe impl Encode for NSTestComparisonOperation {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTestComparisonOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScriptWhoseTest;

    unsafe impl ClassType for NSScriptWhoseTest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSScriptWhoseTest {}

unsafe impl NSObjectProtocol for NSScriptWhoseTest {}

extern_methods!(
    unsafe impl NSScriptWhoseTest {
        #[method(isTrue)]
        pub unsafe fn isTrue(&self) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSScriptWhoseTest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLogicalTest;

    unsafe impl ClassType for NSLogicalTest {
        #[inherits(NSObject)]
        type Super = NSScriptWhoseTest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSLogicalTest {}

unsafe impl NSObjectProtocol for NSLogicalTest {}

extern_methods!(
    unsafe impl NSLogicalTest {
        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Init initAndTestWithTests:)]
        pub unsafe fn initAndTestWithTests(
            this: Allocated<Self>,
            sub_tests: &NSArray<NSSpecifierTest>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Init initOrTestWithTests:)]
        pub unsafe fn initOrTestWithTests(
            this: Allocated<Self>,
            sub_tests: &NSArray<NSSpecifierTest>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initNotTestWithTest:)]
        pub unsafe fn initNotTestWithTest(
            this: Allocated<Self>,
            sub_test: &NSScriptWhoseTest,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptWhoseTest`
    unsafe impl NSLogicalTest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSLogicalTest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSpecifierTest;

    unsafe impl ClassType for NSSpecifierTest {
        #[inherits(NSObject)]
        type Super = NSScriptWhoseTest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSSpecifierTest {}

unsafe impl NSObjectProtocol for NSSpecifierTest {}

extern_methods!(
    unsafe impl NSSpecifierTest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSScriptObjectSpecifiers")]
        #[method_id(@__retain_semantics Init initWithObjectSpecifier:comparisonOperator:testObject:)]
        pub unsafe fn initWithObjectSpecifier_comparisonOperator_testObject(
            this: Allocated<Self>,
            obj1: Option<&NSScriptObjectSpecifier>,
            comp_op: NSTestComparisonOperation,
            obj2: Option<&AnyObject>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSSpecifierTest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_category!(
    /// Category "NSComparisonMethods" on [`NSObject`].
    #[doc(alias = "NSComparisonMethods")]
    pub unsafe trait NSObjectNSComparisonMethods {
        #[method(isEqualTo:)]
        unsafe fn isEqualTo(&self, object: Option<&AnyObject>) -> bool;

        #[method(isLessThanOrEqualTo:)]
        unsafe fn isLessThanOrEqualTo(&self, object: Option<&AnyObject>) -> bool;

        #[method(isLessThan:)]
        unsafe fn isLessThan(&self, object: Option<&AnyObject>) -> bool;

        #[method(isGreaterThanOrEqualTo:)]
        unsafe fn isGreaterThanOrEqualTo(&self, object: Option<&AnyObject>) -> bool;

        #[method(isGreaterThan:)]
        unsafe fn isGreaterThan(&self, object: Option<&AnyObject>) -> bool;

        #[method(isNotEqualTo:)]
        unsafe fn isNotEqualTo(&self, object: Option<&AnyObject>) -> bool;

        #[method(doesContain:)]
        unsafe fn doesContain(&self, object: &AnyObject) -> bool;

        #[cfg(feature = "NSString")]
        #[method(isLike:)]
        unsafe fn isLike(&self, object: &NSString) -> bool;

        #[cfg(feature = "NSString")]
        #[method(isCaseInsensitiveLike:)]
        unsafe fn isCaseInsensitiveLike(&self, object: &NSString) -> bool;
    }

    unsafe impl NSObjectNSComparisonMethods for NSObject {}
);

extern_category!(
    /// Category "NSScriptingComparisonMethods" on [`NSObject`].
    #[doc(alias = "NSScriptingComparisonMethods")]
    pub unsafe trait NSObjectNSScriptingComparisonMethods {
        #[method(scriptingIsEqualTo:)]
        unsafe fn scriptingIsEqualTo(&self, object: &AnyObject) -> bool;

        #[method(scriptingIsLessThanOrEqualTo:)]
        unsafe fn scriptingIsLessThanOrEqualTo(&self, object: &AnyObject) -> bool;

        #[method(scriptingIsLessThan:)]
        unsafe fn scriptingIsLessThan(&self, object: &AnyObject) -> bool;

        #[method(scriptingIsGreaterThanOrEqualTo:)]
        unsafe fn scriptingIsGreaterThanOrEqualTo(&self, object: &AnyObject) -> bool;

        #[method(scriptingIsGreaterThan:)]
        unsafe fn scriptingIsGreaterThan(&self, object: &AnyObject) -> bool;

        #[method(scriptingBeginsWith:)]
        unsafe fn scriptingBeginsWith(&self, object: &AnyObject) -> bool;

        #[method(scriptingEndsWith:)]
        unsafe fn scriptingEndsWith(&self, object: &AnyObject) -> bool;

        #[method(scriptingContains:)]
        unsafe fn scriptingContains(&self, object: &AnyObject) -> bool;
    }

    unsafe impl NSObjectNSScriptingComparisonMethods for NSObject {}
);