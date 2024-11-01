//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static NSCoreDataCoreSpotlightDelegateIndexDidUpdateNotification:
        &'static NSNotificationName;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCoreDataCoreSpotlightDelegate;

    unsafe impl ClassType for NSCoreDataCoreSpotlightDelegate {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSCoreDataCoreSpotlightDelegate {}

extern_methods!(
    unsafe impl NSCoreDataCoreSpotlightDelegate {
        #[method(isIndexingEnabled)]
        pub unsafe fn isIndexingEnabled(&self) -> bool;

        #[method_id(@__retain_semantics Other domainIdentifier)]
        pub unsafe fn domainIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other indexName)]
        pub unsafe fn indexName(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(
            feature = "NSPersistentStoreCoordinator",
            feature = "NSPersistentStoreDescription"
        ))]
        #[method_id(@__retain_semantics Init initForStoreWithDescription:coordinator:)]
        pub unsafe fn initForStoreWithDescription_coordinator(
            this: Allocated<Self>,
            description: &NSPersistentStoreDescription,
            psc: &NSPersistentStoreCoordinator,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "NSManagedObjectModel",
            feature = "NSPersistentStoreDescription"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initForStoreWithDescription:model:)]
        pub unsafe fn initForStoreWithDescription_model(
            this: Allocated<Self>,
            description: &NSPersistentStoreDescription,
            model: &NSManagedObjectModel,
        ) -> Retained<Self>;

        #[method(startSpotlightIndexing)]
        pub unsafe fn startSpotlightIndexing(&self);

        #[method(stopSpotlightIndexing)]
        pub unsafe fn stopSpotlightIndexing(&self);

        #[cfg(feature = "block2")]
        #[method(deleteSpotlightIndexWithCompletionHandler:)]
        pub unsafe fn deleteSpotlightIndexWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSCoreDataCoreSpotlightDelegate {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
