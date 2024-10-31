//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    #[cfg(all(feature = "UITextInput", feature = "UITextInputTraits"))]
    pub unsafe trait UITextDocumentProxy: UIKeyInput + IsMainThreadOnly {
        #[method_id(@__retain_semantics Other documentContextBeforeInput)]
        unsafe fn documentContextBeforeInput(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other documentContextAfterInput)]
        unsafe fn documentContextAfterInput(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other selectedText)]
        unsafe fn selectedText(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other documentInputMode)]
        unsafe fn documentInputMode(&self) -> Option<Retained<UITextInputMode>>;

        #[method_id(@__retain_semantics Other documentIdentifier)]
        unsafe fn documentIdentifier(&self) -> Retained<NSUUID>;

        #[method(adjustTextPositionByCharacterOffset:)]
        unsafe fn adjustTextPositionByCharacterOffset(&self, offset: NSInteger);

        #[method(setMarkedText:selectedRange:)]
        unsafe fn setMarkedText_selectedRange(
            &self,
            marked_text: &NSString,
            selected_range: NSRange,
        );

        #[method(unmarkText)]
        unsafe fn unmarkText(&self);
    }

    #[cfg(all(feature = "UITextInput", feature = "UITextInputTraits"))]
    unsafe impl ProtocolType for dyn UITextDocumentProxy {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    pub struct UIInputViewController;

    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl ClassType for UIInputViewController {
        #[inherits(UIResponder, NSObject)]
        type Super = UIViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSCoding for UIInputViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSObjectProtocol for UIInputViewController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UIInputViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIContentContainer for UIInputViewController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UIInputViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIResponderStandardEditActions for UIInputViewController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITextInput",
    feature = "UIViewController"
))]
unsafe impl UITextInputDelegate for UIInputViewController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UIInputViewController {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIInputViewController {
        #[cfg(all(feature = "UIInputView", feature = "UIView"))]
        #[method_id(@__retain_semantics Other inputView)]
        pub unsafe fn inputView(&self) -> Option<Retained<UIInputView>>;

        #[cfg(all(feature = "UIInputView", feature = "UIView"))]
        #[method(setInputView:)]
        pub unsafe fn setInputView(&self, input_view: Option<&UIInputView>);

        #[cfg(all(feature = "UITextInput", feature = "UITextInputTraits"))]
        #[method_id(@__retain_semantics Other textDocumentProxy)]
        pub unsafe fn textDocumentProxy(&self)
            -> Retained<ProtocolObject<dyn UITextDocumentProxy>>;

        #[method_id(@__retain_semantics Other primaryLanguage)]
        pub unsafe fn primaryLanguage(&self) -> Option<Retained<NSString>>;

        #[method(setPrimaryLanguage:)]
        pub unsafe fn setPrimaryLanguage(&self, primary_language: Option<&NSString>);

        #[method(hasDictationKey)]
        pub unsafe fn hasDictationKey(&self) -> bool;

        #[method(setHasDictationKey:)]
        pub unsafe fn setHasDictationKey(&self, has_dictation_key: bool);

        #[method(hasFullAccess)]
        pub unsafe fn hasFullAccess(&self) -> bool;

        #[method(needsInputModeSwitchKey)]
        pub unsafe fn needsInputModeSwitchKey(&self) -> bool;

        #[method(dismissKeyboard)]
        pub unsafe fn dismissKeyboard(&self);

        #[method(advanceToNextInputMode)]
        pub unsafe fn advanceToNextInputMode(&self);

        #[cfg(all(feature = "UIEvent", feature = "UIView"))]
        #[method(handleInputModeListFromView:withEvent:)]
        pub unsafe fn handleInputModeListFromView_withEvent(&self, view: &UIView, event: &UIEvent);

        #[cfg(all(feature = "UILexicon", feature = "block2"))]
        #[method(requestSupplementaryLexiconWithCompletion:)]
        pub unsafe fn requestSupplementaryLexiconWithCompletion(
            &self,
            completion_handler: &block2::Block<dyn Fn(NonNull<UILexicon>)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UIViewController`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIInputViewController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIInputViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
