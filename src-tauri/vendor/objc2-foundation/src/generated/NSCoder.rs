//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDecodingFailurePolicy(pub NSInteger);
impl NSDecodingFailurePolicy {
    #[doc(alias = "NSDecodingFailurePolicyRaiseException")]
    pub const RaiseException: Self = Self(0);
    #[doc(alias = "NSDecodingFailurePolicySetErrorAndReturn")]
    pub const SetErrorAndReturn: Self = Self(1);
}

unsafe impl Encode for NSDecodingFailurePolicy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSDecodingFailurePolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCoder;

    unsafe impl ClassType for NSCoder {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSCoder {}

extern_methods!(
    unsafe impl NSCoder {
        #[method(encodeValueOfObjCType:at:)]
        pub unsafe fn encodeValueOfObjCType_at(
            &self,
            r#type: NonNull<c_char>,
            addr: NonNull<c_void>,
        );

        #[cfg(feature = "NSData")]
        #[method(encodeDataObject:)]
        pub unsafe fn encodeDataObject(&self, data: &NSData);

        #[cfg(feature = "NSData")]
        #[method_id(@__retain_semantics Other decodeDataObject)]
        pub unsafe fn decodeDataObject(&self) -> Option<Retained<NSData>>;

        #[method(decodeValueOfObjCType:at:size:)]
        pub unsafe fn decodeValueOfObjCType_at_size(
            &self,
            r#type: NonNull<c_char>,
            data: NonNull<c_void>,
            size: NSUInteger,
        );

        #[cfg(feature = "NSString")]
        #[method(versionForClassName:)]
        pub unsafe fn versionForClassName(&self, class_name: &NSString) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSCoder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSExtendedCoder
    unsafe impl NSCoder {
        #[method(encodeObject:)]
        pub unsafe fn encodeObject(&self, object: Option<&AnyObject>);

        #[method(encodeRootObject:)]
        pub unsafe fn encodeRootObject(&self, root_object: &AnyObject);

        #[method(encodeBycopyObject:)]
        pub unsafe fn encodeBycopyObject(&self, an_object: Option<&AnyObject>);

        #[method(encodeByrefObject:)]
        pub unsafe fn encodeByrefObject(&self, an_object: Option<&AnyObject>);

        #[method(encodeConditionalObject:)]
        pub unsafe fn encodeConditionalObject(&self, object: Option<&AnyObject>);

        #[method(encodeArrayOfObjCType:count:at:)]
        pub unsafe fn encodeArrayOfObjCType_count_at(
            &self,
            r#type: NonNull<c_char>,
            count: NSUInteger,
            array: NonNull<c_void>,
        );

        #[method(encodeBytes:length:)]
        pub unsafe fn encodeBytes_length(&self, byteaddr: *mut c_void, length: NSUInteger);

        #[method_id(@__retain_semantics Other decodeObject)]
        pub unsafe fn decodeObject(&self) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSError")]
        #[method_id(@__retain_semantics Other decodeTopLevelObjectAndReturnError:_)]
        pub unsafe fn decodeTopLevelObjectAndReturnError(
            &self,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[method(decodeArrayOfObjCType:count:at:)]
        pub unsafe fn decodeArrayOfObjCType_count_at(
            &self,
            item_type: NonNull<c_char>,
            count: NSUInteger,
            array: NonNull<c_void>,
        );

        #[method(decodeBytesWithReturnedLength:)]
        pub unsafe fn decodeBytesWithReturnedLength(
            &self,
            lengthp: NonNull<NSUInteger>,
        ) -> *mut c_void;

        #[method(encodePropertyList:)]
        pub unsafe fn encodePropertyList(&self, a_property_list: &AnyObject);

        #[method_id(@__retain_semantics Other decodePropertyList)]
        pub unsafe fn decodePropertyList(&self) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSZone")]
        #[method(setObjectZone:)]
        pub unsafe fn setObjectZone(&self, zone: *mut NSZone);

        #[cfg(feature = "NSZone")]
        #[method(objectZone)]
        pub unsafe fn objectZone(&self) -> *mut NSZone;

        #[method(systemVersion)]
        pub unsafe fn systemVersion(&self) -> c_uint;

        #[method(allowsKeyedCoding)]
        pub unsafe fn allowsKeyedCoding(&self) -> bool;

        #[cfg(feature = "NSString")]
        #[method(encodeObject:forKey:)]
        pub unsafe fn encodeObject_forKey(&self, object: Option<&AnyObject>, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeConditionalObject:forKey:)]
        pub unsafe fn encodeConditionalObject_forKey(
            &self,
            object: Option<&AnyObject>,
            key: &NSString,
        );

        #[cfg(feature = "NSString")]
        #[method(encodeBool:forKey:)]
        pub unsafe fn encodeBool_forKey(&self, value: bool, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeInt:forKey:)]
        pub unsafe fn encodeInt_forKey(&self, value: c_int, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeInt32:forKey:)]
        pub unsafe fn encodeInt32_forKey(&self, value: i32, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeInt64:forKey:)]
        pub unsafe fn encodeInt64_forKey(&self, value: i64, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeFloat:forKey:)]
        pub unsafe fn encodeFloat_forKey(&self, value: c_float, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeDouble:forKey:)]
        pub unsafe fn encodeDouble_forKey(&self, value: c_double, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeBytes:length:forKey:)]
        pub unsafe fn encodeBytes_length_forKey(
            &self,
            bytes: *mut u8,
            length: NSUInteger,
            key: &NSString,
        );

        #[cfg(feature = "NSString")]
        #[method(containsValueForKey:)]
        pub unsafe fn containsValueForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other decodeObjectForKey:)]
        pub unsafe fn decodeObjectForKey(&self, key: &NSString) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method_id(@__retain_semantics Other decodeTopLevelObjectForKey:error:_)]
        pub unsafe fn decodeTopLevelObjectForKey_error(
            &self,
            key: &NSString,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[cfg(feature = "NSString")]
        #[method(decodeBoolForKey:)]
        pub unsafe fn decodeBoolForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "NSString")]
        #[method(decodeIntForKey:)]
        pub unsafe fn decodeIntForKey(&self, key: &NSString) -> c_int;

        #[cfg(feature = "NSString")]
        #[method(decodeInt32ForKey:)]
        pub unsafe fn decodeInt32ForKey(&self, key: &NSString) -> i32;

        #[cfg(feature = "NSString")]
        #[method(decodeInt64ForKey:)]
        pub unsafe fn decodeInt64ForKey(&self, key: &NSString) -> i64;

        #[cfg(feature = "NSString")]
        #[method(decodeFloatForKey:)]
        pub unsafe fn decodeFloatForKey(&self, key: &NSString) -> c_float;

        #[cfg(feature = "NSString")]
        #[method(decodeDoubleForKey:)]
        pub unsafe fn decodeDoubleForKey(&self, key: &NSString) -> c_double;

        #[cfg(feature = "NSString")]
        #[method(decodeBytesForKey:returnedLength:)]
        pub unsafe fn decodeBytesForKey_returnedLength(
            &self,
            key: &NSString,
            lengthp: *mut NSUInteger,
        ) -> *mut u8;

        #[cfg(feature = "NSString")]
        #[method(encodeInteger:forKey:)]
        pub unsafe fn encodeInteger_forKey(&self, value: NSInteger, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(decodeIntegerForKey:)]
        pub unsafe fn decodeIntegerForKey(&self, key: &NSString) -> NSInteger;

        #[method(requiresSecureCoding)]
        pub unsafe fn requiresSecureCoding(&self) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other decodeObjectOfClass:forKey:)]
        pub unsafe fn decodeObjectOfClass_forKey(
            &self,
            a_class: &AnyClass,
            key: &NSString,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method_id(@__retain_semantics Other decodeTopLevelObjectOfClass:forKey:error:_)]
        pub unsafe fn decodeTopLevelObjectOfClass_forKey_error(
            &self,
            a_class: &AnyClass,
            key: &NSString,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other decodeArrayOfObjectsOfClass:forKey:)]
        pub unsafe fn decodeArrayOfObjectsOfClass_forKey(
            &self,
            cls: &AnyClass,
            key: &NSString,
        ) -> Option<Retained<NSArray>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other decodeDictionaryWithKeysOfClass:objectsOfClass:forKey:)]
        pub unsafe fn decodeDictionaryWithKeysOfClass_objectsOfClass_forKey(
            &self,
            key_cls: &AnyClass,
            object_cls: &AnyClass,
            key: &NSString,
        ) -> Option<Retained<NSDictionary>>;

        #[cfg(all(feature = "NSSet", feature = "NSString"))]
        #[method_id(@__retain_semantics Other decodeObjectOfClasses:forKey:)]
        pub unsafe fn decodeObjectOfClasses_forKey(
            &self,
            classes: Option<&NSSet<TodoClass>>,
            key: &NSString,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "NSError", feature = "NSSet", feature = "NSString"))]
        #[method_id(@__retain_semantics Other decodeTopLevelObjectOfClasses:forKey:error:_)]
        pub unsafe fn decodeTopLevelObjectOfClasses_forKey_error(
            &self,
            classes: Option<&NSSet<TodoClass>>,
            key: &NSString,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[cfg(all(feature = "NSArray", feature = "NSSet", feature = "NSString"))]
        #[method_id(@__retain_semantics Other decodeArrayOfObjectsOfClasses:forKey:)]
        pub unsafe fn decodeArrayOfObjectsOfClasses_forKey(
            &self,
            classes: &NSSet<TodoClass>,
            key: &NSString,
        ) -> Option<Retained<NSArray>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSSet", feature = "NSString"))]
        #[method_id(@__retain_semantics Other decodeDictionaryWithKeysOfClasses:objectsOfClasses:forKey:)]
        pub unsafe fn decodeDictionaryWithKeysOfClasses_objectsOfClasses_forKey(
            &self,
            key_classes: &NSSet<TodoClass>,
            object_classes: &NSSet<TodoClass>,
            key: &NSString,
        ) -> Option<Retained<NSDictionary>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other decodePropertyListForKey:)]
        pub unsafe fn decodePropertyListForKey(
            &self,
            key: &NSString,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSSet")]
        #[method_id(@__retain_semantics Other allowedClasses)]
        pub unsafe fn allowedClasses(&self) -> Option<Retained<NSSet<TodoClass>>>;

        #[cfg(feature = "NSError")]
        #[method(failWithError:)]
        pub unsafe fn failWithError(&self, error: &NSError);

        #[method(decodingFailurePolicy)]
        pub unsafe fn decodingFailurePolicy(&self) -> NSDecodingFailurePolicy;

        #[cfg(feature = "NSError")]
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Retained<NSError>>;
    }
);

extern "C" {
    #[deprecated = "Not supported"]
    pub fn NXReadNSObjectFromCoder(decoder: &NSCoder) -> *mut NSObject;
}

extern_methods!(
    /// NSTypedstreamCompatibility
    unsafe impl NSCoder {
        #[deprecated = "Not supported"]
        #[method(encodeNXObject:)]
        pub unsafe fn encodeNXObject(&self, object: &AnyObject);

        #[deprecated = "Not supported"]
        #[method_id(@__retain_semantics Other decodeNXObject)]
        pub unsafe fn decodeNXObject(&self) -> Option<Retained<AnyObject>>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSCoder {
        #[deprecated]
        #[method(decodeValueOfObjCType:at:)]
        pub unsafe fn decodeValueOfObjCType_at(
            &self,
            r#type: NonNull<c_char>,
            data: NonNull<c_void>,
        );
    }
);
