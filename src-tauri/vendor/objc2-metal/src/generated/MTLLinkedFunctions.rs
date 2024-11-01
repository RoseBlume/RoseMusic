//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLLinkedFunctions;

    unsafe impl ClassType for MTLLinkedFunctions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for MTLLinkedFunctions {}

unsafe impl NSObjectProtocol for MTLLinkedFunctions {}

extern_methods!(
    unsafe impl MTLLinkedFunctions {
        #[method_id(@__retain_semantics Other linkedFunctions)]
        pub fn linkedFunctions() -> Retained<MTLLinkedFunctions>;

        #[cfg(feature = "MTLLibrary")]
        #[method_id(@__retain_semantics Other functions)]
        pub fn functions(&self) -> Option<Retained<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        #[cfg(feature = "MTLLibrary")]
        #[method(setFunctions:)]
        pub fn setFunctions(&self, functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>);

        #[cfg(feature = "MTLLibrary")]
        #[method_id(@__retain_semantics Other binaryFunctions)]
        pub fn binaryFunctions(&self)
            -> Option<Retained<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        #[cfg(feature = "MTLLibrary")]
        #[method(setBinaryFunctions:)]
        pub fn setBinaryFunctions(
            &self,
            binary_functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>,
        );

        #[cfg(feature = "MTLLibrary")]
        #[method_id(@__retain_semantics Other groups)]
        pub fn groups(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, NSArray<ProtocolObject<dyn MTLFunction>>>>>;

        #[cfg(feature = "MTLLibrary")]
        #[method(setGroups:)]
        pub fn setGroups(
            &self,
            groups: Option<&NSDictionary<NSString, NSArray<ProtocolObject<dyn MTLFunction>>>>,
        );

        #[cfg(feature = "MTLLibrary")]
        #[method_id(@__retain_semantics Other privateFunctions)]
        pub fn privateFunctions(
            &self,
        ) -> Option<Retained<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        #[cfg(feature = "MTLLibrary")]
        #[method(setPrivateFunctions:)]
        pub fn setPrivateFunctions(
            &self,
            private_functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLLinkedFunctions {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for MTLLinkedFunctions {
    #[inline]
    fn default_id() -> Retained<Self> {
        Self::new()
    }
}