//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static kCIFilterGeneratorExportedKey: &'static NSString;
}

extern "C" {
    pub static kCIFilterGeneratorExportedKeyTargetObject: &'static NSString;
}

extern "C" {
    pub static kCIFilterGeneratorExportedKeyName: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIFilterGenerator;

    unsafe impl ClassType for CIFilterGenerator {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CIFilterConstructor")]
unsafe impl CIFilterConstructor for CIFilterGenerator {}

unsafe impl NSCoding for CIFilterGenerator {}

unsafe impl NSCopying for CIFilterGenerator {}

unsafe impl NSObjectProtocol for CIFilterGenerator {}

unsafe impl NSSecureCoding for CIFilterGenerator {}

extern_methods!(
    unsafe impl CIFilterGenerator {
        #[method_id(@__retain_semantics Other filterGenerator)]
        pub unsafe fn filterGenerator() -> Retained<CIFilterGenerator>;

        #[method_id(@__retain_semantics Other filterGeneratorWithContentsOfURL:)]
        pub unsafe fn filterGeneratorWithContentsOfURL(
            a_url: &NSURL,
        ) -> Option<Retained<CIFilterGenerator>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Allocated<Self>,
            a_url: &NSURL,
        ) -> Option<Retained<Self>>;

        #[method(connectObject:withKey:toObject:withKey:)]
        pub unsafe fn connectObject_withKey_toObject_withKey(
            &self,
            source_object: &AnyObject,
            source_key: Option<&NSString>,
            target_object: &AnyObject,
            target_key: &NSString,
        );

        #[method(disconnectObject:withKey:toObject:withKey:)]
        pub unsafe fn disconnectObject_withKey_toObject_withKey(
            &self,
            source_object: &AnyObject,
            source_key: &NSString,
            target_object: &AnyObject,
            target_key: &NSString,
        );

        #[method(exportKey:fromObject:withName:)]
        pub unsafe fn exportKey_fromObject_withName(
            &self,
            key: &NSString,
            target_object: &AnyObject,
            exported_key_name: Option<&NSString>,
        );

        #[method(removeExportedKey:)]
        pub unsafe fn removeExportedKey(&self, exported_key_name: &NSString);

        #[method_id(@__retain_semantics Other exportedKeys)]
        pub unsafe fn exportedKeys(&self) -> Retained<NSDictionary>;

        #[method(setAttributes:forExportedKey:)]
        pub unsafe fn setAttributes_forExportedKey(
            &self,
            attributes: &NSDictionary,
            key: &NSString,
        );

        #[method_id(@__retain_semantics Other classAttributes)]
        pub unsafe fn classAttributes(&self) -> Retained<NSDictionary>;

        #[method(setClassAttributes:)]
        pub unsafe fn setClassAttributes(&self, class_attributes: &NSDictionary);

        #[cfg(feature = "CIFilter")]
        #[method_id(@__retain_semantics Other filter)]
        pub unsafe fn filter(&self) -> Retained<CIFilter>;

        #[method(registerFilterName:)]
        pub unsafe fn registerFilterName(&self, name: &NSString);

        #[method(writeToURL:atomically:)]
        pub unsafe fn writeToURL_atomically(&self, a_url: &NSURL, flag: bool) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIFilterGenerator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
