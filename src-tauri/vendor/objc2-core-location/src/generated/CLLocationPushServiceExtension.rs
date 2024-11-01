//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait CLLocationPushServiceExtension: NSObjectProtocol {
        #[cfg(feature = "block2")]
        #[method(didReceiveLocationPushPayload:completion:)]
        unsafe fn didReceiveLocationPushPayload_completion(
            &self,
            payload: &NSDictionary<NSString, AnyObject>,
            completion: &block2::Block<dyn Fn()>,
        );

        #[optional]
        #[method(serviceExtensionWillTerminate)]
        unsafe fn serviceExtensionWillTerminate(&self);
    }

    unsafe impl ProtocolType for dyn CLLocationPushServiceExtension {}
);