//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIMotionEffect;

    unsafe impl ClassType for UIMotionEffect {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCoding for UIMotionEffect {}

unsafe impl NSCopying for UIMotionEffect {}

unsafe impl NSObjectProtocol for UIMotionEffect {}

extern_methods!(
    unsafe impl UIMotionEffect {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "UIGeometry")]
        #[method_id(@__retain_semantics Other keyPathsAndRelativeValuesForViewerOffset:)]
        pub unsafe fn keyPathsAndRelativeValuesForViewerOffset(
            &self,
            viewer_offset: UIOffset,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIMotionEffect {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIInterpolatingMotionEffectType(pub NSInteger);
impl UIInterpolatingMotionEffectType {
    #[doc(alias = "UIInterpolatingMotionEffectTypeTiltAlongHorizontalAxis")]
    pub const TiltAlongHorizontalAxis: Self = Self(0);
    #[doc(alias = "UIInterpolatingMotionEffectTypeTiltAlongVerticalAxis")]
    pub const TiltAlongVerticalAxis: Self = Self(1);
}

unsafe impl Encode for UIInterpolatingMotionEffectType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIInterpolatingMotionEffectType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIInterpolatingMotionEffect;

    unsafe impl ClassType for UIInterpolatingMotionEffect {
        #[inherits(NSObject)]
        type Super = UIMotionEffect;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCoding for UIInterpolatingMotionEffect {}

unsafe impl NSCopying for UIInterpolatingMotionEffect {}

unsafe impl NSObjectProtocol for UIInterpolatingMotionEffect {}

extern_methods!(
    unsafe impl UIInterpolatingMotionEffect {
        #[method_id(@__retain_semantics Init initWithKeyPath:type:)]
        pub unsafe fn initWithKeyPath_type(
            this: Allocated<Self>,
            key_path: &NSString,
            r#type: UIInterpolatingMotionEffectType,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other keyPath)]
        pub unsafe fn keyPath(&self) -> Retained<NSString>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> UIInterpolatingMotionEffectType;

        #[method_id(@__retain_semantics Other minimumRelativeValue)]
        pub unsafe fn minimumRelativeValue(&self) -> Option<Retained<AnyObject>>;

        #[method(setMinimumRelativeValue:)]
        pub unsafe fn setMinimumRelativeValue(&self, minimum_relative_value: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other maximumRelativeValue)]
        pub unsafe fn maximumRelativeValue(&self) -> Option<Retained<AnyObject>>;

        #[method(setMaximumRelativeValue:)]
        pub unsafe fn setMaximumRelativeValue(&self, maximum_relative_value: Option<&AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIMotionEffect`
    unsafe impl UIInterpolatingMotionEffect {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIInterpolatingMotionEffect {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIMotionEffectGroup;

    unsafe impl ClassType for UIMotionEffectGroup {
        #[inherits(NSObject)]
        type Super = UIMotionEffect;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCoding for UIMotionEffectGroup {}

unsafe impl NSCopying for UIMotionEffectGroup {}

unsafe impl NSObjectProtocol for UIMotionEffectGroup {}

extern_methods!(
    unsafe impl UIMotionEffectGroup {
        #[method_id(@__retain_semantics Other motionEffects)]
        pub unsafe fn motionEffects(&self) -> Option<Retained<NSArray<UIMotionEffect>>>;

        #[method(setMotionEffects:)]
        pub unsafe fn setMotionEffects(&self, motion_effects: Option<&NSArray<UIMotionEffect>>);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIMotionEffect`
    unsafe impl UIMotionEffectGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIMotionEffectGroup {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
