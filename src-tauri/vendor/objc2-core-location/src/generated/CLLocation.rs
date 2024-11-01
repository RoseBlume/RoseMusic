//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub type CLLocationDegrees = c_double;

pub type CLLocationAccuracy = c_double;

pub type CLLocationSpeed = c_double;

pub type CLLocationSpeedAccuracy = c_double;

pub type CLLocationDirection = c_double;

pub type CLLocationDirectionAccuracy = c_double;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLLocationCoordinate2D {
    pub latitude: CLLocationDegrees,
    pub longitude: CLLocationDegrees,
}

unsafe impl Encode for CLLocationCoordinate2D {
    const ENCODING: Encoding = Encoding::Struct(
        "CLLocationCoordinate2D",
        &[<CLLocationDegrees>::ENCODING, <CLLocationDegrees>::ENCODING],
    );
}

unsafe impl RefEncode for CLLocationCoordinate2D {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub type CLLocationDistance = c_double;

extern "C" {
    pub static kCLDistanceFilterNone: CLLocationDistance;
}

extern "C" {
    pub static kCLLocationAccuracyBestForNavigation: CLLocationAccuracy;
}

extern "C" {
    pub static kCLLocationAccuracyBest: CLLocationAccuracy;
}

extern "C" {
    pub static kCLLocationAccuracyNearestTenMeters: CLLocationAccuracy;
}

extern "C" {
    pub static kCLLocationAccuracyHundredMeters: CLLocationAccuracy;
}

extern "C" {
    pub static kCLLocationAccuracyKilometer: CLLocationAccuracy;
}

extern "C" {
    pub static kCLLocationAccuracyThreeKilometers: CLLocationAccuracy;
}

extern "C" {
    pub static kCLLocationAccuracyReduced: CLLocationAccuracy;
}

extern "C" {
    pub static CLLocationDistanceMax: CLLocationDistance;
}

extern "C" {
    pub static CLTimeIntervalMax: NSTimeInterval;
}

extern "C" {
    pub static kCLLocationCoordinate2DInvalid: CLLocationCoordinate2D;
}

extern "C" {
    pub fn CLLocationCoordinate2DIsValid(coord: CLLocationCoordinate2D) -> Bool;
}

extern "C" {
    pub fn CLLocationCoordinate2DMake(
        latitude: CLLocationDegrees,
        longitude: CLLocationDegrees,
    ) -> CLLocationCoordinate2D;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLFloor;

    unsafe impl ClassType for CLFloor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CLFloor {}

unsafe impl NSCopying for CLFloor {}

unsafe impl NSObjectProtocol for CLFloor {}

unsafe impl NSSecureCoding for CLFloor {}

extern_methods!(
    unsafe impl CLFloor {
        #[method(level)]
        pub unsafe fn level(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CLFloor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLLocationSourceInformation;

    unsafe impl ClassType for CLLocationSourceInformation {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CLLocationSourceInformation {}

unsafe impl NSCopying for CLLocationSourceInformation {}

unsafe impl NSObjectProtocol for CLLocationSourceInformation {}

unsafe impl NSSecureCoding for CLLocationSourceInformation {}

extern_methods!(
    unsafe impl CLLocationSourceInformation {
        #[method_id(@__retain_semantics Init initWithSoftwareSimulationState:andExternalAccessoryState:)]
        pub unsafe fn initWithSoftwareSimulationState_andExternalAccessoryState(
            this: Allocated<Self>,
            is_software: bool,
            is_accessory: bool,
        ) -> Retained<Self>;

        #[method(isSimulatedBySoftware)]
        pub unsafe fn isSimulatedBySoftware(&self) -> bool;

        #[method(isProducedByAccessory)]
        pub unsafe fn isProducedByAccessory(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CLLocationSourceInformation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLLocation;

    unsafe impl ClassType for CLLocation {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CLLocation {}

unsafe impl Sync for CLLocation {}

unsafe impl NSCoding for CLLocation {}

unsafe impl NSCopying for CLLocation {}

unsafe impl NSObjectProtocol for CLLocation {}

unsafe impl NSSecureCoding for CLLocation {}

extern_methods!(
    unsafe impl CLLocation {
        #[method_id(@__retain_semantics Init initWithLatitude:longitude:)]
        pub unsafe fn initWithLatitude_longitude(
            this: Allocated<Self>,
            latitude: CLLocationDegrees,
            longitude: CLLocationDegrees,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:timestamp:)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_timestamp(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            timestamp: &NSDate,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:course:speed:timestamp:)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_speed_timestamp(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            course: CLLocationDirection,
            speed: CLLocationSpeed,
            timestamp: &NSDate,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:course:courseAccuracy:speed:speedAccuracy:timestamp:)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_courseAccuracy_speed_speedAccuracy_timestamp(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            course: CLLocationDirection,
            course_accuracy: CLLocationDirectionAccuracy,
            speed: CLLocationSpeed,
            speed_accuracy: CLLocationSpeedAccuracy,
            timestamp: &NSDate,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:course:courseAccuracy:speed:speedAccuracy:timestamp:sourceInfo:)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_courseAccuracy_speed_speedAccuracy_timestamp_sourceInfo(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            course: CLLocationDirection,
            course_accuracy: CLLocationDirectionAccuracy,
            speed: CLLocationSpeed,
            speed_accuracy: CLLocationSpeedAccuracy,
            timestamp: &NSDate,
            source_info: &CLLocationSourceInformation,
        ) -> Retained<Self>;

        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[method(altitude)]
        pub unsafe fn altitude(&self) -> CLLocationDistance;

        #[method(ellipsoidalAltitude)]
        pub unsafe fn ellipsoidalAltitude(&self) -> CLLocationDistance;

        #[method(horizontalAccuracy)]
        pub unsafe fn horizontalAccuracy(&self) -> CLLocationAccuracy;

        #[method(verticalAccuracy)]
        pub unsafe fn verticalAccuracy(&self) -> CLLocationAccuracy;

        #[method(course)]
        pub unsafe fn course(&self) -> CLLocationDirection;

        #[method(courseAccuracy)]
        pub unsafe fn courseAccuracy(&self) -> CLLocationDirectionAccuracy;

        #[method(speed)]
        pub unsafe fn speed(&self) -> CLLocationSpeed;

        #[method(speedAccuracy)]
        pub unsafe fn speedAccuracy(&self) -> CLLocationSpeedAccuracy;

        #[method_id(@__retain_semantics Other timestamp)]
        pub unsafe fn timestamp(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other floor)]
        pub unsafe fn floor(&self) -> Option<Retained<CLFloor>>;

        #[method_id(@__retain_semantics Other sourceInformation)]
        pub unsafe fn sourceInformation(&self) -> Option<Retained<CLLocationSourceInformation>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CLLocation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
