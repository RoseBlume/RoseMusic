//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIListContentImageProperties;

    unsafe impl ClassType for UIListContentImageProperties {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCoding for UIListContentImageProperties {}

unsafe impl NSCopying for UIListContentImageProperties {}

unsafe impl NSObjectProtocol for UIListContentImageProperties {}

unsafe impl NSSecureCoding for UIListContentImageProperties {}

extern_methods!(
    unsafe impl UIListContentImageProperties {
        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        #[method_id(@__retain_semantics Other preferredSymbolConfiguration)]
        pub unsafe fn preferredSymbolConfiguration(
            &self,
        ) -> Option<Retained<UIImageSymbolConfiguration>>;

        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        #[method(setPreferredSymbolConfiguration:)]
        pub unsafe fn setPreferredSymbolConfiguration(
            &self,
            preferred_symbol_configuration: Option<&UIImageSymbolConfiguration>,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other tintColor)]
        pub unsafe fn tintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setTintColor:)]
        pub unsafe fn setTintColor(&self, tint_color: Option<&UIColor>);

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[method(tintColorTransformer)]
        pub unsafe fn tintColorTransformer(&self) -> UIConfigurationColorTransformer;

        #[cfg(all(
            feature = "UIColor",
            feature = "UIConfigurationColorTransformer",
            feature = "block2"
        ))]
        #[method(setTintColorTransformer:)]
        pub unsafe fn setTintColorTransformer(
            &self,
            tint_color_transformer: UIConfigurationColorTransformer,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other resolvedTintColorForTintColor:)]
        pub unsafe fn resolvedTintColorForTintColor(
            &self,
            tint_color: &UIColor,
        ) -> Retained<UIColor>;

        #[method(cornerRadius)]
        pub unsafe fn cornerRadius(&self) -> CGFloat;

        #[method(setCornerRadius:)]
        pub unsafe fn setCornerRadius(&self, corner_radius: CGFloat);

        #[method(maximumSize)]
        pub unsafe fn maximumSize(&self) -> CGSize;

        #[method(setMaximumSize:)]
        pub unsafe fn setMaximumSize(&self, maximum_size: CGSize);

        #[method(reservedLayoutSize)]
        pub unsafe fn reservedLayoutSize(&self) -> CGSize;

        #[method(setReservedLayoutSize:)]
        pub unsafe fn setReservedLayoutSize(&self, reserved_layout_size: CGSize);

        #[method(accessibilityIgnoresInvertColors)]
        pub unsafe fn accessibilityIgnoresInvertColors(&self) -> bool;

        #[method(setAccessibilityIgnoresInvertColors:)]
        pub unsafe fn setAccessibilityIgnoresInvertColors(
            &self,
            accessibility_ignores_invert_colors: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIListContentImageProperties {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern "C" {
    pub static UIListContentImageStandardDimension: CGFloat;
}