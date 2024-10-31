// Take a look at the license at the top of the repository in the LICENSE file.

use std::cell::Cell;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use crate::HasParamSpec;
use crate::IsA;
use crate::Object;
use crate::SendWeakRef;
use crate::WeakRef;

// rustdoc-stripper-ignore-next
/// A type that can be used as a property. It covers every type which have an associated `ParamSpec`
/// (`HasParamSpec`) and some useful types wrapping `HasParamSpec`.
/// The definition is recursive, so you can nest many `Property`s together. The final `ParamSpec` will
/// be the one of the innermost type
pub trait Property {
    type Value: HasParamSpec;
}
impl<T: HasParamSpec> Property for T {
    type Value = T;
}
impl<T: Property> Property for PhantomData<T> {
    type Value = T::Value;
}
impl<T: Property> Property for RefCell<T> {
    type Value = T::Value;
}
impl<T: Property> Property for Cell<T> {
    type Value = T::Value;
}
impl<T: Property> Property for Mutex<T> {
    type Value = T::Value;
}
impl<T: Property> Property for RwLock<T> {
    type Value = T::Value;
}
impl<T: Property> Property for once_cell::sync::OnceCell<T> {
    type Value = T::Value;
}
impl<T: Property> Property for once_cell::unsync::OnceCell<T> {
    type Value = T::Value;
}
impl<T: Property> Property for std::cell::OnceCell<T> {
    type Value = T::Value;
}
impl<T: Property> Property for std::sync::OnceLock<T> {
    type Value = T::Value;
}
// Handle smart pointers trasparently
impl<T: Property> Property for Rc<T> {
    type Value = T::Value;
}
impl<T: Property> Property for Arc<T> {
    type Value = T::Value;
}
impl<T: IsA<Object> + HasParamSpec> Property for WeakRef<T> {
    type Value = Option<T>;
}
impl<T: IsA<Object> + HasParamSpec> Property for SendWeakRef<T> {
    type Value = Option<T>;
}

// rustdoc-stripper-ignore-next
/// A container type implementing this trait can be read by the default getter generated by the `Props` macro.
pub trait PropertyGet {
    type Value;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R;
}

// rustdoc-stripper-ignore-next
/// A container type implementing this trait can be written by the default setter generated by the `Props` macro.
/// It takes a `FnOnce(&mut Self::Value)` so that the caller may access nested fields of a struct
/// by doing `${Self::Value}.member`
pub trait PropertySetNested {
    type SetNestedValue;
    fn set_nested<F: FnOnce(&mut Self::SetNestedValue)>(&self, f: F);
}

// rustdoc-stripper-ignore-next
/// A container type implementing this trait can be written by the default setter generated by the `Props` macro.
pub trait PropertySet {
    type SetValue;
    fn set(&self, v: Self::SetValue);
}
impl<T: PropertySetNested> PropertySet for T {
    type SetValue = T::SetNestedValue;
    fn set(&self, v: Self::SetValue) {
        self.set_nested(|x| *x = v);
    }
}

impl<T: HasParamSpec> PropertyGet for T {
    type Value = T;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        f(self)
    }
}

impl<T: Copy> PropertyGet for Cell<T> {
    type Value = T;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        f(&Cell::get(self))
    }
}
impl<T> PropertySet for Cell<T> {
    type SetValue = T;
    fn set(&self, v: Self::SetValue) {
        self.set(v);
    }
}
impl<T> PropertyGet for RefCell<T> {
    type Value = T;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        f(&self.borrow())
    }
}
impl<T> PropertySetNested for RefCell<T> {
    type SetNestedValue = T;
    fn set_nested<F: FnOnce(&mut Self::SetNestedValue)>(&self, f: F) {
        f(&mut self.borrow_mut());
    }
}

impl<T> PropertyGet for Mutex<T> {
    type Value = T;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        f(&self.lock().unwrap())
    }
}
impl<T> PropertySetNested for Mutex<T> {
    type SetNestedValue = T;
    fn set_nested<F: FnOnce(&mut Self::SetNestedValue)>(&self, f: F) {
        f(&mut self.lock().unwrap());
    }
}

impl<T> PropertyGet for RwLock<T> {
    type Value = T;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        f(&self.read().unwrap())
    }
}
impl<T> PropertySetNested for RwLock<T> {
    type SetNestedValue = T;
    fn set_nested<F: FnOnce(&mut Self::SetNestedValue)>(&self, f: F) {
        f(&mut self.write().unwrap());
    }
}

impl<T> PropertyGet for once_cell::sync::OnceCell<T> {
    type Value = T;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        f(self.get().unwrap())
    }
}
impl<T> PropertyGet for once_cell::unsync::OnceCell<T> {
    type Value = T;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        f(self.get().unwrap())
    }
}
impl<T> PropertySet for once_cell::sync::OnceCell<T> {
    type SetValue = T;
    fn set(&self, v: Self::SetValue) {
        // I can't use `unwrap` because I would have to add a `Debug` bound to _v
        if let Err(_v) = self.set(v) {
            panic!("can't set value of OnceCell multiple times")
        };
    }
}
impl<T> PropertySet for once_cell::unsync::OnceCell<T> {
    type SetValue = T;
    fn set(&self, v: Self::SetValue) {
        // I can't use `unwrap` because I would have to add a `Debug` bound to _v
        if let Err(_v) = self.set(v) {
            panic!("can't set value of OnceCell multiple times")
        };
    }
}

impl<T> PropertyGet for std::cell::OnceCell<T> {
    type Value = T;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        f(self.get().unwrap())
    }
}
impl<T> PropertyGet for std::sync::OnceLock<T> {
    type Value = T;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        f(self.get().unwrap())
    }
}
impl<T> PropertySet for std::cell::OnceCell<T> {
    type SetValue = T;
    fn set(&self, v: Self::SetValue) {
        // I can't use `unwrap` because I would have to add a `Debug` bound to _v
        if let Err(_v) = self.set(v) {
            panic!("can't set value of OnceCell multiple times")
        };
    }
}
impl<T> PropertySet for std::sync::OnceLock<T> {
    type SetValue = T;
    fn set(&self, v: Self::SetValue) {
        // I can't use `unwrap` because I would have to add a `Debug` bound to _v
        if let Err(_v) = self.set(v) {
            panic!("can't set value of OnceCell multiple times")
        };
    }
}

impl<T: IsA<Object>> PropertyGet for WeakRef<T> {
    type Value = Option<T>;

    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        f(&self.upgrade())
    }
}
impl<T: IsA<Object>> PropertySet for WeakRef<T> {
    type SetValue = Option<T>;

    fn set(&self, v: Self::SetValue) {
        self.set(v.as_ref())
    }
}
impl<T: IsA<Object>> PropertyGet for SendWeakRef<T> {
    type Value = Option<T>;

    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        f(&self.upgrade())
    }
}
impl<T: IsA<Object>> PropertySet for SendWeakRef<T> {
    type SetValue = Option<T>;

    fn set(&self, v: Self::SetValue) {
        WeakRef::set(self, v.as_ref());
    }
}

// Smart pointers wrapping a `PropertyRead`/`PropertyWrite`
impl<T: PropertyGet> PropertyGet for Rc<T> {
    type Value = T::Value;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        (**self).get(f)
    }
}
impl<T: PropertySetNested> PropertySetNested for Rc<T> {
    type SetNestedValue = T::SetNestedValue;
    fn set_nested<F: FnOnce(&mut Self::SetNestedValue)>(&self, f: F) {
        (**self).set_nested(f)
    }
}

impl<T: PropertyGet> PropertyGet for Arc<T> {
    type Value = T::Value;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        (**self).get(f)
    }
}
impl<T: PropertySetNested> PropertySetNested for Arc<T> {
    type SetNestedValue = T::SetNestedValue;
    fn set_nested<F: FnOnce(&mut Self::SetNestedValue)>(&self, f: F) {
        (**self).set_nested(f)
    }
}

macro_rules! impl_atomic {
    ($atomic:ty, $valuety:ty) => {
        impl Property for $atomic {
            type Value = $valuety;
        }
        impl PropertyGet for $atomic {
            type Value = $valuety;
            fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
                f(&self.load(Ordering::Acquire))
            }
        }
        impl PropertySet for $atomic {
            type SetValue = $valuety;
            fn set(&self, v: Self::SetValue) {
                self.store(v, Ordering::Release);
            }
        }
    };
}

impl_atomic!(std::sync::atomic::AtomicBool, bool);
impl_atomic!(std::sync::atomic::AtomicI8, i8);
impl_atomic!(std::sync::atomic::AtomicI32, i32);
#[cfg(target_has_atomic = "64")]
impl_atomic!(std::sync::atomic::AtomicI64, i64);
impl_atomic!(std::sync::atomic::AtomicU8, u8);
impl_atomic!(std::sync::atomic::AtomicU32, u32);
#[cfg(target_has_atomic = "64")]
impl_atomic!(std::sync::atomic::AtomicU64, u64);
