//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub type NSStatusItemAutosaveName = NSString;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSStatusItemBehavior(pub NSUInteger);
bitflags::bitflags! {
    impl NSStatusItemBehavior: NSUInteger {
        #[doc(alias = "NSStatusItemBehaviorRemovalAllowed")]
        const RemovalAllowed = 1<<1;
        #[doc(alias = "NSStatusItemBehaviorTerminationOnRemoval")]
        const TerminationOnRemoval = 1<<2;
    }
}

unsafe impl Encode for NSStatusItemBehavior {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSStatusItemBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStatusItem;

    unsafe impl ClassType for NSStatusItem {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSStatusItem {}

extern_methods!(
    unsafe impl NSStatusItem {
        #[cfg(feature = "NSStatusBar")]
        #[method_id(@__retain_semantics Other statusBar)]
        pub unsafe fn statusBar(&self) -> Option<Retained<NSStatusBar>>;

        #[method(length)]
        pub unsafe fn length(&self) -> CGFloat;

        #[method(setLength:)]
        pub unsafe fn setLength(&self, length: CGFloat);

        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self, mtm: MainThreadMarker) -> Option<Retained<NSMenu>>;

        #[cfg(feature = "NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[cfg(all(
            feature = "NSButton",
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSStatusBarButton",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Other button)]
        pub unsafe fn button(&self, mtm: MainThreadMarker) -> Option<Retained<NSStatusBarButton>>;

        #[method(behavior)]
        pub unsafe fn behavior(&self) -> NSStatusItemBehavior;

        #[method(setBehavior:)]
        pub unsafe fn setBehavior(&self, behavior: NSStatusItemBehavior);

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[method(setVisible:)]
        pub unsafe fn setVisible(&self, visible: bool);

        #[method_id(@__retain_semantics Other autosaveName)]
        pub unsafe fn autosaveName(&self) -> Retained<NSStatusItemAutosaveName>;

        #[method(setAutosaveName:)]
        pub unsafe fn setAutosaveName(&self, autosave_name: Option<&NSStatusItemAutosaveName>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSStatusItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSStatusItemDeprecated
    unsafe impl NSStatusItem {
        #[deprecated = "Use the receiver's button.action instead"]
        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[deprecated = "Use the receiver's button.action instead"]
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[deprecated = "Use the receiver's button.doubleAction instead"]
        #[method(doubleAction)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;

        #[deprecated = "Use the receiver's button.doubleAction instead"]
        #[method(setDoubleAction:)]
        pub unsafe fn setDoubleAction(&self, double_action: Option<Sel>);

        #[deprecated = "Use the receiver's button.target instead"]
        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Retained<AnyObject>>;

        #[deprecated = "Use the receiver's button.target instead"]
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[deprecated = "Use the receiver's button.title instead"]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        #[deprecated = "Use the receiver's button.title instead"]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[deprecated = "Use the receiver's button.attributedTitle instead"]
        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Option<Retained<NSAttributedString>>;

        #[deprecated = "Use the receiver's button.attributedTitle instead"]
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: Option<&NSAttributedString>);

        #[cfg(feature = "NSImage")]
        #[deprecated = "Use the receiver's button.image instead"]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[deprecated = "Use the receiver's button.image instead"]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "NSImage")]
        #[deprecated = "Use the receiver's button.alternateImage instead"]
        #[method_id(@__retain_semantics Other alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[deprecated = "Use the receiver's button.alternateImage instead"]
        #[method(setAlternateImage:)]
        pub unsafe fn setAlternateImage(&self, alternate_image: Option<&NSImage>);

        #[deprecated = "Use the receiver's button.enabled instead"]
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[deprecated = "Use the receiver's button.enabled instead"]
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[deprecated = "Use the receiver's button.cell.highlightsBy instead"]
        #[method(highlightMode)]
        pub unsafe fn highlightMode(&self) -> bool;

        #[deprecated = "Use the receiver's button.cell.highlightsBy instead"]
        #[method(setHighlightMode:)]
        pub unsafe fn setHighlightMode(&self, highlight_mode: bool);

        #[deprecated = "Use the receiver's button.toolTip instead"]
        #[method_id(@__retain_semantics Other toolTip)]
        pub unsafe fn toolTip(&self) -> Option<Retained<NSString>>;

        #[deprecated = "Use the receiver's button.toolTip instead"]
        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, tool_tip: Option<&NSString>);

        #[cfg(feature = "NSEvent")]
        #[deprecated = "Use the receiver's button's -sendActionOn: instead"]
        #[method(sendActionOn:)]
        pub unsafe fn sendActionOn(&self, mask: NSEventMask) -> NSInteger;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[deprecated = "Use the standard button property instead"]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self, mtm: MainThreadMarker) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[deprecated = "Use the standard button property instead"]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[deprecated = "Use the standard button instead which handles highlight drawing, making this method obsolete"]
        #[method(drawStatusBarBackgroundInRect:withHighlight:)]
        pub unsafe fn drawStatusBarBackgroundInRect_withHighlight(
            &self,
            rect: NSRect,
            highlight: bool,
        );

        #[cfg(feature = "NSMenu")]
        #[deprecated = "Use the menu property instead"]
        #[method(popUpStatusItemMenu:)]
        pub unsafe fn popUpStatusItemMenu(&self, menu: &NSMenu);
    }
);
