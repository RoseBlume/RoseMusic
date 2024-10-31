// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Auth;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "SoupAuthNegotiate")]
    pub struct AuthNegotiate(Object<ffi::SoupAuthNegotiate>) @extends Auth;

    match fn {
        type_ => || ffi::soup_auth_negotiate_get_type(),
    }
}

impl AuthNegotiate {
    #[doc(alias = "soup_auth_negotiate_supported")]
    pub fn supported() -> bool {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::soup_auth_negotiate_supported()) }
    }
}

impl fmt::Display for AuthNegotiate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AuthNegotiate")
    }
}