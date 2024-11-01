//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WebNavigationType(pub NSInteger);
impl WebNavigationType {
    #[deprecated]
    #[doc(alias = "WebNavigationTypeLinkClicked")]
    pub const LinkClicked: Self = Self(0);
    #[deprecated]
    #[doc(alias = "WebNavigationTypeFormSubmitted")]
    pub const FormSubmitted: Self = Self(1);
    #[deprecated]
    #[doc(alias = "WebNavigationTypeBackForward")]
    pub const BackForward: Self = Self(2);
    #[deprecated]
    #[doc(alias = "WebNavigationTypeReload")]
    pub const Reload: Self = Self(3);
    #[deprecated]
    #[doc(alias = "WebNavigationTypeFormResubmitted")]
    pub const FormResubmitted: Self = Self(4);
    #[deprecated]
    #[doc(alias = "WebNavigationTypeOther")]
    pub const Other: Self = Self(5);
}

unsafe impl Encode for WebNavigationType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WebNavigationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static WebActionNavigationTypeKey: Option<&'static NSString>;
}

extern "C" {
    pub static WebActionElementKey: Option<&'static NSString>;
}

extern "C" {
    pub static WebActionButtonKey: Option<&'static NSString>;
}

extern "C" {
    pub static WebActionModifierFlagsKey: Option<&'static NSString>;
}

extern "C" {
    pub static WebActionOriginalURLKey: Option<&'static NSString>;
}

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebPolicyDecisionListener: NSObjectProtocol {
        #[deprecated]
        #[method(use)]
        unsafe fn r#use(&self);

        #[deprecated]
        #[method(download)]
        unsafe fn download(&self);

        #[deprecated]
        #[method(ignore)]
        unsafe fn ignore(&self);
    }

    unsafe impl ProtocolType for dyn WebPolicyDecisionListener {}
);

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebPolicyDelegate: NSObjectProtocol {
        #[cfg(all(feature = "WebFrame", feature = "WebView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method(webView:decidePolicyForNavigationAction:request:frame:decisionListener:)]
        unsafe fn webView_decidePolicyForNavigationAction_request_frame_decisionListener(
            &self,
            web_view: Option<&WebView>,
            action_information: Option<&NSDictionary>,
            request: Option<&NSURLRequest>,
            frame: Option<&WebFrame>,
            listener: Option<&ProtocolObject<dyn WebPolicyDecisionListener>>,
        );

        #[cfg(all(feature = "WebView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method(webView:decidePolicyForNewWindowAction:request:newFrameName:decisionListener:)]
        unsafe fn webView_decidePolicyForNewWindowAction_request_newFrameName_decisionListener(
            &self,
            web_view: Option<&WebView>,
            action_information: Option<&NSDictionary>,
            request: Option<&NSURLRequest>,
            frame_name: Option<&NSString>,
            listener: Option<&ProtocolObject<dyn WebPolicyDecisionListener>>,
        );

        #[cfg(all(feature = "WebFrame", feature = "WebView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method(webView:decidePolicyForMIMEType:request:frame:decisionListener:)]
        unsafe fn webView_decidePolicyForMIMEType_request_frame_decisionListener(
            &self,
            web_view: Option<&WebView>,
            r#type: Option<&NSString>,
            request: Option<&NSURLRequest>,
            frame: Option<&WebFrame>,
            listener: Option<&ProtocolObject<dyn WebPolicyDecisionListener>>,
        );

        #[cfg(all(feature = "WebFrame", feature = "WebView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[optional]
        #[method(webView:unableToImplementPolicyWithError:frame:)]
        unsafe fn webView_unableToImplementPolicyWithError_frame(
            &self,
            web_view: Option<&WebView>,
            error: Option<&NSError>,
            frame: Option<&WebFrame>,
        );
    }

    unsafe impl ProtocolType for dyn WebPolicyDelegate {}
);
