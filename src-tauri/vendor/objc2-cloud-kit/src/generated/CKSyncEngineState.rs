//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineState;

    unsafe impl ClassType for CKSyncEngineState {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineState {}

unsafe impl Sync for CKSyncEngineState {}

unsafe impl NSObjectProtocol for CKSyncEngineState {}

extern_methods!(
    unsafe impl CKSyncEngineState {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other pendingRecordZoneChanges)]
        pub unsafe fn pendingRecordZoneChanges(
            &self,
        ) -> Retained<NSArray<CKSyncEnginePendingRecordZoneChange>>;

        #[method_id(@__retain_semantics Other pendingDatabaseChanges)]
        pub unsafe fn pendingDatabaseChanges(
            &self,
        ) -> Retained<NSArray<CKSyncEnginePendingDatabaseChange>>;

        #[method(hasPendingUntrackedChanges)]
        pub unsafe fn hasPendingUntrackedChanges(&self) -> bool;

        #[method(setHasPendingUntrackedChanges:)]
        pub unsafe fn setHasPendingUntrackedChanges(&self, has_pending_untracked_changes: bool);

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneIDsWithUnfetchedServerChanges)]
        pub unsafe fn zoneIDsWithUnfetchedServerChanges(&self)
            -> Retained<NSArray<CKRecordZoneID>>;

        #[method(addPendingRecordZoneChanges:)]
        pub unsafe fn addPendingRecordZoneChanges(
            &self,
            changes: &NSArray<CKSyncEnginePendingRecordZoneChange>,
        );

        #[method(removePendingRecordZoneChanges:)]
        pub unsafe fn removePendingRecordZoneChanges(
            &self,
            changes: &NSArray<CKSyncEnginePendingRecordZoneChange>,
        );

        #[method(addPendingDatabaseChanges:)]
        pub unsafe fn addPendingDatabaseChanges(
            &self,
            changes: &NSArray<CKSyncEnginePendingDatabaseChange>,
        );

        #[method(removePendingDatabaseChanges:)]
        pub unsafe fn removePendingDatabaseChanges(
            &self,
            changes: &NSArray<CKSyncEnginePendingDatabaseChange>,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineStateSerialization;

    unsafe impl ClassType for CKSyncEngineStateSerialization {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineStateSerialization {}

unsafe impl Sync for CKSyncEngineStateSerialization {}

unsafe impl NSCoding for CKSyncEngineStateSerialization {}

unsafe impl NSObjectProtocol for CKSyncEngineStateSerialization {}

unsafe impl NSSecureCoding for CKSyncEngineStateSerialization {}

extern_methods!(
    unsafe impl CKSyncEngineStateSerialization {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKSyncEnginePendingRecordZoneChangeType(pub NSInteger);
impl CKSyncEnginePendingRecordZoneChangeType {
    #[doc(alias = "CKSyncEnginePendingRecordZoneChangeTypeSaveRecord")]
    pub const SaveRecord: Self = Self(0);
    #[doc(alias = "CKSyncEnginePendingRecordZoneChangeTypeDeleteRecord")]
    pub const DeleteRecord: Self = Self(1);
}

unsafe impl Encode for CKSyncEnginePendingRecordZoneChangeType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKSyncEnginePendingRecordZoneChangeType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEnginePendingRecordZoneChange;

    unsafe impl ClassType for CKSyncEnginePendingRecordZoneChange {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEnginePendingRecordZoneChange {}

unsafe impl Sync for CKSyncEnginePendingRecordZoneChange {}

unsafe impl NSObjectProtocol for CKSyncEnginePendingRecordZoneChange {}

extern_methods!(
    unsafe impl CKSyncEnginePendingRecordZoneChange {
        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Init initWithRecordID:type:)]
        pub unsafe fn initWithRecordID_type(
            this: Allocated<Self>,
            record_id: &CKRecordID,
            r#type: CKSyncEnginePendingRecordZoneChangeType,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other recordID)]
        pub unsafe fn recordID(&self) -> Retained<CKRecordID>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> CKSyncEnginePendingRecordZoneChangeType;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKSyncEnginePendingDatabaseChangeType(pub NSInteger);
impl CKSyncEnginePendingDatabaseChangeType {
    #[doc(alias = "CKSyncEnginePendingDatabaseChangeTypeSaveZone")]
    pub const SaveZone: Self = Self(0);
    #[doc(alias = "CKSyncEnginePendingDatabaseChangeTypeDeleteZone")]
    pub const DeleteZone: Self = Self(1);
}

unsafe impl Encode for CKSyncEnginePendingDatabaseChangeType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKSyncEnginePendingDatabaseChangeType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEnginePendingDatabaseChange;

    unsafe impl ClassType for CKSyncEnginePendingDatabaseChange {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEnginePendingDatabaseChange {}

unsafe impl Sync for CKSyncEnginePendingDatabaseChange {}

unsafe impl NSObjectProtocol for CKSyncEnginePendingDatabaseChange {}

extern_methods!(
    unsafe impl CKSyncEnginePendingDatabaseChange {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneID)]
        pub unsafe fn zoneID(&self) -> Retained<CKRecordZoneID>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> CKSyncEnginePendingDatabaseChangeType;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEnginePendingZoneSave;

    unsafe impl ClassType for CKSyncEnginePendingZoneSave {
        #[inherits(NSObject)]
        type Super = CKSyncEnginePendingDatabaseChange;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEnginePendingZoneSave {}

unsafe impl Sync for CKSyncEnginePendingZoneSave {}

unsafe impl NSObjectProtocol for CKSyncEnginePendingZoneSave {}

extern_methods!(
    unsafe impl CKSyncEnginePendingZoneSave {
        #[cfg(feature = "CKRecordZone")]
        #[method_id(@__retain_semantics Init initWithZone:)]
        pub unsafe fn initWithZone(this: Allocated<Self>, zone: &CKRecordZone) -> Retained<Self>;

        #[cfg(feature = "CKRecordZone")]
        #[method_id(@__retain_semantics Other zone)]
        pub unsafe fn zone(&self) -> Retained<CKRecordZone>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEnginePendingDatabaseChange`
    unsafe impl CKSyncEnginePendingZoneSave {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEnginePendingZoneDelete;

    unsafe impl ClassType for CKSyncEnginePendingZoneDelete {
        #[inherits(NSObject)]
        type Super = CKSyncEnginePendingDatabaseChange;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEnginePendingZoneDelete {}

unsafe impl Sync for CKSyncEnginePendingZoneDelete {}

unsafe impl NSObjectProtocol for CKSyncEnginePendingZoneDelete {}

extern_methods!(
    unsafe impl CKSyncEnginePendingZoneDelete {
        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithZoneID:)]
        pub unsafe fn initWithZoneID(
            this: Allocated<Self>,
            zone_id: &CKRecordZoneID,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEnginePendingDatabaseChange`
    unsafe impl CKSyncEnginePendingZoneDelete {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
