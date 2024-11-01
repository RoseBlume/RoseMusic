//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern "C" {
    #[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
    pub static NSUndefinedKeyException: &'static NSExceptionName;
}

// NS_TYPED_ENUM
#[cfg(feature = "NSString")]
pub type NSKeyValueOperator = NSString;

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSAverageKeyValueOperator: &'static NSKeyValueOperator;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCountKeyValueOperator: &'static NSKeyValueOperator;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSDistinctUnionOfArraysKeyValueOperator: &'static NSKeyValueOperator;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSDistinctUnionOfObjectsKeyValueOperator: &'static NSKeyValueOperator;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSDistinctUnionOfSetsKeyValueOperator: &'static NSKeyValueOperator;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSMaximumKeyValueOperator: &'static NSKeyValueOperator;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSMinimumKeyValueOperator: &'static NSKeyValueOperator;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSSumKeyValueOperator: &'static NSKeyValueOperator;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSUnionOfArraysKeyValueOperator: &'static NSKeyValueOperator;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSUnionOfObjectsKeyValueOperator: &'static NSKeyValueOperator;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSUnionOfSetsKeyValueOperator: &'static NSKeyValueOperator;
}

extern_category!(
    /// Category "NSKeyValueCoding" on [`NSObject`].
    #[doc(alias = "NSKeyValueCoding")]
    pub unsafe trait NSObjectNSKeyValueCoding {
        #[method(accessInstanceVariablesDirectly)]
        unsafe fn accessInstanceVariablesDirectly() -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other valueForKey:)]
        unsafe fn valueForKey(&self, key: &NSString) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[method(setValue:forKey:)]
        unsafe fn setValue_forKey(&self, value: Option<&AnyObject>, key: &NSString);

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method(validateValue:forKey:error:_)]
        unsafe fn validateValue_forKey_error(
            &self,
            io_value: &mut Option<Retained<AnyObject>>,
            in_key: &NSString,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other mutableArrayValueForKey:)]
        unsafe fn mutableArrayValueForKey(&self, key: &NSString) -> Retained<NSMutableArray>;

        #[cfg(all(feature = "NSOrderedSet", feature = "NSString"))]
        #[method_id(@__retain_semantics Other mutableOrderedSetValueForKey:)]
        unsafe fn mutableOrderedSetValueForKey(
            &self,
            key: &NSString,
        ) -> Retained<NSMutableOrderedSet>;

        #[cfg(all(feature = "NSSet", feature = "NSString"))]
        #[method_id(@__retain_semantics Other mutableSetValueForKey:)]
        unsafe fn mutableSetValueForKey(&self, key: &NSString) -> Retained<NSMutableSet>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other valueForKeyPath:)]
        unsafe fn valueForKeyPath(&self, key_path: &NSString) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[method(setValue:forKeyPath:)]
        unsafe fn setValue_forKeyPath(&self, value: Option<&AnyObject>, key_path: &NSString);

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method(validateValue:forKeyPath:error:_)]
        unsafe fn validateValue_forKeyPath_error(
            &self,
            io_value: &mut Option<Retained<AnyObject>>,
            in_key_path: &NSString,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other mutableArrayValueForKeyPath:)]
        unsafe fn mutableArrayValueForKeyPath(
            &self,
            key_path: &NSString,
        ) -> Retained<NSMutableArray>;

        #[cfg(all(feature = "NSOrderedSet", feature = "NSString"))]
        #[method_id(@__retain_semantics Other mutableOrderedSetValueForKeyPath:)]
        unsafe fn mutableOrderedSetValueForKeyPath(
            &self,
            key_path: &NSString,
        ) -> Retained<NSMutableOrderedSet>;

        #[cfg(all(feature = "NSSet", feature = "NSString"))]
        #[method_id(@__retain_semantics Other mutableSetValueForKeyPath:)]
        unsafe fn mutableSetValueForKeyPath(&self, key_path: &NSString) -> Retained<NSMutableSet>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other valueForUndefinedKey:)]
        unsafe fn valueForUndefinedKey(&self, key: &NSString) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[method(setValue:forUndefinedKey:)]
        unsafe fn setValue_forUndefinedKey(&self, value: Option<&AnyObject>, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(setNilValueForKey:)]
        unsafe fn setNilValueForKey(&self, key: &NSString);

        #[cfg(all(feature = "NSArray", feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other dictionaryWithValuesForKeys:)]
        unsafe fn dictionaryWithValuesForKeys(
            &self,
            keys: &NSArray<NSString>,
        ) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method(setValuesForKeysWithDictionary:)]
        unsafe fn setValuesForKeysWithDictionary(
            &self,
            keyed_values: &NSDictionary<NSString, AnyObject>,
        );
    }

    unsafe impl NSObjectNSKeyValueCoding for NSObject {}
);

extern_methods!(
    /// NSKeyValueCoding
    #[cfg(feature = "NSArray")]
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other valueForKey:)]
        pub unsafe fn valueForKey(&self, key: &NSString) -> Retained<AnyObject>;

        #[cfg(feature = "NSString")]
        #[method(setValue:forKey:)]
        pub unsafe fn setValue_forKey(&self, value: Option<&AnyObject>, key: &NSString);
    }
);

extern_methods!(
    /// NSKeyValueCoding
    #[cfg(feature = "NSDictionary")]
    unsafe impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other valueForKey:)]
        pub unsafe fn valueForKey(&self, key: &NSString) -> Option<Retained<ObjectType>>;
    }
);

extern_methods!(
    /// NSKeyValueCoding
    #[cfg(feature = "NSDictionary")]
    unsafe impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
        #[cfg(feature = "NSString")]
        #[method(setValue:forKey:)]
        pub unsafe fn setValue_forKey(&mut self, value: Option<&ObjectType>, key: &NSString);
    }
);

extern_methods!(
    /// NSKeyValueCoding
    #[cfg(feature = "NSOrderedSet")]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other valueForKey:)]
        pub unsafe fn valueForKey(&self, key: &NSString) -> Retained<AnyObject>;

        #[cfg(feature = "NSString")]
        #[method(setValue:forKey:)]
        pub unsafe fn setValue_forKey(&self, value: Option<&AnyObject>, key: &NSString);
    }
);

extern_methods!(
    /// NSKeyValueCoding
    #[cfg(feature = "NSSet")]
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other valueForKey:)]
        pub unsafe fn valueForKey(&self, key: &NSString) -> Retained<AnyObject>;

        #[cfg(feature = "NSString")]
        #[method(setValue:forKey:)]
        pub unsafe fn setValue_forKey(&self, value: Option<&AnyObject>, key: &NSString);
    }
);
