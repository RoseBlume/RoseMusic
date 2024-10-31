//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIDynamicBehavior")]
    pub struct UIGravityBehavior;

    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl ClassType for UIGravityBehavior {
        #[inherits(NSObject)]
        type Super = UIDynamicBehavior;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "UIDynamicBehavior")]
unsafe impl NSObjectProtocol for UIGravityBehavior {}

extern_methods!(
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UIGravityBehavior {
        #[method_id(@__retain_semantics Init initWithItems:)]
        pub unsafe fn initWithItems(
            this: Allocated<Self>,
            items: &NSArray<ProtocolObject<dyn UIDynamicItem>>,
        ) -> Retained<Self>;

        #[method(addItem:)]
        pub unsafe fn addItem(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[method(removeItem:)]
        pub unsafe fn removeItem(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Retained<NSArray<ProtocolObject<dyn UIDynamicItem>>>;

        #[method(gravityDirection)]
        pub unsafe fn gravityDirection(&self) -> CGVector;

        #[method(setGravityDirection:)]
        pub unsafe fn setGravityDirection(&self, gravity_direction: CGVector);

        #[method(angle)]
        pub unsafe fn angle(&self) -> CGFloat;

        #[method(setAngle:)]
        pub unsafe fn setAngle(&self, angle: CGFloat);

        #[method(magnitude)]
        pub unsafe fn magnitude(&self) -> CGFloat;

        #[method(setMagnitude:)]
        pub unsafe fn setMagnitude(&self, magnitude: CGFloat);

        #[method(setAngle:magnitude:)]
        pub unsafe fn setAngle_magnitude(&self, angle: CGFloat, magnitude: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UIGravityBehavior {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);