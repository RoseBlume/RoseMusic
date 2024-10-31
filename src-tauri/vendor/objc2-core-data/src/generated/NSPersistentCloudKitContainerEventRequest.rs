//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPersistentStoreRequest")]
    pub struct NSPersistentCloudKitContainerEventRequest;

    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl ClassType for NSPersistentCloudKitContainerEventRequest {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl NSCopying for NSPersistentCloudKitContainerEventRequest {}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl NSObjectProtocol for NSPersistentCloudKitContainerEventRequest {}

extern_methods!(
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl NSPersistentCloudKitContainerEventRequest {
        #[cfg(feature = "NSPersistentStoreResult")]
        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSPersistentCloudKitContainerEventResultType;

        #[cfg(feature = "NSPersistentStoreResult")]
        #[method(setResultType:)]
        pub unsafe fn setResultType(
            &self,
            result_type: NSPersistentCloudKitContainerEventResultType,
        );

        #[method_id(@__retain_semantics Other fetchEventsAfterDate:)]
        pub unsafe fn fetchEventsAfterDate(date: &NSDate) -> Retained<Self>;

        #[cfg(feature = "NSPersistentCloudKitContainerEvent")]
        #[method_id(@__retain_semantics Other fetchEventsAfterEvent:)]
        pub unsafe fn fetchEventsAfterEvent(
            event: Option<&NSPersistentCloudKitContainerEvent>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchEventsMatchingFetchRequest:)]
        pub unsafe fn fetchEventsMatchingFetchRequest(
            fetch_request: &NSFetchRequest,
        ) -> Retained<Self>;

        #[cfg(feature = "NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchRequestForEvents)]
        pub unsafe fn fetchRequestForEvents() -> Retained<NSFetchRequest>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl NSPersistentCloudKitContainerEventRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);