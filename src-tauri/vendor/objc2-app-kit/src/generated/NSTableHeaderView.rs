//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub struct NSTableHeaderView;

    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl ClassType for NSTableHeaderView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSTableHeaderView {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSTableHeaderView {}

#[cfg(all(feature = "NSAnimation", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAnimatablePropertyContainer for NSTableHeaderView {}

#[cfg(all(feature = "NSAppearance", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAppearanceCustomization for NSTableHeaderView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSTableHeaderView {}

#[cfg(all(feature = "NSDragging", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSDraggingDestination for NSTableHeaderView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSTableHeaderView {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSTableHeaderView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSViewToolTipOwner for NSTableHeaderView {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTableHeaderView {
        #[cfg(all(feature = "NSControl", feature = "NSTableView"))]
        #[method_id(@__retain_semantics Other tableView)]
        pub unsafe fn tableView(&self) -> Option<Retained<NSTableView>>;

        #[cfg(all(feature = "NSControl", feature = "NSTableView"))]
        #[method(setTableView:)]
        pub unsafe fn setTableView(&self, table_view: Option<&NSTableView>);

        #[method(draggedColumn)]
        pub unsafe fn draggedColumn(&self) -> NSInteger;

        #[method(draggedDistance)]
        pub unsafe fn draggedDistance(&self) -> CGFloat;

        #[method(resizedColumn)]
        pub unsafe fn resizedColumn(&self) -> NSInteger;

        #[method(headerRectOfColumn:)]
        pub unsafe fn headerRectOfColumn(&self, column: NSInteger) -> NSRect;

        #[method(columnAtPoint:)]
        pub unsafe fn columnAtPoint(&self, point: NSPoint) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTableHeaderView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTableHeaderView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTableHeaderView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);