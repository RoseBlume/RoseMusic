//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static NSSQLiteStoreType: &'static NSString;
}

extern "C" {
    pub static NSXMLStoreType: &'static NSString;
}

extern "C" {
    pub static NSBinaryStoreType: &'static NSString;
}

extern "C" {
    pub static NSInMemoryStoreType: &'static NSString;
}

extern "C" {
    pub static NSStoreTypeKey: &'static NSString;
}

extern "C" {
    pub static NSStoreUUIDKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreCoordinatorStoresWillChangeNotification: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreCoordinatorStoresDidChangeNotification: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreCoordinatorWillRemoveStoreNotification: &'static NSString;
}

extern "C" {
    pub static NSAddedPersistentStoresKey: &'static NSString;
}

extern "C" {
    pub static NSRemovedPersistentStoresKey: &'static NSString;
}

extern "C" {
    pub static NSUUIDChangedPersistentStoresKey: &'static NSString;
}

extern "C" {
    pub static NSReadOnlyPersistentStoreOption: &'static NSString;
}

extern "C" {
    pub static NSValidateXMLStoreOption: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreTimeoutOption: &'static NSString;
}

extern "C" {
    pub static NSSQLitePragmasOption: &'static NSString;
}

extern "C" {
    pub static NSSQLiteAnalyzeOption: &'static NSString;
}

extern "C" {
    pub static NSSQLiteManualVacuumOption: &'static NSString;
}

extern "C" {
    pub static NSIgnorePersistentStoreVersioningOption: &'static NSString;
}

extern "C" {
    pub static NSMigratePersistentStoresAutomaticallyOption: &'static NSString;
}

extern "C" {
    pub static NSInferMappingModelAutomaticallyOption: &'static NSString;
}

extern "C" {
    pub static NSStoreModelVersionHashesKey: &'static NSString;
}

extern "C" {
    pub static NSStoreModelVersionIdentifiersKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreOSCompatibility: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreConnectionPoolMaxSizeKey: &'static NSString;
}

extern "C" {
    pub static NSCoreDataCoreSpotlightExporter: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreStagedMigrationManagerOptionKey: &'static NSString;
}

extern "C" {
    pub static NSXMLExternalRecordType: &'static NSString;
}

extern "C" {
    pub static NSBinaryExternalRecordType: &'static NSString;
}

extern "C" {
    pub static NSExternalRecordsFileFormatOption: &'static NSString;
}

extern "C" {
    pub static NSExternalRecordsDirectoryOption: &'static NSString;
}

extern "C" {
    pub static NSExternalRecordExtensionOption: &'static NSString;
}

extern "C" {
    pub static NSEntityNameInPathKey: &'static NSString;
}

extern "C" {
    pub static NSStoreUUIDInPathKey: &'static NSString;
}

extern "C" {
    pub static NSStorePathKey: &'static NSString;
}

extern "C" {
    pub static NSModelPathKey: &'static NSString;
}

extern "C" {
    pub static NSObjectURIKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreForceDestroyOption: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreFileProtectionKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentHistoryTrackingKey: &'static NSString;
}

extern "C" {
    pub static NSBinaryStoreSecureDecodingClasses: &'static NSString;
}

extern "C" {
    pub static NSBinaryStoreInsecureDecodingCompatibilityOption: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreRemoteChangeNotificationPostOptionKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreRemoteChangeNotification: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreURLKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentHistoryTokenKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreDeferredLightweightMigrationOptionKey: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentStoreCoordinator;

    unsafe impl ClassType for NSPersistentStoreCoordinator {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSPersistentStoreCoordinator {}

unsafe impl Sync for NSPersistentStoreCoordinator {}

unsafe impl NSLocking for NSPersistentStoreCoordinator {}

unsafe impl NSObjectProtocol for NSPersistentStoreCoordinator {}

extern_methods!(
    unsafe impl NSPersistentStoreCoordinator {
        #[cfg(feature = "NSManagedObjectModel")]
        #[method_id(@__retain_semantics Init initWithManagedObjectModel:)]
        pub unsafe fn initWithManagedObjectModel(
            this: Allocated<Self>,
            model: &NSManagedObjectModel,
        ) -> Retained<Self>;

        #[cfg(feature = "NSManagedObjectModel")]
        #[method_id(@__retain_semantics Other managedObjectModel)]
        pub unsafe fn managedObjectModel(&self) -> Retained<NSManagedObjectModel>;

        #[cfg(feature = "NSPersistentStore")]
        #[method_id(@__retain_semantics Other persistentStores)]
        pub unsafe fn persistentStores(&self) -> Retained<NSArray<NSPersistentStore>>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "NSPersistentStore")]
        #[method_id(@__retain_semantics Other persistentStoreForURL:)]
        pub unsafe fn persistentStoreForURL(
            &self,
            url: &NSURL,
        ) -> Option<Retained<NSPersistentStore>>;

        #[cfg(feature = "NSPersistentStore")]
        #[method_id(@__retain_semantics Other URLForPersistentStore:)]
        pub unsafe fn URLForPersistentStore(&self, store: &NSPersistentStore) -> Retained<NSURL>;

        #[cfg(feature = "NSPersistentStore")]
        #[method(setURL:forPersistentStore:)]
        pub unsafe fn setURL_forPersistentStore(
            &self,
            url: &NSURL,
            store: &NSPersistentStore,
        ) -> bool;

        #[cfg(feature = "NSPersistentStore")]
        #[method_id(@__retain_semantics Other addPersistentStoreWithType:configuration:URL:options:error:_)]
        pub unsafe fn addPersistentStoreWithType_configuration_URL_options_error(
            &self,
            store_type: &NSString,
            configuration: Option<&NSString>,
            store_url: Option<&NSURL>,
            options: Option<&NSDictionary>,
        ) -> Result<Retained<NSPersistentStore>, Retained<NSError>>;

        #[cfg(all(feature = "NSPersistentStoreDescription", feature = "block2"))]
        #[method(addPersistentStoreWithDescription:completionHandler:)]
        pub unsafe fn addPersistentStoreWithDescription_completionHandler(
            &self,
            store_description: &NSPersistentStoreDescription,
            block: &block2::Block<dyn Fn(NonNull<NSPersistentStoreDescription>, *mut NSError)>,
        );

        #[cfg(feature = "NSPersistentStore")]
        #[method(removePersistentStore:error:_)]
        pub unsafe fn removePersistentStore_error(
            &self,
            store: &NSPersistentStore,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "NSPersistentStore")]
        #[method(setMetadata:forPersistentStore:)]
        pub unsafe fn setMetadata_forPersistentStore(
            &self,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
            store: &NSPersistentStore,
        );

        #[cfg(feature = "NSPersistentStore")]
        #[method_id(@__retain_semantics Other metadataForPersistentStore:)]
        pub unsafe fn metadataForPersistentStore(
            &self,
            store: &NSPersistentStore,
        ) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[cfg(feature = "NSManagedObjectID")]
        #[method_id(@__retain_semantics Other managedObjectIDForURIRepresentation:)]
        pub unsafe fn managedObjectIDForURIRepresentation(
            &self,
            url: &NSURL,
        ) -> Option<Retained<NSManagedObjectID>>;

        #[cfg(all(
            feature = "NSManagedObjectContext",
            feature = "NSPersistentStoreRequest"
        ))]
        #[method_id(@__retain_semantics Other executeRequest:withContext:error:_)]
        pub unsafe fn executeRequest_withContext_error(
            &self,
            request: &NSPersistentStoreRequest,
            context: &NSManagedObjectContext,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other registeredStoreTypes)]
        pub unsafe fn registeredStoreTypes() -> Retained<NSDictionary<NSString, NSValue>>;

        #[method(registerStoreClass:forStoreType:)]
        pub unsafe fn registerStoreClass_forStoreType(
            store_class: Option<&AnyClass>,
            store_type: &NSString,
        );

        #[method_id(@__retain_semantics Other metadataForPersistentStoreOfType:URL:options:error:_)]
        pub unsafe fn metadataForPersistentStoreOfType_URL_options_error(
            store_type: &NSString,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Result<Retained<NSDictionary<NSString, AnyObject>>, Retained<NSError>>;

        #[method(setMetadata:forPersistentStoreOfType:URL:options:error:_)]
        pub unsafe fn setMetadata_forPersistentStoreOfType_URL_options_error(
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
            store_type: &NSString,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Result<(), Retained<NSError>>;

        #[deprecated = "Spotlight integration is deprecated. Use CoreSpotlight integration instead."]
        #[method_id(@__retain_semantics Other elementsDerivedFromExternalRecordURL:)]
        pub unsafe fn elementsDerivedFromExternalRecordURL(
            file_url: &NSURL,
        ) -> Retained<NSDictionary>;

        #[cfg(feature = "NSPersistentStore")]
        #[deprecated = "Spotlight integration is deprecated. Use CoreSpotlight integration instead."]
        #[method_id(@__retain_semantics Other importStoreWithIdentifier:fromExternalRecordsDirectory:toURL:options:withType:error:_)]
        pub unsafe fn importStoreWithIdentifier_fromExternalRecordsDirectory_toURL_options_withType_error(
            &self,
            store_identifier: Option<&NSString>,
            external_records_url: &NSURL,
            destination_url: &NSURL,
            options: Option<&NSDictionary>,
            store_type: &NSString,
        ) -> Result<Retained<NSPersistentStore>, Retained<NSError>>;

        #[cfg(feature = "NSPersistentStore")]
        #[method_id(@__retain_semantics Other migratePersistentStore:toURL:options:withType:error:_)]
        pub unsafe fn migratePersistentStore_toURL_options_withType_error(
            &self,
            store: &NSPersistentStore,
            url: &NSURL,
            options: Option<&NSDictionary>,
            store_type: &NSString,
        ) -> Result<Retained<NSPersistentStore>, Retained<NSError>>;

        #[method(destroyPersistentStoreAtURL:withType:options:error:_)]
        pub unsafe fn destroyPersistentStoreAtURL_withType_options_error(
            &self,
            url: &NSURL,
            store_type: &NSString,
            options: Option<&NSDictionary>,
        ) -> Result<(), Retained<NSError>>;

        #[method(replacePersistentStoreAtURL:destinationOptions:withPersistentStoreFromURL:sourceOptions:storeType:error:_)]
        pub unsafe fn replacePersistentStoreAtURL_destinationOptions_withPersistentStoreFromURL_sourceOptions_storeType_error(
            &self,
            destination_url: &NSURL,
            destination_options: Option<&NSDictionary>,
            source_url: &NSURL,
            source_options: Option<&NSDictionary>,
            store_type: &NSString,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "block2")]
        #[method(performBlock:)]
        pub unsafe fn performBlock(&self, block: &block2::Block<dyn Fn()>);

        #[cfg(feature = "block2")]
        #[method(performBlockAndWait:)]
        pub unsafe fn performBlockAndWait(&self, block: &block2::Block<dyn Fn() + '_>);

        #[cfg(feature = "NSPersistentHistoryToken")]
        #[method_id(@__retain_semantics Other currentPersistentHistoryTokenFromStores:)]
        pub unsafe fn currentPersistentHistoryTokenFromStores(
            &self,
            stores: Option<&NSArray>,
        ) -> Option<Retained<NSPersistentHistoryToken>>;

        #[method(finishDeferredLightweightMigration:_)]
        pub unsafe fn finishDeferredLightweightMigration(&self) -> Result<(), Retained<NSError>>;

        #[method(finishDeferredLightweightMigrationTask:_)]
        pub unsafe fn finishDeferredLightweightMigrationTask(
            &self,
        ) -> Result<(), Retained<NSError>>;

        #[deprecated = "Use -metadataForPersistentStoreOfType:URL:options:error: and pass in an options dictionary matching addPersistentStoreWithType"]
        #[method_id(@__retain_semantics Other metadataForPersistentStoreWithURL:error:_)]
        pub unsafe fn metadataForPersistentStoreWithURL_error(
            url: &NSURL,
        ) -> Result<Retained<NSDictionary>, Retained<NSError>>;

        #[deprecated = "Use -performBlockAndWait: instead"]
        #[method(lock)]
        pub unsafe fn lock(&self);

        #[deprecated = "Use -performBlockAndWait: instead"]
        #[method(unlock)]
        pub unsafe fn unlock(&self);

        #[deprecated = "Use -performBlock: instead"]
        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;

        #[deprecated = "Use -metadataForPersistentStoreOfType:URL:options:error: and pass in an options dictionary matching addPersistentStoreWithType"]
        #[method_id(@__retain_semantics Other metadataForPersistentStoreOfType:URL:error:_)]
        pub unsafe fn metadataForPersistentStoreOfType_URL_error(
            store_type: Option<&NSString>,
            url: &NSURL,
        ) -> Result<Retained<NSDictionary<NSString, AnyObject>>, Retained<NSError>>;

        #[deprecated = "Use  -setMetadata:forPersistentStoreOfType:URL:options:error: and pass in an options dictionary matching addPersistentStoreWithType"]
        #[method(setMetadata:forPersistentStoreOfType:URL:error:_)]
        pub unsafe fn setMetadata_forPersistentStoreOfType_URL_error(
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
            store_type: Option<&NSString>,
            url: &NSURL,
        ) -> Result<(), Retained<NSError>>;

        #[deprecated = "Please see the release notes and Core Data documentation."]
        #[method(removeUbiquitousContentAndPersistentStoreAtURL:options:error:_)]
        pub unsafe fn removeUbiquitousContentAndPersistentStoreAtURL_options_error(
            store_url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPersistentStoreCoordinator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_ENUM
#[deprecated = "Please see the release notes and Core Data documentation."]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPersistentStoreUbiquitousTransitionType(pub NSUInteger);
impl NSPersistentStoreUbiquitousTransitionType {
    #[deprecated = "Please see the release notes and Core Data documentation."]
    #[doc(alias = "NSPersistentStoreUbiquitousTransitionTypeAccountAdded")]
    pub const AccountAdded: Self = Self(1);
    #[deprecated = "Please see the release notes and Core Data documentation."]
    #[doc(alias = "NSPersistentStoreUbiquitousTransitionTypeAccountRemoved")]
    pub const AccountRemoved: Self = Self(2);
    #[deprecated = "Please see the release notes and Core Data documentation."]
    #[doc(alias = "NSPersistentStoreUbiquitousTransitionTypeContentRemoved")]
    pub const ContentRemoved: Self = Self(3);
    #[deprecated = "Please see the release notes and Core Data documentation."]
    #[doc(alias = "NSPersistentStoreUbiquitousTransitionTypeInitialImportCompleted")]
    pub const InitialImportCompleted: Self = Self(4);
}

unsafe impl Encode for NSPersistentStoreUbiquitousTransitionType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPersistentStoreUbiquitousTransitionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static NSPersistentStoreUbiquitousContentNameKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreUbiquitousContentURLKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreDidImportUbiquitousContentChangesNotification: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreUbiquitousTransitionTypeKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreUbiquitousPeerTokenOption: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreRemoveUbiquitousMetadataOption: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreUbiquitousContainerIdentifierKey: &'static NSString;
}

extern "C" {
    pub static NSPersistentStoreRebuildFromUbiquitousContentOption: &'static NSString;
}