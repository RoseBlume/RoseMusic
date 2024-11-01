// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ListModel;
use glib::{prelude::*, translate::*};
use std::fmt;
#[cfg(feature = "v2_64")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_64")))]
use std::mem;

glib::wrapper! {
    #[doc(alias = "GListStore")]
    pub struct ListStore(Object<ffi::GListStore, ffi::GListStoreClass>) @implements ListModel;

    match fn {
        type_ => || ffi::g_list_store_get_type(),
    }
}

impl ListStore {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ListStore`] objects.
    ///
    /// This method returns an instance of [`ListStoreBuilder`](crate::builders::ListStoreBuilder) which can be used to create [`ListStore`] objects.
    pub fn builder() -> ListStoreBuilder {
        ListStoreBuilder::new()
    }

    #[doc(alias = "g_list_store_append")]
    pub fn append(&self, item: &impl IsA<glib::Object>) {
        unsafe {
            ffi::g_list_store_append(self.to_glib_none().0, item.as_ref().to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_64")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_64")))]
    #[doc(alias = "g_list_store_find")]
    pub fn find(&self, item: &impl IsA<glib::Object>) -> Option<u32> {
        unsafe {
            let mut position = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::g_list_store_find(
                self.to_glib_none().0,
                item.as_ref().to_glib_none().0,
                position.as_mut_ptr(),
            ));
            if ret {
                Some(position.assume_init())
            } else {
                None
            }
        }
    }

    #[doc(alias = "g_list_store_insert")]
    pub fn insert(&self, position: u32, item: &impl IsA<glib::Object>) {
        unsafe {
            ffi::g_list_store_insert(
                self.to_glib_none().0,
                position,
                item.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_list_store_remove")]
    pub fn remove(&self, position: u32) {
        unsafe {
            ffi::g_list_store_remove(self.to_glib_none().0, position);
        }
    }

    #[doc(alias = "g_list_store_remove_all")]
    pub fn remove_all(&self) {
        unsafe {
            ffi::g_list_store_remove_all(self.to_glib_none().0);
        }
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ListStore`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ListStoreBuilder {
    builder: glib::object::ObjectBuilder<'static, ListStore>,
}

impl ListStoreBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn item_type(self, item_type: glib::types::Type) -> Self {
        Self {
            builder: self.builder.property("item-type", item_type),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ListStore`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ListStore {
        self.builder.build()
    }
}

impl fmt::Display for ListStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ListStore")
    }
}
