// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkX11DeviceManagerCore")]
    pub struct X11DeviceManagerCore(Object<ffi::GdkX11DeviceManagerCore, ffi::GdkX11DeviceManagerCoreClass>) @extends gdk::DeviceManager;

    match fn {
        type_ => || ffi::gdk_x11_device_manager_core_get_type(),
    }
}

impl X11DeviceManagerCore {
    pub const NONE: Option<&'static X11DeviceManagerCore> = None;
}

impl fmt::Display for X11DeviceManagerCore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("X11DeviceManagerCore")
    }
}