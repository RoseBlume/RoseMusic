//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static WebKitErrorDomain: Option<&'static NSString>;
}

extern "C" {
    pub static WebKitErrorMIMETypeKey: Option<&'static NSString>;
}

extern "C" {
    pub static WebKitErrorPlugInNameKey: Option<&'static NSString>;
}

extern "C" {
    pub static WebKitErrorPlugInPageURLStringKey: Option<&'static NSString>;
}

#[deprecated]
pub const WebKitErrorCannotShowMIMEType: c_uint = 100;
#[deprecated]
pub const WebKitErrorCannotShowURL: c_uint = 101;
#[deprecated]
pub const WebKitErrorFrameLoadInterruptedByPolicyChange: c_uint = 102;

#[deprecated]
pub const WebKitErrorCannotFindPlugIn: c_uint = 200;
#[deprecated]
pub const WebKitErrorCannotLoadPlugIn: c_uint = 201;
#[deprecated]
pub const WebKitErrorJavaUnavailable: c_uint = 202;
#[deprecated]
pub const WebKitErrorBlockedPlugInVersion: c_uint = 203;
