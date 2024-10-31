//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNLabeledValue<ValueType: ?Sized = AnyObject> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut ValueType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<ValueType: ?Sized + Message> ClassType for CNLabeledValue<ValueType> {
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

unsafe impl<ValueType: ?Sized + NSCoding> NSCoding for CNLabeledValue<ValueType> {}

unsafe impl<ValueType: ?Sized + IsIdCloneable> NSCopying for CNLabeledValue<ValueType> {}

unsafe impl<ValueType: ?Sized> NSObjectProtocol for CNLabeledValue<ValueType> {}

unsafe impl<ValueType: ?Sized + NSSecureCoding> NSSecureCoding for CNLabeledValue<ValueType> {}

extern_methods!(
    unsafe impl<ValueType: Message> CNLabeledValue<ValueType> {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Retained<ValueType>;

        #[method_id(@__retain_semantics Other labeledValueWithLabel:value:)]
        pub unsafe fn labeledValueWithLabel_value(
            label: Option<&NSString>,
            value: &ValueType,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLabel:value:)]
        pub unsafe fn initWithLabel_value(
            this: Allocated<Self>,
            label: Option<&NSString>,
            value: &ValueType,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other labeledValueBySettingLabel:)]
        pub unsafe fn labeledValueBySettingLabel(&self, label: Option<&NSString>)
            -> Retained<Self>;

        #[method_id(@__retain_semantics Other labeledValueBySettingValue:)]
        pub unsafe fn labeledValueBySettingValue(&self, value: &ValueType) -> Retained<Self>;

        #[method_id(@__retain_semantics Other labeledValueBySettingLabel:value:)]
        pub unsafe fn labeledValueBySettingLabel_value(
            &self,
            label: Option<&NSString>,
            value: &ValueType,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other localizedStringForLabel:)]
        pub unsafe fn localizedStringForLabel(label: &NSString) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ValueType: Message> CNLabeledValue<ValueType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static CNLabelHome: &'static NSString;
}

extern "C" {
    pub static CNLabelWork: &'static NSString;
}

extern "C" {
    pub static CNLabelSchool: &'static NSString;
}

extern "C" {
    pub static CNLabelOther: &'static NSString;
}

extern "C" {
    pub static CNLabelEmailiCloud: &'static NSString;
}

extern "C" {
    pub static CNLabelURLAddressHomePage: &'static NSString;
}

extern "C" {
    pub static CNLabelDateAnniversary: &'static NSString;
}
