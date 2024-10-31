// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GUnixFDList")]
    pub struct UnixFDList(Object<ffi::GUnixFDList, ffi::GUnixFDListClass>);

    match fn {
        type_ => || ffi::g_unix_fd_list_get_type(),
    }
}

impl UnixFDList {
    pub const NONE: Option<&'static UnixFDList> = None;

    #[doc(alias = "g_unix_fd_list_new")]
    pub fn new() -> UnixFDList {
        unsafe { from_glib_full(ffi::g_unix_fd_list_new()) }
    }
}

impl Default for UnixFDList {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::UnixFDList>> Sealed for T {}
}

pub trait UnixFDListExt: IsA<UnixFDList> + sealed::Sealed + 'static {
    #[doc(alias = "g_unix_fd_list_get_length")]
    #[doc(alias = "get_length")]
    fn length(&self) -> i32 {
        unsafe { ffi::g_unix_fd_list_get_length(self.as_ref().to_glib_none().0) }
    }
}

impl<O: IsA<UnixFDList>> UnixFDListExt for O {}

impl fmt::Display for UnixFDList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UnixFDList")
    }
}
