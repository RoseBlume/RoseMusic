//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait NSTextViewportLayoutControllerDelegate: NSObjectProtocol {
        #[method(viewportBoundsForTextViewportLayoutController:)]
        unsafe fn viewportBoundsForTextViewportLayoutController(
            &self,
            text_viewport_layout_controller: &NSTextViewportLayoutController,
        ) -> CGRect;

        #[cfg(feature = "NSTextLayoutFragment")]
        #[method(textViewportLayoutController:configureRenderingSurfaceForTextLayoutFragment:)]
        unsafe fn textViewportLayoutController_configureRenderingSurfaceForTextLayoutFragment(
            &self,
            text_viewport_layout_controller: &NSTextViewportLayoutController,
            text_layout_fragment: &NSTextLayoutFragment,
        );

        #[optional]
        #[method(textViewportLayoutControllerWillLayout:)]
        unsafe fn textViewportLayoutControllerWillLayout(
            &self,
            text_viewport_layout_controller: &NSTextViewportLayoutController,
        );

        #[optional]
        #[method(textViewportLayoutControllerDidLayout:)]
        unsafe fn textViewportLayoutControllerDidLayout(
            &self,
            text_viewport_layout_controller: &NSTextViewportLayoutController,
        );
    }

    unsafe impl ProtocolType for dyn NSTextViewportLayoutControllerDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextViewportLayoutController;

    unsafe impl ClassType for NSTextViewportLayoutController {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSTextViewportLayoutController {}

extern_methods!(
    unsafe impl NSTextViewportLayoutController {
        #[cfg(feature = "NSTextLayoutManager")]
        #[method_id(@__retain_semantics Init initWithTextLayoutManager:)]
        pub unsafe fn initWithTextLayoutManager(
            this: Allocated<Self>,
            text_layout_manager: &NSTextLayoutManager,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSTextViewportLayoutControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTextViewportLayoutControllerDelegate>>,
        );

        #[cfg(feature = "NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Retained<NSTextLayoutManager>>;

        #[method(viewportBounds)]
        pub unsafe fn viewportBounds(&self) -> CGRect;

        #[cfg(feature = "NSTextRange")]
        #[method_id(@__retain_semantics Other viewportRange)]
        pub unsafe fn viewportRange(&self) -> Option<Retained<NSTextRange>>;

        #[method(layoutViewport)]
        pub unsafe fn layoutViewport(&self);

        #[cfg(feature = "NSTextRange")]
        #[method(relocateViewportToTextLocation:)]
        pub unsafe fn relocateViewportToTextLocation(
            &self,
            text_location: &ProtocolObject<dyn NSTextLocation>,
        ) -> CGFloat;

        #[method(adjustViewportByVerticalOffset:)]
        pub unsafe fn adjustViewportByVerticalOffset(&self, vertical_offset: CGFloat);
    }
);