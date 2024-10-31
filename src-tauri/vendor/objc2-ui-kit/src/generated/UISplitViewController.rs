//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISplitViewControllerDisplayMode(pub NSInteger);
impl UISplitViewControllerDisplayMode {
    #[doc(alias = "UISplitViewControllerDisplayModeAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UISplitViewControllerDisplayModeSecondaryOnly")]
    pub const SecondaryOnly: Self = Self(1);
    #[doc(alias = "UISplitViewControllerDisplayModeOneBesideSecondary")]
    pub const OneBesideSecondary: Self = Self(2);
    #[doc(alias = "UISplitViewControllerDisplayModeOneOverSecondary")]
    pub const OneOverSecondary: Self = Self(3);
    #[doc(alias = "UISplitViewControllerDisplayModeTwoBesideSecondary")]
    pub const TwoBesideSecondary: Self = Self(4);
    #[doc(alias = "UISplitViewControllerDisplayModeTwoOverSecondary")]
    pub const TwoOverSecondary: Self = Self(5);
    #[doc(alias = "UISplitViewControllerDisplayModeTwoDisplaceSecondary")]
    pub const TwoDisplaceSecondary: Self = Self(6);
    #[deprecated]
    #[doc(alias = "UISplitViewControllerDisplayModePrimaryHidden")]
    pub const PrimaryHidden: Self = Self(UISplitViewControllerDisplayMode::SecondaryOnly.0);
    #[deprecated]
    #[doc(alias = "UISplitViewControllerDisplayModeAllVisible")]
    pub const AllVisible: Self = Self(UISplitViewControllerDisplayMode::OneBesideSecondary.0);
    #[deprecated]
    #[doc(alias = "UISplitViewControllerDisplayModePrimaryOverlay")]
    pub const PrimaryOverlay: Self = Self(UISplitViewControllerDisplayMode::OneOverSecondary.0);
}

unsafe impl Encode for UISplitViewControllerDisplayMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISplitViewControllerDisplayMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISplitViewControllerPrimaryEdge(pub NSInteger);
impl UISplitViewControllerPrimaryEdge {
    #[doc(alias = "UISplitViewControllerPrimaryEdgeLeading")]
    pub const Leading: Self = Self(0);
    #[doc(alias = "UISplitViewControllerPrimaryEdgeTrailing")]
    pub const Trailing: Self = Self(1);
}

unsafe impl Encode for UISplitViewControllerPrimaryEdge {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISplitViewControllerPrimaryEdge {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISplitViewControllerBackgroundStyle(pub NSInteger);
impl UISplitViewControllerBackgroundStyle {
    #[doc(alias = "UISplitViewControllerBackgroundStyleNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UISplitViewControllerBackgroundStyleSidebar")]
    pub const Sidebar: Self = Self(1);
}

unsafe impl Encode for UISplitViewControllerBackgroundStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISplitViewControllerBackgroundStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISplitViewControllerStyle(pub NSInteger);
impl UISplitViewControllerStyle {
    #[deprecated]
    #[doc(alias = "UISplitViewControllerStyleUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "UISplitViewControllerStyleDoubleColumn")]
    pub const DoubleColumn: Self = Self(1);
    #[doc(alias = "UISplitViewControllerStyleTripleColumn")]
    pub const TripleColumn: Self = Self(2);
}

unsafe impl Encode for UISplitViewControllerStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISplitViewControllerStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISplitViewControllerColumn(pub NSInteger);
impl UISplitViewControllerColumn {
    #[doc(alias = "UISplitViewControllerColumnPrimary")]
    pub const Primary: Self = Self(0);
    #[doc(alias = "UISplitViewControllerColumnSupplementary")]
    pub const Supplementary: Self = Self(1);
    #[doc(alias = "UISplitViewControllerColumnSecondary")]
    pub const Secondary: Self = Self(2);
    #[doc(alias = "UISplitViewControllerColumnCompact")]
    pub const Compact: Self = Self(3);
}

unsafe impl Encode for UISplitViewControllerColumn {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISplitViewControllerColumn {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISplitViewControllerSplitBehavior(pub NSInteger);
impl UISplitViewControllerSplitBehavior {
    #[doc(alias = "UISplitViewControllerSplitBehaviorAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UISplitViewControllerSplitBehaviorTile")]
    pub const Tile: Self = Self(1);
    #[doc(alias = "UISplitViewControllerSplitBehaviorOverlay")]
    pub const Overlay: Self = Self(2);
    #[doc(alias = "UISplitViewControllerSplitBehaviorDisplace")]
    pub const Displace: Self = Self(3);
}

unsafe impl Encode for UISplitViewControllerSplitBehavior {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISplitViewControllerSplitBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISplitViewControllerDisplayModeButtonVisibility(pub NSInteger);
impl UISplitViewControllerDisplayModeButtonVisibility {
    #[doc(alias = "UISplitViewControllerDisplayModeButtonVisibilityAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UISplitViewControllerDisplayModeButtonVisibilityNever")]
    pub const Never: Self = Self(1);
    #[doc(alias = "UISplitViewControllerDisplayModeButtonVisibilityAlways")]
    pub const Always: Self = Self(2);
}

unsafe impl Encode for UISplitViewControllerDisplayModeButtonVisibility {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISplitViewControllerDisplayModeButtonVisibility {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static UISplitViewControllerAutomaticDimension: CGFloat;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    pub struct UISplitViewController;

    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl ClassType for UISplitViewController {
        #[inherits(UIResponder, NSObject)]
        type Super = UIViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSCoding for UISplitViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSObjectProtocol for UISplitViewController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UISplitViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIContentContainer for UISplitViewController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UISplitViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIResponderStandardEditActions for UISplitViewController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UISplitViewController {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UISplitViewController {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithStyle:)]
        pub unsafe fn initWithStyle(
            this: Allocated<Self>,
            style: UISplitViewControllerStyle,
        ) -> Retained<Self>;

        #[method(style)]
        pub unsafe fn style(&self) -> UISplitViewControllerStyle;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UISplitViewControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UISplitViewControllerDelegate>>,
        );

        #[method(showsSecondaryOnlyButton)]
        pub unsafe fn showsSecondaryOnlyButton(&self) -> bool;

        #[method(setShowsSecondaryOnlyButton:)]
        pub unsafe fn setShowsSecondaryOnlyButton(&self, shows_secondary_only_button: bool);

        #[method(preferredSplitBehavior)]
        pub unsafe fn preferredSplitBehavior(&self) -> UISplitViewControllerSplitBehavior;

        #[method(setPreferredSplitBehavior:)]
        pub unsafe fn setPreferredSplitBehavior(
            &self,
            preferred_split_behavior: UISplitViewControllerSplitBehavior,
        );

        #[method(splitBehavior)]
        pub unsafe fn splitBehavior(&self) -> UISplitViewControllerSplitBehavior;

        #[method(setViewController:forColumn:)]
        pub unsafe fn setViewController_forColumn(
            &self,
            vc: Option<&UIViewController>,
            column: UISplitViewControllerColumn,
        );

        #[method_id(@__retain_semantics Other viewControllerForColumn:)]
        pub unsafe fn viewControllerForColumn(
            &self,
            column: UISplitViewControllerColumn,
        ) -> Option<Retained<UIViewController>>;

        #[method(hideColumn:)]
        pub unsafe fn hideColumn(&self, column: UISplitViewControllerColumn);

        #[method(showColumn:)]
        pub unsafe fn showColumn(&self, column: UISplitViewControllerColumn);

        #[method_id(@__retain_semantics Other viewControllers)]
        pub unsafe fn viewControllers(&self) -> Retained<NSArray<UIViewController>>;

        #[method(setViewControllers:)]
        pub unsafe fn setViewControllers(&self, view_controllers: &NSArray<UIViewController>);

        #[method(presentsWithGesture)]
        pub unsafe fn presentsWithGesture(&self) -> bool;

        #[method(setPresentsWithGesture:)]
        pub unsafe fn setPresentsWithGesture(&self, presents_with_gesture: bool);

        #[method(isCollapsed)]
        pub unsafe fn isCollapsed(&self) -> bool;

        #[method(preferredDisplayMode)]
        pub unsafe fn preferredDisplayMode(&self) -> UISplitViewControllerDisplayMode;

        #[method(setPreferredDisplayMode:)]
        pub unsafe fn setPreferredDisplayMode(
            &self,
            preferred_display_mode: UISplitViewControllerDisplayMode,
        );

        #[method(displayMode)]
        pub unsafe fn displayMode(&self) -> UISplitViewControllerDisplayMode;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method_id(@__retain_semantics Other displayModeButtonItem)]
        pub unsafe fn displayModeButtonItem(&self) -> Retained<UIBarButtonItem>;

        #[method(displayModeButtonVisibility)]
        pub unsafe fn displayModeButtonVisibility(
            &self,
        ) -> UISplitViewControllerDisplayModeButtonVisibility;

        #[method(setDisplayModeButtonVisibility:)]
        pub unsafe fn setDisplayModeButtonVisibility(
            &self,
            display_mode_button_visibility: UISplitViewControllerDisplayModeButtonVisibility,
        );

        #[method(preferredPrimaryColumnWidthFraction)]
        pub unsafe fn preferredPrimaryColumnWidthFraction(&self) -> CGFloat;

        #[method(setPreferredPrimaryColumnWidthFraction:)]
        pub unsafe fn setPreferredPrimaryColumnWidthFraction(
            &self,
            preferred_primary_column_width_fraction: CGFloat,
        );

        #[method(preferredPrimaryColumnWidth)]
        pub unsafe fn preferredPrimaryColumnWidth(&self) -> CGFloat;

        #[method(setPreferredPrimaryColumnWidth:)]
        pub unsafe fn setPreferredPrimaryColumnWidth(
            &self,
            preferred_primary_column_width: CGFloat,
        );

        #[method(minimumPrimaryColumnWidth)]
        pub unsafe fn minimumPrimaryColumnWidth(&self) -> CGFloat;

        #[method(setMinimumPrimaryColumnWidth:)]
        pub unsafe fn setMinimumPrimaryColumnWidth(&self, minimum_primary_column_width: CGFloat);

        #[method(maximumPrimaryColumnWidth)]
        pub unsafe fn maximumPrimaryColumnWidth(&self) -> CGFloat;

        #[method(setMaximumPrimaryColumnWidth:)]
        pub unsafe fn setMaximumPrimaryColumnWidth(&self, maximum_primary_column_width: CGFloat);

        #[method(primaryColumnWidth)]
        pub unsafe fn primaryColumnWidth(&self) -> CGFloat;

        #[method(preferredSupplementaryColumnWidthFraction)]
        pub unsafe fn preferredSupplementaryColumnWidthFraction(&self) -> CGFloat;

        #[method(setPreferredSupplementaryColumnWidthFraction:)]
        pub unsafe fn setPreferredSupplementaryColumnWidthFraction(
            &self,
            preferred_supplementary_column_width_fraction: CGFloat,
        );

        #[method(preferredSupplementaryColumnWidth)]
        pub unsafe fn preferredSupplementaryColumnWidth(&self) -> CGFloat;

        #[method(setPreferredSupplementaryColumnWidth:)]
        pub unsafe fn setPreferredSupplementaryColumnWidth(
            &self,
            preferred_supplementary_column_width: CGFloat,
        );

        #[method(minimumSupplementaryColumnWidth)]
        pub unsafe fn minimumSupplementaryColumnWidth(&self) -> CGFloat;

        #[method(setMinimumSupplementaryColumnWidth:)]
        pub unsafe fn setMinimumSupplementaryColumnWidth(
            &self,
            minimum_supplementary_column_width: CGFloat,
        );

        #[method(maximumSupplementaryColumnWidth)]
        pub unsafe fn maximumSupplementaryColumnWidth(&self) -> CGFloat;

        #[method(setMaximumSupplementaryColumnWidth:)]
        pub unsafe fn setMaximumSupplementaryColumnWidth(
            &self,
            maximum_supplementary_column_width: CGFloat,
        );

        #[method(supplementaryColumnWidth)]
        pub unsafe fn supplementaryColumnWidth(&self) -> CGFloat;

        #[method(primaryEdge)]
        pub unsafe fn primaryEdge(&self) -> UISplitViewControllerPrimaryEdge;

        #[method(setPrimaryEdge:)]
        pub unsafe fn setPrimaryEdge(&self, primary_edge: UISplitViewControllerPrimaryEdge);

        #[method(showViewController:sender:)]
        pub unsafe fn showViewController_sender(
            &self,
            vc: &UIViewController,
            sender: Option<&AnyObject>,
        );

        #[method(showDetailViewController:sender:)]
        pub unsafe fn showDetailViewController_sender(
            &self,
            vc: &UIViewController,
            sender: Option<&AnyObject>,
        );

        #[method(primaryBackgroundStyle)]
        pub unsafe fn primaryBackgroundStyle(&self) -> UISplitViewControllerBackgroundStyle;

        #[method(setPrimaryBackgroundStyle:)]
        pub unsafe fn setPrimaryBackgroundStyle(
            &self,
            primary_background_style: UISplitViewControllerBackgroundStyle,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UISplitViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait UISplitViewControllerDelegate: IsMainThreadOnly {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(splitViewController:willChangeToDisplayMode:)]
        unsafe fn splitViewController_willChangeToDisplayMode(
            &self,
            svc: &UISplitViewController,
            display_mode: UISplitViewControllerDisplayMode,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(targetDisplayModeForActionInSplitViewController:)]
        unsafe fn targetDisplayModeForActionInSplitViewController(
            &self,
            svc: &UISplitViewController,
        ) -> UISplitViewControllerDisplayMode;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(splitViewController:showViewController:sender:)]
        unsafe fn splitViewController_showViewController_sender(
            &self,
            split_view_controller: &UISplitViewController,
            vc: &UIViewController,
            sender: Option<&AnyObject>,
        ) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(splitViewController:showDetailViewController:sender:)]
        unsafe fn splitViewController_showDetailViewController_sender(
            &self,
            split_view_controller: &UISplitViewController,
            vc: &UIViewController,
            sender: Option<&AnyObject>,
        ) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method_id(@__retain_semantics Other primaryViewControllerForCollapsingSplitViewController:)]
        unsafe fn primaryViewControllerForCollapsingSplitViewController(
            &self,
            split_view_controller: &UISplitViewController,
        ) -> Option<Retained<UIViewController>>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method_id(@__retain_semantics Other primaryViewControllerForExpandingSplitViewController:)]
        unsafe fn primaryViewControllerForExpandingSplitViewController(
            &self,
            split_view_controller: &UISplitViewController,
        ) -> Option<Retained<UIViewController>>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(splitViewController:collapseSecondaryViewController:ontoPrimaryViewController:)]
        unsafe fn splitViewController_collapseSecondaryViewController_ontoPrimaryViewController(
            &self,
            split_view_controller: &UISplitViewController,
            secondary_view_controller: &UIViewController,
            primary_view_controller: &UIViewController,
        ) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method_id(@__retain_semantics Other splitViewController:separateSecondaryViewControllerFromPrimaryViewController:)]
        unsafe fn splitViewController_separateSecondaryViewControllerFromPrimaryViewController(
            &self,
            split_view_controller: &UISplitViewController,
            primary_view_controller: &UIViewController,
        ) -> Option<Retained<UIViewController>>;

        #[cfg(all(
            feature = "UIOrientation",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(splitViewControllerSupportedInterfaceOrientations:)]
        unsafe fn splitViewControllerSupportedInterfaceOrientations(
            &self,
            split_view_controller: &UISplitViewController,
        ) -> UIInterfaceOrientationMask;

        #[cfg(all(
            feature = "UIOrientation",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(splitViewControllerPreferredInterfaceOrientationForPresentation:)]
        unsafe fn splitViewControllerPreferredInterfaceOrientationForPresentation(
            &self,
            split_view_controller: &UISplitViewController,
        ) -> UIInterfaceOrientation;

        #[cfg(all(
            feature = "UIBarButtonItem",
            feature = "UIBarItem",
            feature = "UIPopoverController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[deprecated = "Use splitViewController:willChangeToDisplayMode: and displayModeButtonItem instead"]
        #[optional]
        #[method(splitViewController:willHideViewController:withBarButtonItem:forPopoverController:)]
        unsafe fn splitViewController_willHideViewController_withBarButtonItem_forPopoverController(
            &self,
            svc: &UISplitViewController,
            a_view_controller: &UIViewController,
            bar_button_item: &UIBarButtonItem,
            pc: &UIPopoverController,
        );

        #[cfg(all(
            feature = "UIBarButtonItem",
            feature = "UIBarItem",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[deprecated = "Use splitViewController:willChangeToDisplayMode: and displayModeButtonItem instead"]
        #[optional]
        #[method(splitViewController:willShowViewController:invalidatingBarButtonItem:)]
        unsafe fn splitViewController_willShowViewController_invalidatingBarButtonItem(
            &self,
            svc: &UISplitViewController,
            a_view_controller: &UIViewController,
            bar_button_item: &UIBarButtonItem,
        );

        #[cfg(all(
            feature = "UIPopoverController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[deprecated]
        #[optional]
        #[method(splitViewController:popoverController:willPresentViewController:)]
        unsafe fn splitViewController_popoverController_willPresentViewController(
            &self,
            svc: &UISplitViewController,
            pc: &UIPopoverController,
            a_view_controller: &UIViewController,
        );

        #[cfg(all(
            feature = "UIOrientation",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[deprecated]
        #[optional]
        #[method(splitViewController:shouldHideViewController:inOrientation:)]
        unsafe fn splitViewController_shouldHideViewController_inOrientation(
            &self,
            svc: &UISplitViewController,
            vc: &UIViewController,
            orientation: UIInterfaceOrientation,
        ) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(splitViewController:topColumnForCollapsingToProposedTopColumn:)]
        unsafe fn splitViewController_topColumnForCollapsingToProposedTopColumn(
            &self,
            svc: &UISplitViewController,
            proposed_top_column: UISplitViewControllerColumn,
        ) -> UISplitViewControllerColumn;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(splitViewController:displayModeForExpandingToProposedDisplayMode:)]
        unsafe fn splitViewController_displayModeForExpandingToProposedDisplayMode(
            &self,
            svc: &UISplitViewController,
            proposed_display_mode: UISplitViewControllerDisplayMode,
        ) -> UISplitViewControllerDisplayMode;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(splitViewControllerDidCollapse:)]
        unsafe fn splitViewControllerDidCollapse(&self, svc: &UISplitViewController);

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(splitViewControllerDidExpand:)]
        unsafe fn splitViewControllerDidExpand(&self, svc: &UISplitViewController);

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(splitViewController:willShowColumn:)]
        unsafe fn splitViewController_willShowColumn(
            &self,
            svc: &UISplitViewController,
            column: UISplitViewControllerColumn,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(splitViewController:willHideColumn:)]
        unsafe fn splitViewController_willHideColumn(
            &self,
            svc: &UISplitViewController,
            column: UISplitViewControllerColumn,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(splitViewControllerInteractivePresentationGestureWillBegin:)]
        unsafe fn splitViewControllerInteractivePresentationGestureWillBegin(
            &self,
            svc: &UISplitViewController,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(splitViewControllerInteractivePresentationGestureDidEnd:)]
        unsafe fn splitViewControllerInteractivePresentationGestureDidEnd(
            &self,
            svc: &UISplitViewController,
        );
    }

    unsafe impl ProtocolType for dyn UISplitViewControllerDelegate {}
);

extern_methods!(
    /// UISplitViewController
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIViewController {
        #[method_id(@__retain_semantics Other splitViewController)]
        pub unsafe fn splitViewController(&self) -> Option<Retained<UISplitViewController>>;

        #[method(collapseSecondaryViewController:forSplitViewController:)]
        pub unsafe fn collapseSecondaryViewController_forSplitViewController(
            &self,
            secondary_view_controller: &UIViewController,
            split_view_controller: &UISplitViewController,
        );

        #[method_id(@__retain_semantics Other separateSecondaryViewControllerForSplitViewController:)]
        pub unsafe fn separateSecondaryViewControllerForSplitViewController(
            &self,
            split_view_controller: &UISplitViewController,
        ) -> Option<Retained<UIViewController>>;
    }
);