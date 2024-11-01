//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-image")]
#[cfg(target_vendor = "apple")]
use objc2_core_image::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type NSGraphicsContextAttributeKey = NSString;

extern "C" {
    pub static NSGraphicsContextDestinationAttributeName: &'static NSGraphicsContextAttributeKey;
}

extern "C" {
    pub static NSGraphicsContextRepresentationFormatAttributeName:
        &'static NSGraphicsContextAttributeKey;
}

// NS_TYPED_ENUM
pub type NSGraphicsContextRepresentationFormatName = NSString;

extern "C" {
    pub static NSGraphicsContextPSFormat: &'static NSGraphicsContextRepresentationFormatName;
}

extern "C" {
    pub static NSGraphicsContextPDFFormat: &'static NSGraphicsContextRepresentationFormatName;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSImageInterpolation(pub NSUInteger);
impl NSImageInterpolation {
    #[doc(alias = "NSImageInterpolationDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSImageInterpolationNone")]
    pub const None: Self = Self(1);
    #[doc(alias = "NSImageInterpolationLow")]
    pub const Low: Self = Self(2);
    #[doc(alias = "NSImageInterpolationMedium")]
    pub const Medium: Self = Self(4);
    #[doc(alias = "NSImageInterpolationHigh")]
    pub const High: Self = Self(3);
}

unsafe impl Encode for NSImageInterpolation {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSImageInterpolation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSGraphicsContext;

    unsafe impl ClassType for NSGraphicsContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSGraphicsContext {}

extern_methods!(
    unsafe impl NSGraphicsContext {
        #[method_id(@__retain_semantics Other graphicsContextWithAttributes:)]
        pub unsafe fn graphicsContextWithAttributes(
            attributes: &NSDictionary<NSGraphicsContextAttributeKey, AnyObject>,
        ) -> Option<Retained<NSGraphicsContext>>;

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[method_id(@__retain_semantics Other graphicsContextWithWindow:)]
        pub unsafe fn graphicsContextWithWindow(window: &NSWindow) -> Retained<NSGraphicsContext>;

        #[cfg(all(feature = "NSBitmapImageRep", feature = "NSImageRep"))]
        #[method_id(@__retain_semantics Other graphicsContextWithBitmapImageRep:)]
        pub unsafe fn graphicsContextWithBitmapImageRep(
            bitmap_rep: &NSBitmapImageRep,
        ) -> Option<Retained<NSGraphicsContext>>;

        #[method_id(@__retain_semantics Other currentContext)]
        pub unsafe fn currentContext() -> Option<Retained<NSGraphicsContext>>;

        #[method(setCurrentContext:)]
        pub unsafe fn setCurrentContext(current_context: Option<&NSGraphicsContext>);

        #[method(currentContextDrawingToScreen)]
        pub unsafe fn currentContextDrawingToScreen() -> bool;

        #[method(saveGraphicsState)]
        pub unsafe fn saveGraphicsState_class();

        #[method(restoreGraphicsState)]
        pub unsafe fn restoreGraphicsState_class();

        #[method_id(@__retain_semantics Other attributes)]
        pub unsafe fn attributes(
            &self,
        ) -> Option<Retained<NSDictionary<NSGraphicsContextAttributeKey, AnyObject>>>;

        #[method(isDrawingToScreen)]
        pub unsafe fn isDrawingToScreen(&self) -> bool;

        #[method(saveGraphicsState)]
        pub unsafe fn saveGraphicsState(&self);

        #[method(restoreGraphicsState)]
        pub unsafe fn restoreGraphicsState(&self);

        #[method(flushGraphics)]
        pub unsafe fn flushGraphics(&self);

        #[method(isFlipped)]
        pub unsafe fn isFlipped(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSGraphicsContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSGraphicsContext_RenderingOptions
    unsafe impl NSGraphicsContext {
        #[method(shouldAntialias)]
        pub unsafe fn shouldAntialias(&self) -> bool;

        #[method(setShouldAntialias:)]
        pub unsafe fn setShouldAntialias(&self, should_antialias: bool);

        #[method(imageInterpolation)]
        pub unsafe fn imageInterpolation(&self) -> NSImageInterpolation;

        #[method(setImageInterpolation:)]
        pub unsafe fn setImageInterpolation(&self, image_interpolation: NSImageInterpolation);

        #[method(patternPhase)]
        pub unsafe fn patternPhase(&self) -> NSPoint;

        #[method(setPatternPhase:)]
        pub unsafe fn setPatternPhase(&self, pattern_phase: NSPoint);

        #[cfg(feature = "NSGraphics")]
        #[method(compositingOperation)]
        pub unsafe fn compositingOperation(&self) -> NSCompositingOperation;

        #[cfg(feature = "NSGraphics")]
        #[method(setCompositingOperation:)]
        pub unsafe fn setCompositingOperation(&self, compositing_operation: NSCompositingOperation);

        #[cfg(feature = "NSGraphics")]
        #[method(colorRenderingIntent)]
        pub unsafe fn colorRenderingIntent(&self) -> NSColorRenderingIntent;

        #[cfg(feature = "NSGraphics")]
        #[method(setColorRenderingIntent:)]
        pub unsafe fn setColorRenderingIntent(
            &self,
            color_rendering_intent: NSColorRenderingIntent,
        );
    }
);

extern_methods!(
    /// NSQuartzCoreAdditions
    unsafe impl NSGraphicsContext {
        #[cfg(feature = "objc2-core-image")]
        #[cfg(target_vendor = "apple")]
        #[method_id(@__retain_semantics Other CIContext)]
        pub unsafe fn CIContext(&self) -> Option<Retained<CIContext>>;
    }
);

extern_methods!(
    /// NSGraphicsContextDeprecated
    unsafe impl NSGraphicsContext {
        #[deprecated = "This method has no effect"]
        #[method(setGraphicsState:)]
        pub unsafe fn setGraphicsState(g_state: NSInteger);

        #[deprecated]
        #[method_id(@__retain_semantics Other focusStack)]
        pub unsafe fn focusStack(&self) -> Option<Retained<AnyObject>>;

        #[deprecated]
        #[method(setFocusStack:)]
        pub unsafe fn setFocusStack(&self, stack: Option<&AnyObject>);

        #[deprecated]
        #[method_id(@__retain_semantics Other graphicsContextWithGraphicsPort:flipped:)]
        pub unsafe fn graphicsContextWithGraphicsPort_flipped(
            graphics_port: NonNull<c_void>,
            initial_flipped_state: bool,
        ) -> Retained<NSGraphicsContext>;

        #[deprecated]
        #[method(graphicsPort)]
        pub unsafe fn graphicsPort(&self) -> NonNull<c_void>;
    }
);