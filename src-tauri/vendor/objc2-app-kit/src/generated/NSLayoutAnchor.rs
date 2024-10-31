//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLayoutAnchor<AnchorType: ?Sized = AnyObject> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut AnchorType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<AnchorType: ?Sized + Message> ClassType for NSLayoutAnchor<AnchorType> {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

unsafe impl<AnchorType: ?Sized + NSCoding> NSCoding for NSLayoutAnchor<AnchorType> {}

unsafe impl<AnchorType: ?Sized + IsIdCloneable> NSCopying for NSLayoutAnchor<AnchorType> {}

unsafe impl<AnchorType: ?Sized> NSObjectProtocol for NSLayoutAnchor<AnchorType> {}

extern_methods!(
    unsafe impl<AnchorType: Message> NSLayoutAnchor<AnchorType> {
        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintEqualToAnchor:)]
        pub unsafe fn constraintEqualToAnchor(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintGreaterThanOrEqualToAnchor:)]
        pub unsafe fn constraintGreaterThanOrEqualToAnchor(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintLessThanOrEqualToAnchor:)]
        pub unsafe fn constraintLessThanOrEqualToAnchor(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintEqualToAnchor:constant:)]
        pub unsafe fn constraintEqualToAnchor_constant(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
            c: CGFloat,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintGreaterThanOrEqualToAnchor:constant:)]
        pub unsafe fn constraintGreaterThanOrEqualToAnchor_constant(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
            c: CGFloat,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintLessThanOrEqualToAnchor:constant:)]
        pub unsafe fn constraintLessThanOrEqualToAnchor_constant(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
            c: CGFloat,
        ) -> Retained<NSLayoutConstraint>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other item)]
        pub unsafe fn item(&self) -> Option<Retained<AnyObject>>;

        #[method(hasAmbiguousLayout)]
        pub unsafe fn hasAmbiguousLayout(&self) -> bool;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintsAffectingLayout)]
        pub unsafe fn constraintsAffectingLayout(&self) -> Retained<NSArray<NSLayoutConstraint>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<AnchorType: Message> NSLayoutAnchor<AnchorType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSLayoutXAxisAnchor;

    unsafe impl ClassType for NSLayoutXAxisAnchor {
        #[inherits(NSObject)]
        type Super = NSLayoutAnchor;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NSLayoutXAxisAnchor {}

unsafe impl NSCopying for NSLayoutXAxisAnchor {}

unsafe impl NSObjectProtocol for NSLayoutXAxisAnchor {}

extern_methods!(
    unsafe impl NSLayoutXAxisAnchor {
        #[method_id(@__retain_semantics Other anchorWithOffsetToAnchor:)]
        pub unsafe fn anchorWithOffsetToAnchor(
            &self,
            other_anchor: &NSLayoutXAxisAnchor,
        ) -> Retained<NSLayoutDimension>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintEqualToSystemSpacingAfterAnchor:multiplier:)]
        pub unsafe fn constraintEqualToSystemSpacingAfterAnchor_multiplier(
            &self,
            anchor: &NSLayoutXAxisAnchor,
            multiplier: CGFloat,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintGreaterThanOrEqualToSystemSpacingAfterAnchor:multiplier:)]
        pub unsafe fn constraintGreaterThanOrEqualToSystemSpacingAfterAnchor_multiplier(
            &self,
            anchor: &NSLayoutXAxisAnchor,
            multiplier: CGFloat,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintLessThanOrEqualToSystemSpacingAfterAnchor:multiplier:)]
        pub unsafe fn constraintLessThanOrEqualToSystemSpacingAfterAnchor_multiplier(
            &self,
            anchor: &NSLayoutXAxisAnchor,
            multiplier: CGFloat,
        ) -> Retained<NSLayoutConstraint>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSLayoutXAxisAnchor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSLayoutYAxisAnchor;

    unsafe impl ClassType for NSLayoutYAxisAnchor {
        #[inherits(NSObject)]
        type Super = NSLayoutAnchor;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NSLayoutYAxisAnchor {}

unsafe impl NSCopying for NSLayoutYAxisAnchor {}

unsafe impl NSObjectProtocol for NSLayoutYAxisAnchor {}

extern_methods!(
    unsafe impl NSLayoutYAxisAnchor {
        #[method_id(@__retain_semantics Other anchorWithOffsetToAnchor:)]
        pub unsafe fn anchorWithOffsetToAnchor(
            &self,
            other_anchor: &NSLayoutYAxisAnchor,
        ) -> Retained<NSLayoutDimension>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintEqualToSystemSpacingBelowAnchor:multiplier:)]
        pub unsafe fn constraintEqualToSystemSpacingBelowAnchor_multiplier(
            &self,
            anchor: &NSLayoutYAxisAnchor,
            multiplier: CGFloat,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintGreaterThanOrEqualToSystemSpacingBelowAnchor:multiplier:)]
        pub unsafe fn constraintGreaterThanOrEqualToSystemSpacingBelowAnchor_multiplier(
            &self,
            anchor: &NSLayoutYAxisAnchor,
            multiplier: CGFloat,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintLessThanOrEqualToSystemSpacingBelowAnchor:multiplier:)]
        pub unsafe fn constraintLessThanOrEqualToSystemSpacingBelowAnchor_multiplier(
            &self,
            anchor: &NSLayoutYAxisAnchor,
            multiplier: CGFloat,
        ) -> Retained<NSLayoutConstraint>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSLayoutYAxisAnchor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSLayoutDimension;

    unsafe impl ClassType for NSLayoutDimension {
        #[inherits(NSObject)]
        type Super = NSLayoutAnchor;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NSLayoutDimension {}

unsafe impl NSCopying for NSLayoutDimension {}

unsafe impl NSObjectProtocol for NSLayoutDimension {}

extern_methods!(
    unsafe impl NSLayoutDimension {
        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintEqualToConstant:)]
        pub unsafe fn constraintEqualToConstant(&self, c: CGFloat) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintGreaterThanOrEqualToConstant:)]
        pub unsafe fn constraintGreaterThanOrEqualToConstant(
            &self,
            c: CGFloat,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintLessThanOrEqualToConstant:)]
        pub unsafe fn constraintLessThanOrEqualToConstant(
            &self,
            c: CGFloat,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintEqualToAnchor:multiplier:)]
        pub unsafe fn constraintEqualToAnchor_multiplier(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintGreaterThanOrEqualToAnchor:multiplier:)]
        pub unsafe fn constraintGreaterThanOrEqualToAnchor_multiplier(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintLessThanOrEqualToAnchor:multiplier:)]
        pub unsafe fn constraintLessThanOrEqualToAnchor_multiplier(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintEqualToAnchor:multiplier:constant:)]
        pub unsafe fn constraintEqualToAnchor_multiplier_constant(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
            c: CGFloat,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintGreaterThanOrEqualToAnchor:multiplier:constant:)]
        pub unsafe fn constraintGreaterThanOrEqualToAnchor_multiplier_constant(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
            c: CGFloat,
        ) -> Retained<NSLayoutConstraint>;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintLessThanOrEqualToAnchor:multiplier:constant:)]
        pub unsafe fn constraintLessThanOrEqualToAnchor_multiplier_constant(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
            c: CGFloat,
        ) -> Retained<NSLayoutConstraint>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSLayoutDimension {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
