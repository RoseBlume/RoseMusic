//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CKOperation")]
    pub struct CKFetchShareMetadataOperation;

    #[cfg(feature = "CKOperation")]
    unsafe impl ClassType for CKFetchShareMetadataOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CKOperation")]
unsafe impl NSObjectProtocol for CKFetchShareMetadataOperation {}

extern_methods!(
    #[cfg(feature = "CKOperation")]
    unsafe impl CKFetchShareMetadataOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithShareURLs:)]
        pub unsafe fn initWithShareURLs(
            this: Allocated<Self>,
            share_ur_ls: &NSArray<NSURL>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other shareURLs)]
        pub unsafe fn shareURLs(&self) -> Option<Retained<NSArray<NSURL>>>;

        #[method(setShareURLs:)]
        pub unsafe fn setShareURLs(&self, share_ur_ls: Option<&NSArray<NSURL>>);

        #[method(shouldFetchRootRecord)]
        pub unsafe fn shouldFetchRootRecord(&self) -> bool;

        #[method(setShouldFetchRootRecord:)]
        pub unsafe fn setShouldFetchRootRecord(&self, should_fetch_root_record: bool);

        #[cfg(feature = "CKRecord")]
        #[method_id(@__retain_semantics Other rootRecordDesiredKeys)]
        pub unsafe fn rootRecordDesiredKeys(&self) -> Option<Retained<NSArray<CKRecordFieldKey>>>;

        #[cfg(feature = "CKRecord")]
        #[method(setRootRecordDesiredKeys:)]
        pub unsafe fn setRootRecordDesiredKeys(
            &self,
            root_record_desired_keys: Option<&NSArray<CKRecordFieldKey>>,
        );

        #[cfg(all(feature = "CKShareMetadata", feature = "block2"))]
        #[method(perShareMetadataBlock)]
        pub unsafe fn perShareMetadataBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<NSURL>, *mut CKShareMetadata, *mut NSError)>;

        #[cfg(all(feature = "CKShareMetadata", feature = "block2"))]
        #[method(setPerShareMetadataBlock:)]
        pub unsafe fn setPerShareMetadataBlock(
            &self,
            per_share_metadata_block: Option<
                &block2::Block<dyn Fn(NonNull<NSURL>, *mut CKShareMetadata, *mut NSError)>,
            >,
        );

        #[cfg(feature = "block2")]
        #[method(fetchShareMetadataCompletionBlock)]
        pub unsafe fn fetchShareMetadataCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(*mut NSError)>;

        #[cfg(feature = "block2")]
        #[method(setFetchShareMetadataCompletionBlock:)]
        pub unsafe fn setFetchShareMetadataCompletionBlock(
            &self,
            fetch_share_metadata_completion_block: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CKOperation")]
    unsafe impl CKFetchShareMetadataOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
