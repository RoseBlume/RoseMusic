//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDynamicItemCollisionBoundsType(pub NSUInteger);
impl UIDynamicItemCollisionBoundsType {
    #[doc(alias = "UIDynamicItemCollisionBoundsTypeRectangle")]
    pub const Rectangle: Self = Self(0);
    #[doc(alias = "UIDynamicItemCollisionBoundsTypeEllipse")]
    pub const Ellipse: Self = Self(1);
    #[doc(alias = "UIDynamicItemCollisionBoundsTypePath")]
    pub const Path: Self = Self(2);
}

unsafe impl Encode for UIDynamicItemCollisionBoundsType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIDynamicItemCollisionBoundsType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait UIDynamicItem: NSObjectProtocol + IsMainThreadOnly {
        #[method(center)]
        unsafe fn center(&self) -> CGPoint;

        #[method(setCenter:)]
        unsafe fn setCenter(&self, center: CGPoint);

        #[method(bounds)]
        unsafe fn bounds(&self) -> CGRect;

        #[method(transform)]
        unsafe fn transform(&self) -> CGAffineTransform;

        #[method(setTransform:)]
        unsafe fn setTransform(&self, transform: CGAffineTransform);

        #[optional]
        #[method(collisionBoundsType)]
        unsafe fn collisionBoundsType(&self) -> UIDynamicItemCollisionBoundsType;

        #[cfg(feature = "UIBezierPath")]
        #[optional]
        #[method_id(@__retain_semantics Other collisionBoundingPath)]
        unsafe fn collisionBoundingPath(&self) -> Retained<UIBezierPath>;
    }

    unsafe impl ProtocolType for dyn UIDynamicItem {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIDynamicItemGroup;

    unsafe impl ClassType for UIDynamicItemGroup {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIDynamicItemGroup {}

unsafe impl UIDynamicItem for UIDynamicItemGroup {}

extern_methods!(
    unsafe impl UIDynamicItemGroup {
        #[method_id(@__retain_semantics Init initWithItems:)]
        pub unsafe fn initWithItems(
            this: Allocated<Self>,
            items: &NSArray<ProtocolObject<dyn UIDynamicItem>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Retained<NSArray<ProtocolObject<dyn UIDynamicItem>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIDynamicItemGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIDynamicBehavior;

    unsafe impl ClassType for UIDynamicBehavior {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIDynamicBehavior {}

extern_methods!(
    unsafe impl UIDynamicBehavior {
        #[method(addChildBehavior:)]
        pub unsafe fn addChildBehavior(&self, behavior: &UIDynamicBehavior);

        #[method(removeChildBehavior:)]
        pub unsafe fn removeChildBehavior(&self, behavior: &UIDynamicBehavior);

        #[method_id(@__retain_semantics Other childBehaviors)]
        pub unsafe fn childBehaviors(&self) -> Retained<NSArray<UIDynamicBehavior>>;

        #[cfg(feature = "block2")]
        #[method(action)]
        pub unsafe fn action(&self) -> *mut block2::Block<dyn Fn()>;

        #[cfg(feature = "block2")]
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<&block2::Block<dyn Fn()>>);

        #[cfg(feature = "UIDynamicAnimator")]
        #[method(willMoveToAnimator:)]
        pub unsafe fn willMoveToAnimator(&self, dynamic_animator: Option<&UIDynamicAnimator>);

        #[cfg(feature = "UIDynamicAnimator")]
        #[method_id(@__retain_semantics Other dynamicAnimator)]
        pub unsafe fn dynamicAnimator(&self) -> Option<Retained<UIDynamicAnimator>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIDynamicBehavior {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
