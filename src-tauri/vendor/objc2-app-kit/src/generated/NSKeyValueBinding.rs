//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-data")]
#[cfg(target_vendor = "apple")]
use objc2_core_data::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSBindingName = NSString;

// NS_TYPED_ENUM
pub type NSBindingOption = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSBindingSelectionMarker;

    unsafe impl ClassType for NSBindingSelectionMarker {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for NSBindingSelectionMarker {}

unsafe impl NSObjectProtocol for NSBindingSelectionMarker {}

extern_methods!(
    unsafe impl NSBindingSelectionMarker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other multipleValuesSelectionMarker)]
        pub unsafe fn multipleValuesSelectionMarker() -> Retained<NSBindingSelectionMarker>;

        #[method_id(@__retain_semantics Other noSelectionMarker)]
        pub unsafe fn noSelectionMarker() -> Retained<NSBindingSelectionMarker>;

        #[method_id(@__retain_semantics Other notApplicableSelectionMarker)]
        pub unsafe fn notApplicableSelectionMarker() -> Retained<NSBindingSelectionMarker>;

        #[method(setDefaultPlaceholder:forMarker:onClass:withBinding:)]
        pub unsafe fn setDefaultPlaceholder_forMarker_onClass_withBinding(
            placeholder: Option<&AnyObject>,
            marker: Option<&NSBindingSelectionMarker>,
            object_class: &AnyClass,
            binding: &NSBindingName,
        );

        #[method_id(@__retain_semantics Other defaultPlaceholderForMarker:onClass:withBinding:)]
        pub unsafe fn defaultPlaceholderForMarker_onClass_withBinding(
            marker: Option<&NSBindingSelectionMarker>,
            object_class: &AnyClass,
            binding: &NSBindingName,
        ) -> Option<Retained<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSBindingSelectionMarker {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static NSMultipleValuesMarker: &'static AnyObject;
}

extern "C" {
    pub static NSNoSelectionMarker: &'static AnyObject;
}

extern "C" {
    pub static NSNotApplicableMarker: &'static AnyObject;
}

extern "C" {
    pub fn NSIsControllerMarker(object: Option<&AnyObject>) -> Bool;
}

// NS_TYPED_ENUM
pub type NSBindingInfoKey = NSString;

extern "C" {
    pub static NSObservedObjectKey: &'static NSBindingInfoKey;
}

extern "C" {
    pub static NSObservedKeyPathKey: &'static NSBindingInfoKey;
}

extern "C" {
    pub static NSOptionsKey: &'static NSBindingInfoKey;
}

extern_category!(
    /// Category "NSKeyValueBindingCreation" on [`NSObject`].
    #[doc(alias = "NSKeyValueBindingCreation")]
    pub unsafe trait NSObjectNSKeyValueBindingCreation {
        #[method(exposeBinding:)]
        unsafe fn exposeBinding(binding: &NSBindingName);

        #[method_id(@__retain_semantics Other exposedBindings)]
        unsafe fn exposedBindings(&self) -> Retained<NSArray<NSBindingName>>;

        #[method(valueClassForBinding:)]
        unsafe fn valueClassForBinding(&self, binding: &NSBindingName)
            -> Option<&'static AnyClass>;

        #[method(bind:toObject:withKeyPath:options:)]
        unsafe fn bind_toObject_withKeyPath_options(
            &self,
            binding: &NSBindingName,
            observable: &AnyObject,
            key_path: &NSString,
            options: Option<&NSDictionary<NSBindingOption, AnyObject>>,
        );

        #[method(unbind:)]
        unsafe fn unbind(&self, binding: &NSBindingName);

        #[method_id(@__retain_semantics Other infoForBinding:)]
        unsafe fn infoForBinding(
            &self,
            binding: &NSBindingName,
        ) -> Option<Retained<NSDictionary<NSBindingInfoKey, AnyObject>>>;

        #[cfg(feature = "objc2-core-data")]
        #[cfg(target_vendor = "apple")]
        #[method_id(@__retain_semantics Other optionDescriptionsForBinding:)]
        unsafe fn optionDescriptionsForBinding(
            &self,
            binding: &NSBindingName,
        ) -> Retained<NSArray<NSAttributeDescription>>;
    }

    unsafe impl NSObjectNSKeyValueBindingCreation for NSObject {}
);

extern_protocol!(
    pub unsafe trait NSEditor: NSObjectProtocol + IsMainThreadOnly {
        #[method(discardEditing)]
        unsafe fn discardEditing(&self);

        #[method(commitEditing)]
        unsafe fn commitEditing(&self) -> bool;

        #[method(commitEditingWithDelegate:didCommitSelector:contextInfo:)]
        unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo(
            &self,
            delegate: Option<&AnyObject>,
            did_commit_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[method(commitEditingAndReturnError:_)]
        unsafe fn commitEditingAndReturnError(&self) -> Result<(), Retained<NSError>>;
    }

    unsafe impl ProtocolType for dyn NSEditor {}
);

extern_protocol!(
    pub unsafe trait NSEditorRegistration: NSObjectProtocol + IsMainThreadOnly {
        #[optional]
        #[method(objectDidBeginEditing:)]
        unsafe fn objectDidBeginEditing(&self, editor: &ProtocolObject<dyn NSEditor>);

        #[optional]
        #[method(objectDidEndEditing:)]
        unsafe fn objectDidEndEditing(&self, editor: &ProtocolObject<dyn NSEditor>);
    }

    unsafe impl ProtocolType for dyn NSEditorRegistration {}
);

extern "C" {
    pub static NSAlignmentBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSAlternateImageBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSAlternateTitleBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSAnimateBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSAnimationDelayBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSArgumentBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSAttributedStringBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSContentArrayBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSContentArrayForMultipleSelectionBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSContentBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSContentDictionaryBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSContentHeightBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSContentObjectBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSContentObjectsBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSContentSetBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSContentValuesBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSContentWidthBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSCriticalValueBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSDataBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSDisplayPatternTitleBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSDisplayPatternValueBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSDocumentEditedBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSDoubleClickArgumentBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSDoubleClickTargetBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSEditableBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSEnabledBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSExcludedKeysBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSFilterPredicateBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSFontBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSFontBoldBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSFontFamilyNameBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSFontItalicBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSFontNameBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSFontSizeBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSHeaderTitleBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSHiddenBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSImageBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSIncludedKeysBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSInitialKeyBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSInitialValueBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSIsIndeterminateBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSLabelBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSLocalizedKeyDictionaryBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSManagedObjectContextBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSMaximumRecentsBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSMaxValueBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSMaxWidthBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSMinValueBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSMinWidthBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSMixedStateImageBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSOffStateImageBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSOnStateImageBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSPositioningRectBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSPredicateBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSRecentSearchesBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSRepresentedFilenameBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSRowHeightBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSSelectedIdentifierBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSSelectedIndexBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSSelectedLabelBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSSelectedObjectBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSSelectedObjectsBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSSelectedTagBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSSelectedValueBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSSelectedValuesBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSSelectionIndexesBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSSelectionIndexPathsBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSSortDescriptorsBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSTargetBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSTextColorBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSTitleBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSToolTipBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSTransparentBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSValueBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSValuePathBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSValueURLBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSVisibleBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSWarningValueBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSWidthBinding: &'static NSBindingName;
}

extern "C" {
    pub static NSAllowsEditingMultipleValuesSelectionBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSAllowsNullArgumentBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSAlwaysPresentsApplicationModalAlertsBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSConditionallySetsEditableBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSConditionallySetsEnabledBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSConditionallySetsHiddenBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSContinuouslyUpdatesValueBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSCreatesSortDescriptorBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSDeletesObjectsOnRemoveBindingsOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSDisplayNameBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSDisplayPatternBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSContentPlacementTagBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSHandlesContentAsCompoundValueBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSInsertsNullPlaceholderBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSInvokesSeparatelyWithArrayObjectsBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSMultipleValuesPlaceholderBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSNoSelectionPlaceholderBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSNotApplicablePlaceholderBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSNullPlaceholderBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSRaisesForNotApplicableKeysBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSPredicateFormatBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSSelectorNameBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSSelectsAllWhenSettingContentBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSValidatesImmediatelyBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSValueTransformerNameBindingOption: &'static NSBindingOption;
}

extern "C" {
    pub static NSValueTransformerBindingOption: &'static NSBindingOption;
}