// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{LoggerLogLevel, Message, SessionFeature};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "SoupLogger")]
    pub struct Logger(Object<ffi::SoupLogger, ffi::SoupLoggerClass>) @implements SessionFeature;

    match fn {
        type_ => || ffi::soup_logger_get_type(),
    }
}

impl Logger {
    #[doc(alias = "soup_logger_new")]
    pub fn new(level: LoggerLogLevel) -> Logger {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::soup_logger_new(level.into_glib())) }
    }

    #[doc(alias = "soup_logger_get_max_body_size")]
    #[doc(alias = "get_max_body_size")]
    pub fn max_body_size(&self) -> i32 {
        unsafe { ffi::soup_logger_get_max_body_size(self.to_glib_none().0) }
    }

    #[doc(alias = "soup_logger_set_max_body_size")]
    pub fn set_max_body_size(&self, max_body_size: i32) {
        unsafe {
            ffi::soup_logger_set_max_body_size(self.to_glib_none().0, max_body_size);
        }
    }

    #[doc(alias = "soup_logger_set_request_filter")]
    pub fn set_request_filter<P: Fn(&Logger, &Message) -> LoggerLogLevel + 'static>(
        &self,
        request_filter: P,
    ) {
        let request_filter_data: Box_<P> = Box_::new(request_filter);
        unsafe extern "C" fn request_filter_func<
            P: Fn(&Logger, &Message) -> LoggerLogLevel + 'static,
        >(
            logger: *mut ffi::SoupLogger,
            msg: *mut ffi::SoupMessage,
            user_data: glib::ffi::gpointer,
        ) -> ffi::SoupLoggerLogLevel {
            let logger = from_glib_borrow(logger);
            let msg = from_glib_borrow(msg);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&logger, &msg).into_glib()
        }
        let request_filter = Some(request_filter_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&Logger, &Message) -> LoggerLogLevel + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = request_filter_data;
        unsafe {
            ffi::soup_logger_set_request_filter(
                self.to_glib_none().0,
                request_filter,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "soup_logger_set_response_filter")]
    pub fn set_response_filter<P: Fn(&Logger, &Message) -> LoggerLogLevel + 'static>(
        &self,
        response_filter: P,
    ) {
        let response_filter_data: Box_<P> = Box_::new(response_filter);
        unsafe extern "C" fn response_filter_func<
            P: Fn(&Logger, &Message) -> LoggerLogLevel + 'static,
        >(
            logger: *mut ffi::SoupLogger,
            msg: *mut ffi::SoupMessage,
            user_data: glib::ffi::gpointer,
        ) -> ffi::SoupLoggerLogLevel {
            let logger = from_glib_borrow(logger);
            let msg = from_glib_borrow(msg);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&logger, &msg).into_glib()
        }
        let response_filter = Some(response_filter_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&Logger, &Message) -> LoggerLogLevel + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = response_filter_data;
        unsafe {
            ffi::soup_logger_set_response_filter(
                self.to_glib_none().0,
                response_filter,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    pub fn level(&self) -> LoggerLogLevel {
        ObjectExt::property(self, "level")
    }

    pub fn set_level(&self, level: LoggerLogLevel) {
        ObjectExt::set_property(self, "level", level)
    }

    #[doc(alias = "level")]
    pub fn connect_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_level_trampoline<F: Fn(&Logger) + 'static>(
            this: *mut ffi::SoupLogger,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::level\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_level_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-body-size")]
    pub fn connect_max_body_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_body_size_trampoline<F: Fn(&Logger) + 'static>(
            this: *mut ffi::SoupLogger,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-body-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_body_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Logger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Logger")
    }
}
