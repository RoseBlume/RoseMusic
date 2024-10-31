// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{CookieJar, SessionFeature};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "SoupCookieJarText")]
    pub struct CookieJarText(Object<ffi::SoupCookieJarText, ffi::SoupCookieJarTextClass>) @extends CookieJar, @implements SessionFeature;

    match fn {
        type_ => || ffi::soup_cookie_jar_text_get_type(),
    }
}

impl CookieJarText {
    #[doc(alias = "soup_cookie_jar_text_new")]
    pub fn new(filename: &str, read_only: bool) -> CookieJarText {
        assert_initialized_main_thread!();
        unsafe {
            CookieJar::from_glib_full(ffi::soup_cookie_jar_text_new(
                filename.to_glib_none().0,
                read_only.into_glib(),
            ))
            .unsafe_cast()
        }
    }

    pub fn filename(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "filename")
    }
}

impl fmt::Display for CookieJarText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CookieJarText")
    }
}
