//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait NSWindowRestoration: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(all(
            feature = "NSResponder",
            feature = "NSUserInterfaceItemIdentification",
            feature = "NSWindow",
            feature = "block2"
        ))]
        #[method(restoreWindowWithIdentifier:state:completionHandler:)]
        unsafe fn restoreWindowWithIdentifier_state_completionHandler(
            identifier: &NSUserInterfaceItemIdentifier,
            state: &NSCoder,
            completion_handler: &block2::Block<dyn Fn(*mut NSWindow, *mut NSError)>,
            mtm: MainThreadMarker,
        );
    }

    unsafe impl ProtocolType for dyn NSWindowRestoration {}
);

extern_methods!(
    /// NSWindowRestoration
    #[cfg(feature = "NSDocumentController")]
    unsafe impl NSDocumentController {}
);

#[cfg(feature = "NSDocumentController")]
unsafe impl NSWindowRestoration for NSDocumentController {}

extern_methods!(
    /// NSWindowRestoration
    #[cfg(all(feature = "NSApplication", feature = "NSResponder"))]
    unsafe impl NSApplication {
        #[cfg(all(
            feature = "NSUserInterfaceItemIdentification",
            feature = "NSWindow",
            feature = "block2"
        ))]
        #[method(restoreWindowWithIdentifier:state:completionHandler:)]
        pub unsafe fn restoreWindowWithIdentifier_state_completionHandler(
            &self,
            identifier: &NSUserInterfaceItemIdentifier,
            state: &NSCoder,
            completion_handler: &block2::Block<dyn Fn(*mut NSWindow, *mut NSError)>,
        ) -> bool;
    }
);

extern "C" {
    pub static NSApplicationDidFinishRestoringWindowsNotification: &'static NSNotificationName;
}

extern_methods!(
    /// NSUserInterfaceRestoration
    #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSWindow {
        #[method(isRestorable)]
        pub unsafe fn isRestorable(&self) -> bool;

        #[method(setRestorable:)]
        pub unsafe fn setRestorable(&self, restorable: bool);

        #[method(restorationClass)]
        pub unsafe fn restorationClass(&self) -> Option<&'static AnyClass>;

        #[method(setRestorationClass:)]
        pub unsafe fn setRestorationClass(&self, restoration_class: Option<&AnyClass>);

        #[method(disableSnapshotRestoration)]
        pub unsafe fn disableSnapshotRestoration(&self);

        #[method(enableSnapshotRestoration)]
        pub unsafe fn enableSnapshotRestoration(&self);
    }
);

extern_methods!(
    /// NSRestorableState
    #[cfg(feature = "NSResponder")]
    unsafe impl NSResponder {
        #[method(encodeRestorableStateWithCoder:)]
        pub unsafe fn encodeRestorableStateWithCoder(&self, coder: &NSCoder);

        #[method(encodeRestorableStateWithCoder:backgroundQueue:)]
        pub unsafe fn encodeRestorableStateWithCoder_backgroundQueue(
            &self,
            coder: &NSCoder,
            queue: &NSOperationQueue,
        );

        #[method(restoreStateWithCoder:)]
        pub unsafe fn restoreStateWithCoder(&self, coder: &NSCoder);

        #[method(invalidateRestorableState)]
        pub unsafe fn invalidateRestorableState(&self);

        #[method_id(@__retain_semantics Other restorableStateKeyPaths)]
        pub unsafe fn restorableStateKeyPaths(mtm: MainThreadMarker)
            -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other allowedClassesForRestorableStateKeyPath:)]
        pub unsafe fn allowedClassesForRestorableStateKeyPath(
            key_path: &NSString,
            mtm: MainThreadMarker,
        ) -> Retained<NSArray<TodoClass>>;
    }
);

extern_methods!(
    /// NSRestorableStateExtension
    #[cfg(all(feature = "NSApplication", feature = "NSResponder"))]
    unsafe impl NSApplication {
        #[method(extendStateRestoration)]
        pub unsafe fn extendStateRestoration(&self);

        #[method(completeStateRestoration)]
        pub unsafe fn completeStateRestoration(&self);
    }
);

extern_methods!(
    /// NSRestorableState
    #[cfg(feature = "NSDocument")]
    unsafe impl NSDocument {
        #[cfg(all(
            feature = "NSResponder",
            feature = "NSUserInterfaceItemIdentification",
            feature = "NSWindow",
            feature = "block2"
        ))]
        #[method(restoreDocumentWindowWithIdentifier:state:completionHandler:)]
        pub unsafe fn restoreDocumentWindowWithIdentifier_state_completionHandler(
            &self,
            identifier: &NSUserInterfaceItemIdentifier,
            state: &NSCoder,
            completion_handler: &block2::Block<dyn Fn(*mut NSWindow, *mut NSError)>,
        );

        #[method(encodeRestorableStateWithCoder:)]
        pub unsafe fn encodeRestorableStateWithCoder(&self, coder: &NSCoder);

        #[method(encodeRestorableStateWithCoder:backgroundQueue:)]
        pub unsafe fn encodeRestorableStateWithCoder_backgroundQueue(
            &self,
            coder: &NSCoder,
            queue: &NSOperationQueue,
        );

        #[method(restoreStateWithCoder:)]
        pub unsafe fn restoreStateWithCoder(&self, coder: &NSCoder);

        #[method(invalidateRestorableState)]
        pub unsafe fn invalidateRestorableState(&self);

        #[method_id(@__retain_semantics Other restorableStateKeyPaths)]
        pub unsafe fn restorableStateKeyPaths(mtm: MainThreadMarker)
            -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other allowedClassesForRestorableStateKeyPath:)]
        pub unsafe fn allowedClassesForRestorableStateKeyPath(
            key_path: &NSString,
            mtm: MainThreadMarker,
        ) -> Retained<NSArray<TodoClass>>;
    }
);