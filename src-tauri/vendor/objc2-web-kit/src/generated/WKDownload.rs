//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKDownload;

    unsafe impl ClassType for WKDownload {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for WKDownload {}

unsafe impl NSProgressReporting for WKDownload {}

extern_methods!(
    unsafe impl WKDownload {
        #[method_id(@__retain_semantics Other originalRequest)]
        pub unsafe fn originalRequest(&self) -> Option<Retained<NSURLRequest>>;

        #[cfg(all(feature = "WKWebView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other webView)]
        pub unsafe fn webView(&self, mtm: MainThreadMarker) -> Option<Retained<WKWebView>>;

        #[cfg(feature = "WKDownloadDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn WKDownloadDelegate>>>;

        #[cfg(feature = "WKDownloadDelegate")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn WKDownloadDelegate>>);

        #[cfg(feature = "block2")]
        #[method(cancel:)]
        pub unsafe fn cancel(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSData)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKDownload {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
