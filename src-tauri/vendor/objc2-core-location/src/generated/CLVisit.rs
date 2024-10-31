//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLVisit;

    unsafe impl ClassType for CLVisit {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CLVisit {}

unsafe impl NSCopying for CLVisit {}

unsafe impl NSObjectProtocol for CLVisit {}

unsafe impl NSSecureCoding for CLVisit {}

extern_methods!(
    unsafe impl CLVisit {
        #[method_id(@__retain_semantics Other arrivalDate)]
        pub unsafe fn arrivalDate(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other departureDate)]
        pub unsafe fn departureDate(&self) -> Retained<NSDate>;

        #[cfg(feature = "CLLocation")]
        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[cfg(feature = "CLLocation")]
        #[method(horizontalAccuracy)]
        pub unsafe fn horizontalAccuracy(&self) -> CLLocationAccuracy;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CLVisit {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
