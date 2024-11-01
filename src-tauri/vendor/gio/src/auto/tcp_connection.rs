// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{IOStream, SocketConnection};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GTcpConnection")]
    pub struct TcpConnection(Object<ffi::GTcpConnection, ffi::GTcpConnectionClass>) @extends SocketConnection, IOStream;

    match fn {
        type_ => || ffi::g_tcp_connection_get_type(),
    }
}

impl TcpConnection {
    pub const NONE: Option<&'static TcpConnection> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::TcpConnection>> Sealed for T {}
}

pub trait TcpConnectionExt: IsA<TcpConnection> + sealed::Sealed + 'static {
    #[doc(alias = "g_tcp_connection_get_graceful_disconnect")]
    #[doc(alias = "get_graceful_disconnect")]
    fn is_graceful_disconnect(&self) -> bool {
        unsafe {
            from_glib(ffi::g_tcp_connection_get_graceful_disconnect(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_tcp_connection_set_graceful_disconnect")]
    fn set_graceful_disconnect(&self, graceful_disconnect: bool) {
        unsafe {
            ffi::g_tcp_connection_set_graceful_disconnect(
                self.as_ref().to_glib_none().0,
                graceful_disconnect.into_glib(),
            );
        }
    }

    #[doc(alias = "graceful-disconnect")]
    fn connect_graceful_disconnect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_graceful_disconnect_trampoline<
            P: IsA<TcpConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTcpConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TcpConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::graceful-disconnect\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_graceful_disconnect_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<TcpConnection>> TcpConnectionExt for O {}

impl fmt::Display for TcpConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TcpConnection")
    }
}
