//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSClassDescription;

    unsafe impl ClassType for NSClassDescription {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSClassDescription {}

extern_methods!(
    unsafe impl NSClassDescription {
        #[method(registerClassDescription:forClass:)]
        pub unsafe fn registerClassDescription_forClass(
            description: &NSClassDescription,
            a_class: &AnyClass,
        );

        #[method(invalidateClassDescriptionCache)]
        pub unsafe fn invalidateClassDescriptionCache();

        #[method_id(@__retain_semantics Other classDescriptionForClass:)]
        pub unsafe fn classDescriptionForClass(
            a_class: &AnyClass,
        ) -> Option<Retained<NSClassDescription>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other attributeKeys)]
        pub unsafe fn attributeKeys(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other toOneRelationshipKeys)]
        pub unsafe fn toOneRelationshipKeys(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other toManyRelationshipKeys)]
        pub unsafe fn toManyRelationshipKeys(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other inverseForRelationshipKey:)]
        pub unsafe fn inverseForRelationshipKey(
            &self,
            relationship_key: &NSString,
        ) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSClassDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_category!(
    /// Category "NSClassDescriptionPrimitives" on [`NSObject`].
    #[doc(alias = "NSClassDescriptionPrimitives")]
    pub unsafe trait NSObjectNSClassDescriptionPrimitives {
        #[method_id(@__retain_semantics Other classDescription)]
        unsafe fn classDescription(&self) -> Retained<NSClassDescription>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other attributeKeys)]
        unsafe fn attributeKeys(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other toOneRelationshipKeys)]
        unsafe fn toOneRelationshipKeys(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other toManyRelationshipKeys)]
        unsafe fn toManyRelationshipKeys(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other inverseForRelationshipKey:)]
        unsafe fn inverseForRelationshipKey(
            &self,
            relationship_key: &NSString,
        ) -> Option<Retained<NSString>>;
    }

    unsafe impl NSObjectNSClassDescriptionPrimitives for NSObject {}
);

extern "C" {
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSClassDescriptionNeededForClassNotification: &'static NSNotificationName;
}
