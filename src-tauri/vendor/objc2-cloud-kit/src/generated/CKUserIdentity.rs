//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKUserIdentity;

    unsafe impl ClassType for CKUserIdentity {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CKUserIdentity {}

unsafe impl NSCopying for CKUserIdentity {}

unsafe impl NSObjectProtocol for CKUserIdentity {}

unsafe impl NSSecureCoding for CKUserIdentity {}

extern_methods!(
    unsafe impl CKUserIdentity {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CKUserIdentityLookupInfo")]
        #[method_id(@__retain_semantics Other lookupInfo)]
        pub unsafe fn lookupInfo(&self) -> Option<Retained<CKUserIdentityLookupInfo>>;

        #[method_id(@__retain_semantics Other nameComponents)]
        pub unsafe fn nameComponents(&self) -> Option<Retained<NSPersonNameComponents>>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other userRecordID)]
        pub unsafe fn userRecordID(&self) -> Option<Retained<CKRecordID>>;

        #[method_id(@__retain_semantics Other contactIdentifiers)]
        pub unsafe fn contactIdentifiers(&self) -> Retained<NSArray<NSString>>;

        #[method(hasiCloudAccount)]
        pub unsafe fn hasiCloudAccount(&self) -> bool;
    }
);