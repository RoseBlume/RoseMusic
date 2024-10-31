// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{translate::*, MainContext};

crate::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Source(Shared<ffi::GSource>);

    match fn {
        ref => |ptr| ffi::g_source_ref(ptr),
        unref => |ptr| ffi::g_source_unref(ptr),
        type_ => || ffi::g_source_get_type(),
    }
}

impl Source {
    //#[doc(alias = "g_source_new")]
    //pub fn new(source_funcs: /*Ignored*/&mut SourceFuncs, struct_size: u32) -> Source {
    //    unsafe { TODO: call ffi:g_source_new() }
    //}

    #[doc(alias = "g_source_add_child_source")]
    pub fn add_child_source(&self, child_source: &Source) {
        unsafe {
            ffi::g_source_add_child_source(self.to_glib_none().0, child_source.to_glib_none().0);
        }
    }

    //#[doc(alias = "g_source_add_poll")]
    //pub fn add_poll(&self, fd: /*Ignored*/&mut PollFD) {
    //    unsafe { TODO: call ffi:g_source_add_poll() }
    //}

    //#[doc(alias = "g_source_add_unix_fd")]
    //pub fn add_unix_fd(&self, fd: i32, events: IOCondition) -> /*Unimplemented*/Basic: Pointer {
    //    unsafe { TODO: call ffi:g_source_add_unix_fd() }
    //}

    #[doc(alias = "g_source_destroy")]
    pub fn destroy(&self) {
        unsafe {
            ffi::g_source_destroy(self.to_glib_none().0);
        }
    }

    #[doc(alias = "g_source_get_can_recurse")]
    #[doc(alias = "get_can_recurse")]
    pub fn can_recurse(&self) -> bool {
        unsafe { from_glib(ffi::g_source_get_can_recurse(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_source_get_context")]
    #[doc(alias = "get_context")]
    pub fn context(&self) -> Option<MainContext> {
        unsafe { from_glib_none(ffi::g_source_get_context(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_source_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<crate::GString> {
        unsafe { from_glib_none(ffi::g_source_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_source_get_priority")]
    #[doc(alias = "get_priority")]
    pub fn priority(&self) -> i32 {
        unsafe { ffi::g_source_get_priority(self.to_glib_none().0) }
    }

    #[doc(alias = "g_source_get_ready_time")]
    #[doc(alias = "get_ready_time")]
    pub fn ready_time(&self) -> i64 {
        unsafe { ffi::g_source_get_ready_time(self.to_glib_none().0) }
    }

    #[doc(alias = "g_source_get_time")]
    #[doc(alias = "get_time")]
    pub fn time(&self) -> i64 {
        unsafe { ffi::g_source_get_time(self.to_glib_none().0) }
    }

    #[doc(alias = "g_source_is_destroyed")]
    pub fn is_destroyed(&self) -> bool {
        unsafe { from_glib(ffi::g_source_is_destroyed(self.to_glib_none().0)) }
    }

    //#[doc(alias = "g_source_modify_unix_fd")]
    //pub fn modify_unix_fd(&self, tag: /*Unimplemented*/Basic: Pointer, new_events: IOCondition) {
    //    unsafe { TODO: call ffi:g_source_modify_unix_fd() }
    //}

    //#[doc(alias = "g_source_query_unix_fd")]
    //pub fn query_unix_fd(&self, tag: /*Unimplemented*/Basic: Pointer) -> IOCondition {
    //    unsafe { TODO: call ffi:g_source_query_unix_fd() }
    //}

    #[doc(alias = "g_source_remove_child_source")]
    pub fn remove_child_source(&self, child_source: &Source) {
        unsafe {
            ffi::g_source_remove_child_source(self.to_glib_none().0, child_source.to_glib_none().0);
        }
    }

    //#[doc(alias = "g_source_remove_poll")]
    //pub fn remove_poll(&self, fd: /*Ignored*/&mut PollFD) {
    //    unsafe { TODO: call ffi:g_source_remove_poll() }
    //}

    //#[doc(alias = "g_source_remove_unix_fd")]
    //pub fn remove_unix_fd(&self, tag: /*Unimplemented*/Basic: Pointer) {
    //    unsafe { TODO: call ffi:g_source_remove_unix_fd() }
    //}

    //#[doc(alias = "g_source_remove_by_funcs_user_data")]
    //pub fn remove_by_funcs_user_data(funcs: /*Ignored*/&mut SourceFuncs, user_data: /*Unimplemented*/Option<Basic: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:g_source_remove_by_funcs_user_data() }
    //}

    //#[doc(alias = "g_source_remove_by_user_data")]
    //pub fn remove_by_user_data(user_data: /*Unimplemented*/Option<Basic: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:g_source_remove_by_user_data() }
    //}
}

unsafe impl Send for Source {}
unsafe impl Sync for Source {}
