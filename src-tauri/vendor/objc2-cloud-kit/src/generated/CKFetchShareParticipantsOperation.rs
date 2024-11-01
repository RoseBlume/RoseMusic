//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CKOperation")]
    pub struct CKFetchShareParticipantsOperation;

    #[cfg(feature = "CKOperation")]
    unsafe impl ClassType for CKFetchShareParticipantsOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CKOperation")]
unsafe impl NSObjectProtocol for CKFetchShareParticipantsOperation {}

extern_methods!(
    #[cfg(feature = "CKOperation")]
    unsafe impl CKFetchShareParticipantsOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "CKUserIdentityLookupInfo")]
        #[method_id(@__retain_semantics Init initWithUserIdentityLookupInfos:)]
        pub unsafe fn initWithUserIdentityLookupInfos(
            this: Allocated<Self>,
            user_identity_lookup_infos: &NSArray<CKUserIdentityLookupInfo>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKUserIdentityLookupInfo")]
        #[method_id(@__retain_semantics Other userIdentityLookupInfos)]
        pub unsafe fn userIdentityLookupInfos(
            &self,
        ) -> Option<Retained<NSArray<CKUserIdentityLookupInfo>>>;

        #[cfg(feature = "CKUserIdentityLookupInfo")]
        #[method(setUserIdentityLookupInfos:)]
        pub unsafe fn setUserIdentityLookupInfos(
            &self,
            user_identity_lookup_infos: Option<&NSArray<CKUserIdentityLookupInfo>>,
        );

        #[cfg(all(feature = "CKShareParticipant", feature = "block2"))]
        #[deprecated = "Use perShareParticipantCompletionBlock instead, which surfaces per-share-participant errors"]
        #[method(shareParticipantFetchedBlock)]
        pub unsafe fn shareParticipantFetchedBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKShareParticipant>)>;

        #[cfg(all(feature = "CKShareParticipant", feature = "block2"))]
        #[deprecated = "Use perShareParticipantCompletionBlock instead, which surfaces per-share-participant errors"]
        #[method(setShareParticipantFetchedBlock:)]
        pub unsafe fn setShareParticipantFetchedBlock(
            &self,
            share_participant_fetched_block: Option<
                &block2::Block<dyn Fn(NonNull<CKShareParticipant>)>,
            >,
        );

        #[cfg(all(
            feature = "CKShareParticipant",
            feature = "CKUserIdentityLookupInfo",
            feature = "block2"
        ))]
        #[method(perShareParticipantCompletionBlock)]
        pub unsafe fn perShareParticipantCompletionBlock(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(NonNull<CKUserIdentityLookupInfo>, *mut CKShareParticipant, *mut NSError),
        >;

        #[cfg(all(
            feature = "CKShareParticipant",
            feature = "CKUserIdentityLookupInfo",
            feature = "block2"
        ))]
        #[method(setPerShareParticipantCompletionBlock:)]
        pub unsafe fn setPerShareParticipantCompletionBlock(
            &self,
            per_share_participant_completion_block: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<CKUserIdentityLookupInfo>,
                        *mut CKShareParticipant,
                        *mut NSError,
                    ),
                >,
            >,
        );

        #[cfg(feature = "block2")]
        #[method(fetchShareParticipantsCompletionBlock)]
        pub unsafe fn fetchShareParticipantsCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(*mut NSError)>;

        #[cfg(feature = "block2")]
        #[method(setFetchShareParticipantsCompletionBlock:)]
        pub unsafe fn setFetchShareParticipantsCompletionBlock(
            &self,
            fetch_share_participants_completion_block: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CKOperation")]
    unsafe impl CKFetchShareParticipantsOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
