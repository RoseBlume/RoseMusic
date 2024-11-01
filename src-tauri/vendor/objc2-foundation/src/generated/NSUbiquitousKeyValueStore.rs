//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSUbiquitousKeyValueStore;

    unsafe impl ClassType for NSUbiquitousKeyValueStore {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSUbiquitousKeyValueStore {}

extern_methods!(
    unsafe impl NSUbiquitousKeyValueStore {
        #[method_id(@__retain_semantics Other defaultStore)]
        pub unsafe fn defaultStore() -> Retained<NSUbiquitousKeyValueStore>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(&self, a_key: &NSString) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(&self, an_object: Option<&AnyObject>, a_key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(removeObjectForKey:)]
        pub unsafe fn removeObjectForKey(&self, a_key: &NSString);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other stringForKey:)]
        pub unsafe fn stringForKey(&self, a_key: &NSString) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other arrayForKey:)]
        pub unsafe fn arrayForKey(&self, a_key: &NSString) -> Option<Retained<NSArray>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other dictionaryForKey:)]
        pub unsafe fn dictionaryForKey(
            &self,
            a_key: &NSString,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "NSData", feature = "NSString"))]
        #[method_id(@__retain_semantics Other dataForKey:)]
        pub unsafe fn dataForKey(&self, a_key: &NSString) -> Option<Retained<NSData>>;

        #[cfg(feature = "NSString")]
        #[method(longLongForKey:)]
        pub unsafe fn longLongForKey(&self, a_key: &NSString) -> c_longlong;

        #[cfg(feature = "NSString")]
        #[method(doubleForKey:)]
        pub unsafe fn doubleForKey(&self, a_key: &NSString) -> c_double;

        #[cfg(feature = "NSString")]
        #[method(boolForKey:)]
        pub unsafe fn boolForKey(&self, a_key: &NSString) -> bool;

        #[cfg(feature = "NSString")]
        #[method(setString:forKey:)]
        pub unsafe fn setString_forKey(&self, a_string: Option<&NSString>, a_key: &NSString);

        #[cfg(all(feature = "NSData", feature = "NSString"))]
        #[method(setData:forKey:)]
        pub unsafe fn setData_forKey(&self, a_data: Option<&NSData>, a_key: &NSString);

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method(setArray:forKey:)]
        pub unsafe fn setArray_forKey(&self, an_array: Option<&NSArray>, a_key: &NSString);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method(setDictionary:forKey:)]
        pub unsafe fn setDictionary_forKey(
            &self,
            a_dictionary: Option<&NSDictionary<NSString, AnyObject>>,
            a_key: &NSString,
        );

        #[cfg(feature = "NSString")]
        #[method(setLongLong:forKey:)]
        pub unsafe fn setLongLong_forKey(&self, value: c_longlong, a_key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(setDouble:forKey:)]
        pub unsafe fn setDouble_forKey(&self, value: c_double, a_key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(setBool:forKey:)]
        pub unsafe fn setBool_forKey(&self, value: bool, a_key: &NSString);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(
            &self,
        ) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[method(synchronize)]
        pub unsafe fn synchronize(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSUbiquitousKeyValueStore {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUbiquitousKeyValueStoreDidChangeExternallyNotification:
        &'static NSNotificationName;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSUbiquitousKeyValueStoreChangeReasonKey: &'static NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSUbiquitousKeyValueStoreChangedKeysKey: &'static NSString;
}

pub const NSUbiquitousKeyValueStoreServerChange: NSInteger = 0;
pub const NSUbiquitousKeyValueStoreInitialSyncChange: NSInteger = 1;
pub const NSUbiquitousKeyValueStoreQuotaViolationChange: NSInteger = 2;
pub const NSUbiquitousKeyValueStoreAccountChange: NSInteger = 3;