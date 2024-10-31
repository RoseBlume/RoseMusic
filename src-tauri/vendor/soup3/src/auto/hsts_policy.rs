// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Message;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct HSTSPolicy(Boxed<ffi::SoupHSTSPolicy>);

    match fn {
        copy => |ptr| ffi::soup_hsts_policy_copy(mut_override(ptr)),
        free => |ptr| ffi::soup_hsts_policy_free(ptr),
        type_ => || ffi::soup_hsts_policy_get_type(),
    }
}

impl HSTSPolicy {
    #[doc(alias = "soup_hsts_policy_new")]
    pub fn new(domain: &str, max_age: libc::c_ulong, include_subdomains: bool) -> HSTSPolicy {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::soup_hsts_policy_new(
                domain.to_glib_none().0,
                max_age,
                include_subdomains.into_glib(),
            ))
        }
    }

    #[doc(alias = "soup_hsts_policy_new_from_response")]
    #[doc(alias = "new_from_response")]
    pub fn from_response(msg: &Message) -> Option<HSTSPolicy> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::soup_hsts_policy_new_from_response(
                msg.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "soup_hsts_policy_new_full")]
    pub fn new_full(
        domain: &str,
        max_age: libc::c_ulong,
        expires: &glib::DateTime,
        include_subdomains: bool,
    ) -> HSTSPolicy {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::soup_hsts_policy_new_full(
                domain.to_glib_none().0,
                max_age,
                expires.to_glib_none().0,
                include_subdomains.into_glib(),
            ))
        }
    }

    #[doc(alias = "soup_hsts_policy_new_session_policy")]
    pub fn new_session_policy(domain: &str, include_subdomains: bool) -> HSTSPolicy {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::soup_hsts_policy_new_session_policy(
                domain.to_glib_none().0,
                include_subdomains.into_glib(),
            ))
        }
    }

    #[doc(alias = "soup_hsts_policy_get_domain")]
    #[doc(alias = "get_domain")]
    pub fn domain(&mut self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::soup_hsts_policy_get_domain(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "soup_hsts_policy_get_expires")]
    #[doc(alias = "get_expires")]
    pub fn expires(&mut self) -> Option<glib::DateTime> {
        unsafe { from_glib_full(ffi::soup_hsts_policy_get_expires(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "soup_hsts_policy_get_max_age")]
    #[doc(alias = "get_max_age")]
    pub fn max_age(&mut self) -> libc::c_ulong {
        unsafe { ffi::soup_hsts_policy_get_max_age(self.to_glib_none_mut().0) }
    }

    #[doc(alias = "soup_hsts_policy_includes_subdomains")]
    pub fn includes_subdomains(&mut self) -> bool {
        unsafe {
            from_glib(ffi::soup_hsts_policy_includes_subdomains(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "soup_hsts_policy_is_expired")]
    pub fn is_expired(&mut self) -> bool {
        unsafe { from_glib(ffi::soup_hsts_policy_is_expired(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "soup_hsts_policy_is_session_policy")]
    pub fn is_session_policy(&mut self) -> bool {
        unsafe {
            from_glib(ffi::soup_hsts_policy_is_session_policy(
                self.to_glib_none_mut().0,
            ))
        }
    }
}