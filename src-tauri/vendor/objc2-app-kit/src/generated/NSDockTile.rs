//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(feature = "NSApplication")]
pub static NSAppKitVersionNumberWithDockTilePlugInSupport: NSAppKitVersion = 1001.0 as _;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDockTile;

    unsafe impl ClassType for NSDockTile {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSDockTile {}

extern_methods!(
    unsafe impl NSDockTile {
        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other contentView)]
        pub unsafe fn contentView(&self, mtm: MainThreadMarker) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, content_view: Option<&NSView>);

        #[method(display)]
        pub unsafe fn display(&self);

        #[method(showsApplicationBadge)]
        pub unsafe fn showsApplicationBadge(&self) -> bool;

        #[method(setShowsApplicationBadge:)]
        pub unsafe fn setShowsApplicationBadge(&self, shows_application_badge: bool);

        #[method_id(@__retain_semantics Other badgeLabel)]
        pub unsafe fn badgeLabel(&self) -> Option<Retained<NSString>>;

        #[method(setBadgeLabel:)]
        pub unsafe fn setBadgeLabel(&self, badge_label: Option<&NSString>);

        #[method_id(@__retain_semantics Other owner)]
        pub unsafe fn owner(&self) -> Option<Retained<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSDockTile {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSDockTilePlugIn: NSObjectProtocol {
        #[method(setDockTile:)]
        unsafe fn setDockTile(&self, dock_tile: Option<&NSDockTile>);

        #[cfg(feature = "NSMenu")]
        #[optional]
        #[method_id(@__retain_semantics Other dockMenu)]
        unsafe fn dockMenu(&self, mtm: MainThreadMarker) -> Option<Retained<NSMenu>>;
    }

    unsafe impl ProtocolType for dyn NSDockTilePlugIn {}
);
