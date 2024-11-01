//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait NSTextAttachmentCellProtocol: NSObjectProtocol {
        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(drawWithFrame:inView:)]
        unsafe fn drawWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: Option<&NSView>,
            mtm: MainThreadMarker,
        );

        #[method(wantsToTrackMouse)]
        unsafe fn wantsToTrackMouse(&self, mtm: MainThreadMarker) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(highlight:withFrame:inView:)]
        unsafe fn highlight_withFrame_inView(
            &self,
            flag: bool,
            cell_frame: NSRect,
            control_view: Option<&NSView>,
            mtm: MainThreadMarker,
        );

        #[cfg(all(feature = "NSEvent", feature = "NSResponder", feature = "NSView"))]
        #[method(trackMouse:inRect:ofView:untilMouseUp:)]
        unsafe fn trackMouse_inRect_ofView_untilMouseUp(
            &self,
            the_event: &NSEvent,
            cell_frame: NSRect,
            control_view: Option<&NSView>,
            flag: bool,
            mtm: MainThreadMarker,
        ) -> bool;

        #[method(cellSize)]
        unsafe fn cellSize(&self) -> NSSize;

        #[method(cellBaselineOffset)]
        unsafe fn cellBaselineOffset(&self) -> NSPoint;

        #[cfg(feature = "NSTextAttachment")]
        #[method_id(@__retain_semantics Other attachment)]
        unsafe fn attachment(&self) -> Option<Retained<NSTextAttachment>>;

        #[cfg(feature = "NSTextAttachment")]
        #[method(setAttachment:)]
        unsafe fn setAttachment(&self, attachment: Option<&NSTextAttachment>);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(drawWithFrame:inView:characterIndex:)]
        unsafe fn drawWithFrame_inView_characterIndex(
            &self,
            cell_frame: NSRect,
            control_view: Option<&NSView>,
            char_index: NSUInteger,
            mtm: MainThreadMarker,
        );

        #[cfg(all(
            feature = "NSLayoutManager",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[method(drawWithFrame:inView:characterIndex:layoutManager:)]
        unsafe fn drawWithFrame_inView_characterIndex_layoutManager(
            &self,
            cell_frame: NSRect,
            control_view: Option<&NSView>,
            char_index: NSUInteger,
            layout_manager: &NSLayoutManager,
            mtm: MainThreadMarker,
        );

        #[cfg(all(feature = "NSEvent", feature = "NSResponder", feature = "NSView"))]
        #[method(wantsToTrackMouseForEvent:inRect:ofView:atCharacterIndex:)]
        unsafe fn wantsToTrackMouseForEvent_inRect_ofView_atCharacterIndex(
            &self,
            the_event: &NSEvent,
            cell_frame: NSRect,
            control_view: Option<&NSView>,
            char_index: NSUInteger,
            mtm: MainThreadMarker,
        ) -> bool;

        #[cfg(all(feature = "NSEvent", feature = "NSResponder", feature = "NSView"))]
        #[method(trackMouse:inRect:ofView:atCharacterIndex:untilMouseUp:)]
        unsafe fn trackMouse_inRect_ofView_atCharacterIndex_untilMouseUp(
            &self,
            the_event: &NSEvent,
            cell_frame: NSRect,
            control_view: Option<&NSView>,
            char_index: NSUInteger,
            flag: bool,
            mtm: MainThreadMarker,
        ) -> bool;

        #[cfg(feature = "NSTextContainer")]
        #[method(cellFrameForTextContainer:proposedLineFragment:glyphPosition:characterIndex:)]
        unsafe fn cellFrameForTextContainer_proposedLineFragment_glyphPosition_characterIndex(
            &self,
            text_container: &NSTextContainer,
            line_frag: NSRect,
            position: NSPoint,
            char_index: NSUInteger,
        ) -> NSRect;
    }

    unsafe impl ProtocolType for dyn NSTextAttachmentCellProtocol {
        const NAME: &'static str = "NSTextAttachmentCell";
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSCell")]
    pub struct NSTextAttachmentCell;

    #[cfg(feature = "NSCell")]
    unsafe impl ClassType for NSTextAttachmentCell {
        #[inherits(NSObject)]
        type Super = NSCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(feature = "NSAccessibilityProtocols", feature = "NSCell"))]
unsafe impl NSAccessibility for NSTextAttachmentCell {}

#[cfg(all(feature = "NSAccessibilityProtocols", feature = "NSCell"))]
unsafe impl NSAccessibilityElementProtocol for NSTextAttachmentCell {}

#[cfg(feature = "NSCell")]
unsafe impl NSCoding for NSTextAttachmentCell {}

#[cfg(feature = "NSCell")]
unsafe impl NSCopying for NSTextAttachmentCell {}

#[cfg(feature = "NSCell")]
unsafe impl NSObjectProtocol for NSTextAttachmentCell {}

#[cfg(feature = "NSCell")]
unsafe impl NSTextAttachmentCellProtocol for NSTextAttachmentCell {}

#[cfg(all(feature = "NSCell", feature = "NSUserInterfaceItemIdentification"))]
unsafe impl NSUserInterfaceItemIdentification for NSTextAttachmentCell {}

extern_methods!(
    #[cfg(feature = "NSCell")]
    unsafe impl NSTextAttachmentCell {}
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "NSCell")]
    unsafe impl NSTextAttachmentCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Allocated<Self>, string: &NSString) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Allocated<Self>,
            image: Option<&NSImage>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSCell")]
    unsafe impl NSTextAttachmentCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
