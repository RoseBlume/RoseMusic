// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{prelude::*, translate::*, Object};
use std::fmt;

crate::wrapper! {
    #[doc(alias = "GSignalGroup")]
    pub struct SignalGroup(Object<gobject_ffi::GSignalGroup>);

    match fn {
        type_ => || gobject_ffi::g_signal_group_get_type(),
    }
}

impl SignalGroup {
    #[doc(alias = "g_signal_group_new")]
    #[doc(alias = "new")]
    pub fn with_type(target_type: crate::types::Type) -> SignalGroup {
        unsafe { from_glib_full(gobject_ffi::g_signal_group_new(target_type.into_glib())) }
    }

    #[doc(alias = "g_signal_group_block")]
    pub fn block(&self) {
        unsafe {
            gobject_ffi::g_signal_group_block(self.to_glib_none().0);
        }
    }

    #[doc(alias = "g_signal_group_dup_target")]
    #[doc(alias = "dup_target")]
    pub fn target(&self) -> Option<Object> {
        unsafe {
            from_glib_full(gobject_ffi::g_signal_group_dup_target(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_signal_group_set_target")]
    pub fn set_target(&self, target: Option<&impl IsA<Object>>) {
        unsafe {
            gobject_ffi::g_signal_group_set_target(
                self.to_glib_none().0,
                target.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_signal_group_unblock")]
    pub fn unblock(&self) {
        unsafe {
            gobject_ffi::g_signal_group_unblock(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_72")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
    #[doc(alias = "target-type")]
    pub fn target_type(&self) -> crate::types::Type {
        ObjectExt::property(self, "target-type")
    }
}

unsafe impl Send for SignalGroup {}
unsafe impl Sync for SignalGroup {}

impl fmt::Display for SignalGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SignalGroup")
    }
}