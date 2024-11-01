//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIControlEvents(pub NSUInteger);
bitflags::bitflags! {
    impl UIControlEvents: NSUInteger {
        const UIControlEventTouchDown = 1<<0;
        const UIControlEventTouchDownRepeat = 1<<1;
        const UIControlEventTouchDragInside = 1<<2;
        const UIControlEventTouchDragOutside = 1<<3;
        const UIControlEventTouchDragEnter = 1<<4;
        const UIControlEventTouchDragExit = 1<<5;
        const UIControlEventTouchUpInside = 1<<6;
        const UIControlEventTouchUpOutside = 1<<7;
        const UIControlEventTouchCancel = 1<<8;
        const UIControlEventValueChanged = 1<<12;
        const UIControlEventPrimaryActionTriggered = 1<<13;
        const UIControlEventMenuActionTriggered = 1<<14;
        const UIControlEventEditingDidBegin = 1<<16;
        const UIControlEventEditingChanged = 1<<17;
        const UIControlEventEditingDidEnd = 1<<18;
        const UIControlEventEditingDidEndOnExit = 1<<19;
        const UIControlEventAllTouchEvents = 0x00000FFF;
        const UIControlEventAllEditingEvents = 0x000F0000;
        const UIControlEventApplicationReserved = 0x0F000000;
        const UIControlEventSystemReserved = 0xF0000000;
        const UIControlEventAllEvents = 0xFFFFFFFF;
    }
}

unsafe impl Encode for UIControlEvents {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIControlEvents {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIControlContentVerticalAlignment(pub NSInteger);
impl UIControlContentVerticalAlignment {
    #[doc(alias = "UIControlContentVerticalAlignmentCenter")]
    pub const Center: Self = Self(0);
    #[doc(alias = "UIControlContentVerticalAlignmentTop")]
    pub const Top: Self = Self(1);
    #[doc(alias = "UIControlContentVerticalAlignmentBottom")]
    pub const Bottom: Self = Self(2);
    #[doc(alias = "UIControlContentVerticalAlignmentFill")]
    pub const Fill: Self = Self(3);
}

unsafe impl Encode for UIControlContentVerticalAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIControlContentVerticalAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIControlContentHorizontalAlignment(pub NSInteger);
impl UIControlContentHorizontalAlignment {
    #[doc(alias = "UIControlContentHorizontalAlignmentCenter")]
    pub const Center: Self = Self(0);
    #[doc(alias = "UIControlContentHorizontalAlignmentLeft")]
    pub const Left: Self = Self(1);
    #[doc(alias = "UIControlContentHorizontalAlignmentRight")]
    pub const Right: Self = Self(2);
    #[doc(alias = "UIControlContentHorizontalAlignmentFill")]
    pub const Fill: Self = Self(3);
    #[doc(alias = "UIControlContentHorizontalAlignmentLeading")]
    pub const Leading: Self = Self(4);
    #[doc(alias = "UIControlContentHorizontalAlignmentTrailing")]
    pub const Trailing: Self = Self(5);
}

unsafe impl Encode for UIControlContentHorizontalAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIControlContentHorizontalAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIControlState(pub NSUInteger);
bitflags::bitflags! {
    impl UIControlState: NSUInteger {
        #[doc(alias = "UIControlStateNormal")]
        const Normal = 0;
        #[doc(alias = "UIControlStateHighlighted")]
        const Highlighted = 1<<0;
        #[doc(alias = "UIControlStateDisabled")]
        const Disabled = 1<<1;
        #[doc(alias = "UIControlStateSelected")]
        const Selected = 1<<2;
        #[doc(alias = "UIControlStateFocused")]
        const Focused = 1<<3;
        #[doc(alias = "UIControlStateApplication")]
        const Application = 0x00FF0000;
        #[doc(alias = "UIControlStateReserved")]
        const Reserved = 0xFF000000;
    }
}

unsafe impl Encode for UIControlState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIControlState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UIControl;

    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl ClassType for UIControl {
        #[inherits(UIResponder, NSObject)]
        type Super = UIView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UIControl {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIControl {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIControl {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UIControl {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UIControl {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIControl {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIControl {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UIControl {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UIControl {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UIControl {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIControl {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIControl {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIControl {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Init initWithFrame:primaryAction:)]
        pub unsafe fn initWithFrame_primaryAction(
            this: Allocated<Self>,
            frame: CGRect,
            primary_action: Option<&UIAction>,
        ) -> Retained<Self>;

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(isSelected)]
        pub unsafe fn isSelected(&self) -> bool;

        #[method(setSelected:)]
        pub unsafe fn setSelected(&self, selected: bool);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        #[method(setHighlighted:)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);

        #[method(contentVerticalAlignment)]
        pub unsafe fn contentVerticalAlignment(&self) -> UIControlContentVerticalAlignment;

        #[method(setContentVerticalAlignment:)]
        pub unsafe fn setContentVerticalAlignment(
            &self,
            content_vertical_alignment: UIControlContentVerticalAlignment,
        );

        #[method(contentHorizontalAlignment)]
        pub unsafe fn contentHorizontalAlignment(&self) -> UIControlContentHorizontalAlignment;

        #[method(setContentHorizontalAlignment:)]
        pub unsafe fn setContentHorizontalAlignment(
            &self,
            content_horizontal_alignment: UIControlContentHorizontalAlignment,
        );

        #[method(effectiveContentHorizontalAlignment)]
        pub unsafe fn effectiveContentHorizontalAlignment(
            &self,
        ) -> UIControlContentHorizontalAlignment;

        #[method(state)]
        pub unsafe fn state(&self) -> UIControlState;

        #[method(isTracking)]
        pub unsafe fn isTracking(&self) -> bool;

        #[method(isTouchInside)]
        pub unsafe fn isTouchInside(&self) -> bool;

        #[cfg(all(feature = "UIEvent", feature = "UITouch"))]
        #[method(beginTrackingWithTouch:withEvent:)]
        pub unsafe fn beginTrackingWithTouch_withEvent(
            &self,
            touch: &UITouch,
            event: Option<&UIEvent>,
        ) -> bool;

        #[cfg(all(feature = "UIEvent", feature = "UITouch"))]
        #[method(continueTrackingWithTouch:withEvent:)]
        pub unsafe fn continueTrackingWithTouch_withEvent(
            &self,
            touch: &UITouch,
            event: Option<&UIEvent>,
        ) -> bool;

        #[cfg(all(feature = "UIEvent", feature = "UITouch"))]
        #[method(endTrackingWithTouch:withEvent:)]
        pub unsafe fn endTrackingWithTouch_withEvent(
            &self,
            touch: Option<&UITouch>,
            event: Option<&UIEvent>,
        );

        #[cfg(feature = "UIEvent")]
        #[method(cancelTrackingWithEvent:)]
        pub unsafe fn cancelTrackingWithEvent(&self, event: Option<&UIEvent>);

        #[method(addTarget:action:forControlEvents:)]
        pub unsafe fn addTarget_action_forControlEvents(
            &self,
            target: Option<&AnyObject>,
            action: Sel,
            control_events: UIControlEvents,
        );

        #[method(removeTarget:action:forControlEvents:)]
        pub unsafe fn removeTarget_action_forControlEvents(
            &self,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            control_events: UIControlEvents,
        );

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method(addAction:forControlEvents:)]
        pub unsafe fn addAction_forControlEvents(
            &self,
            action: &UIAction,
            control_events: UIControlEvents,
        );

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method(removeAction:forControlEvents:)]
        pub unsafe fn removeAction_forControlEvents(
            &self,
            action: &UIAction,
            control_events: UIControlEvents,
        );

        #[cfg(feature = "UIAction")]
        #[method(removeActionForIdentifier:forControlEvents:)]
        pub unsafe fn removeActionForIdentifier_forControlEvents(
            &self,
            action_identifier: &UIActionIdentifier,
            control_events: UIControlEvents,
        );

        #[method(performPrimaryAction)]
        pub unsafe fn performPrimaryAction(&self);

        #[method_id(@__retain_semantics Other allTargets)]
        pub unsafe fn allTargets(&self) -> Retained<NSSet>;

        #[method(allControlEvents)]
        pub unsafe fn allControlEvents(&self) -> UIControlEvents;

        #[method_id(@__retain_semantics Other actionsForTarget:forControlEvent:)]
        pub unsafe fn actionsForTarget_forControlEvent(
            &self,
            target: Option<&AnyObject>,
            control_event: UIControlEvents,
        ) -> Option<Retained<NSArray<NSString>>>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement", feature = "block2"))]
        #[method(enumerateEventHandlers:)]
        pub unsafe fn enumerateEventHandlers(
            &self,
            iterator: &block2::Block<
                dyn Fn(*mut UIAction, *mut AnyObject, Option<Sel>, UIControlEvents, NonNull<Bool>)
                    + '_,
            >,
        );

        #[cfg(feature = "UIEvent")]
        #[method(sendAction:to:forEvent:)]
        pub unsafe fn sendAction_to_forEvent(
            &self,
            action: Sel,
            target: Option<&AnyObject>,
            event: Option<&UIEvent>,
        );

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method(sendAction:)]
        pub unsafe fn sendAction(&self, action: &UIAction);

        #[method(sendActionsForControlEvents:)]
        pub unsafe fn sendActionsForControlEvents(&self, control_events: UIControlEvents);

        #[cfg(feature = "UIContextMenuInteraction")]
        #[method_id(@__retain_semantics Other contextMenuInteraction)]
        pub unsafe fn contextMenuInteraction(&self) -> Option<Retained<UIContextMenuInteraction>>;

        #[method(isContextMenuInteractionEnabled)]
        pub unsafe fn isContextMenuInteractionEnabled(&self) -> bool;

        #[method(setContextMenuInteractionEnabled:)]
        pub unsafe fn setContextMenuInteractionEnabled(
            &self,
            context_menu_interaction_enabled: bool,
        );

        #[method(showsMenuAsPrimaryAction)]
        pub unsafe fn showsMenuAsPrimaryAction(&self) -> bool;

        #[method(setShowsMenuAsPrimaryAction:)]
        pub unsafe fn setShowsMenuAsPrimaryAction(&self, shows_menu_as_primary_action: bool);

        #[cfg(feature = "UIContextMenuConfiguration")]
        #[method(menuAttachmentPointForConfiguration:)]
        pub unsafe fn menuAttachmentPointForConfiguration(
            &self,
            configuration: &UIContextMenuConfiguration,
        ) -> CGPoint;

        #[method_id(@__retain_semantics Other toolTip)]
        pub unsafe fn toolTip(&self) -> Option<Retained<NSString>>;

        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, tool_tip: Option<&NSString>);

        #[cfg(feature = "UIToolTipInteraction")]
        #[method_id(@__retain_semantics Other toolTipInteraction)]
        pub unsafe fn toolTipInteraction(&self) -> Option<Retained<UIToolTipInteraction>>;

        #[method(isSymbolAnimationEnabled)]
        pub unsafe fn isSymbolAnimationEnabled(&self) -> bool;

        #[method(setSymbolAnimationEnabled:)]
        pub unsafe fn setSymbolAnimationEnabled(&self, symbol_animation_enabled: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIControl {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIControl {
        #[cfg(all(
            feature = "UIContextMenuConfiguration",
            feature = "UIContextMenuInteraction"
        ))]
        #[method_id(@__retain_semantics Other contextMenuInteraction:configurationForMenuAtLocation:)]
        pub unsafe fn contextMenuInteraction_configurationForMenuAtLocation(
            &self,
            interaction: &UIContextMenuInteraction,
            location: CGPoint,
        ) -> Option<Retained<UIContextMenuConfiguration>>;

        #[cfg(all(
            feature = "UIContextMenuConfiguration",
            feature = "UIContextMenuInteraction",
            feature = "UITargetedPreview"
        ))]
        #[method_id(@__retain_semantics Other contextMenuInteraction:previewForHighlightingMenuWithConfiguration:)]
        pub unsafe fn contextMenuInteraction_previewForHighlightingMenuWithConfiguration(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
        ) -> Option<Retained<UITargetedPreview>>;

        #[cfg(all(
            feature = "UIContextMenuConfiguration",
            feature = "UIContextMenuInteraction",
            feature = "UITargetedPreview"
        ))]
        #[method_id(@__retain_semantics Other contextMenuInteraction:previewForDismissingMenuWithConfiguration:)]
        pub unsafe fn contextMenuInteraction_previewForDismissingMenuWithConfiguration(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
        ) -> Option<Retained<UITargetedPreview>>;

        #[cfg(all(
            feature = "UIContextMenuConfiguration",
            feature = "UIContextMenuInteraction"
        ))]
        #[method(contextMenuInteraction:willDisplayMenuForConfiguration:animator:)]
        pub unsafe fn contextMenuInteraction_willDisplayMenuForConfiguration_animator(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            animator: Option<&ProtocolObject<dyn UIContextMenuInteractionAnimating>>,
        );

        #[cfg(all(
            feature = "UIContextMenuConfiguration",
            feature = "UIContextMenuInteraction"
        ))]
        #[method(contextMenuInteraction:willEndForConfiguration:animator:)]
        pub unsafe fn contextMenuInteraction_willEndForConfiguration_animator(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            animator: Option<&ProtocolObject<dyn UIContextMenuInteractionAnimating>>,
        );

        #[cfg(all(
            feature = "UIContextMenuConfiguration",
            feature = "UIContextMenuInteraction"
        ))]
        #[method(contextMenuInteraction:willPerformPreviewActionForMenuWithConfiguration:animator:)]
        pub unsafe fn contextMenuInteraction_willPerformPreviewActionForMenuWithConfiguration_animator(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            animator: &ProtocolObject<dyn UIContextMenuInteractionCommitAnimating>,
        );
    }
);

#[cfg(all(
    feature = "UIContextMenuInteraction",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIContextMenuInteractionDelegate for UIControl {}