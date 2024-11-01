//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait MTLEvent: NSObjectProtocol + IsRetainable {
        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Option<Retained<ProtocolObject<dyn MTLDevice>>>;

        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);
    }

    unsafe impl ProtocolType for dyn MTLEvent {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLSharedEventListener;

    unsafe impl ClassType for MTLSharedEventListener {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MTLSharedEventListener {}

extern_methods!(
    unsafe impl MTLSharedEventListener {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLSharedEventListener {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for MTLSharedEventListener {
    #[inline]
    fn default_id() -> Retained<Self> {
        Self::new()
    }
}

#[cfg(feature = "block2")]
pub type MTLSharedEventNotificationBlock =
    *mut block2::Block<dyn Fn(NonNull<ProtocolObject<dyn MTLSharedEvent>>, u64)>;

extern_protocol!(
    pub unsafe trait MTLSharedEvent: MTLEvent + IsRetainable {
        #[cfg(feature = "block2")]
        #[method(notifyListener:atValue:block:)]
        unsafe fn notifyListener_atValue_block(
            &self,
            listener: &MTLSharedEventListener,
            value: u64,
            block: MTLSharedEventNotificationBlock,
        );

        #[method_id(@__retain_semantics New newSharedEventHandle)]
        unsafe fn newSharedEventHandle(&self) -> Retained<MTLSharedEventHandle>;

        #[method(waitUntilSignaledValue:timeoutMS:)]
        unsafe fn waitUntilSignaledValue_timeoutMS(&self, value: u64, milliseconds: u64) -> bool;

        #[method(signaledValue)]
        unsafe fn signaledValue(&self) -> u64;

        #[method(setSignaledValue:)]
        unsafe fn setSignaledValue(&self, signaled_value: u64);
    }

    unsafe impl ProtocolType for dyn MTLSharedEvent {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLSharedEventHandle;

    unsafe impl ClassType for MTLSharedEventHandle {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for MTLSharedEventHandle {}

unsafe impl NSObjectProtocol for MTLSharedEventHandle {}

unsafe impl NSSecureCoding for MTLSharedEventHandle {}

extern_methods!(
    unsafe impl MTLSharedEventHandle {
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLSharedEventHandle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
