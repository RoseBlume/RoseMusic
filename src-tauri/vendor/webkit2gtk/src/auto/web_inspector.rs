// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use crate::WebViewBase;
use glib::{
  prelude::*,
  signal::{connect_raw, SignalHandlerId},
  translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "WebKitWebInspector")]
    pub struct WebInspector(Object<ffi::WebKitWebInspector, ffi::WebKitWebInspectorClass>);

    match fn {
        type_ => || ffi::webkit_web_inspector_get_type(),
    }
}

impl WebInspector {
  pub const NONE: Option<&'static WebInspector> = None;
}

mod sealed {
  pub trait Sealed {}
  impl<T: super::IsA<super::WebInspector>> Sealed for T {}
}

pub trait WebInspectorExt: IsA<WebInspector> + sealed::Sealed + 'static {
  #[doc(alias = "webkit_web_inspector_attach")]
  fn attach(&self) {
    unsafe {
      ffi::webkit_web_inspector_attach(self.as_ref().to_glib_none().0);
    }
  }

  #[doc(alias = "webkit_web_inspector_close")]
  fn close(&self) {
    unsafe {
      ffi::webkit_web_inspector_close(self.as_ref().to_glib_none().0);
    }
  }

  #[doc(alias = "webkit_web_inspector_detach")]
  fn detach(&self) {
    unsafe {
      ffi::webkit_web_inspector_detach(self.as_ref().to_glib_none().0);
    }
  }

  #[doc(alias = "webkit_web_inspector_get_attached_height")]
  #[doc(alias = "get_attached_height")]
  fn attached_height(&self) -> u32 {
    unsafe { ffi::webkit_web_inspector_get_attached_height(self.as_ref().to_glib_none().0) }
  }

  #[cfg(feature = "v2_8")]
  #[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
  #[doc(alias = "webkit_web_inspector_get_can_attach")]
  #[doc(alias = "get_can_attach")]
  fn can_attach(&self) -> bool {
    unsafe {
      from_glib(ffi::webkit_web_inspector_get_can_attach(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  #[doc(alias = "webkit_web_inspector_get_inspected_uri")]
  #[doc(alias = "get_inspected_uri")]
  fn inspected_uri(&self) -> Option<glib::GString> {
    unsafe {
      from_glib_none(ffi::webkit_web_inspector_get_inspected_uri(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  #[doc(alias = "webkit_web_inspector_get_web_view")]
  #[doc(alias = "get_web_view")]
  fn web_view(&self) -> Option<WebViewBase> {
    unsafe {
      from_glib_none(ffi::webkit_web_inspector_get_web_view(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  #[doc(alias = "webkit_web_inspector_is_attached")]
  fn is_attached(&self) -> bool {
    unsafe {
      from_glib(ffi::webkit_web_inspector_is_attached(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  #[doc(alias = "webkit_web_inspector_show")]
  fn show(&self) {
    unsafe {
      ffi::webkit_web_inspector_show(self.as_ref().to_glib_none().0);
    }
  }

  #[doc(alias = "attach")]
  fn connect_attach<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
    unsafe extern "C" fn attach_trampoline<P: IsA<WebInspector>, F: Fn(&P) -> bool + 'static>(
      this: *mut ffi::WebKitWebInspector,
      f: glib::ffi::gpointer,
    ) -> glib::ffi::gboolean {
      let f: &F = &*(f as *const F);
      f(WebInspector::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
    }
    unsafe {
      let f: Box_<F> = Box_::new(f);
      connect_raw(
        self.as_ptr() as *mut _,
        b"attach\0".as_ptr() as *const _,
        Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
          attach_trampoline::<Self, F> as *const (),
        )),
        Box_::into_raw(f),
      )
    }
  }

  #[doc(alias = "bring-to-front")]
  fn connect_bring_to_front<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
    unsafe extern "C" fn bring_to_front_trampoline<
      P: IsA<WebInspector>,
      F: Fn(&P) -> bool + 'static,
    >(
      this: *mut ffi::WebKitWebInspector,
      f: glib::ffi::gpointer,
    ) -> glib::ffi::gboolean {
      let f: &F = &*(f as *const F);
      f(WebInspector::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
    }
    unsafe {
      let f: Box_<F> = Box_::new(f);
      connect_raw(
        self.as_ptr() as *mut _,
        b"bring-to-front\0".as_ptr() as *const _,
        Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
          bring_to_front_trampoline::<Self, F> as *const (),
        )),
        Box_::into_raw(f),
      )
    }
  }

  #[doc(alias = "closed")]
  fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
    unsafe extern "C" fn closed_trampoline<P: IsA<WebInspector>, F: Fn(&P) + 'static>(
      this: *mut ffi::WebKitWebInspector,
      f: glib::ffi::gpointer,
    ) {
      let f: &F = &*(f as *const F);
      f(WebInspector::from_glib_borrow(this).unsafe_cast_ref())
    }
    unsafe {
      let f: Box_<F> = Box_::new(f);
      connect_raw(
        self.as_ptr() as *mut _,
        b"closed\0".as_ptr() as *const _,
        Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
          closed_trampoline::<Self, F> as *const (),
        )),
        Box_::into_raw(f),
      )
    }
  }

  #[doc(alias = "detach")]
  fn connect_detach<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
    unsafe extern "C" fn detach_trampoline<P: IsA<WebInspector>, F: Fn(&P) -> bool + 'static>(
      this: *mut ffi::WebKitWebInspector,
      f: glib::ffi::gpointer,
    ) -> glib::ffi::gboolean {
      let f: &F = &*(f as *const F);
      f(WebInspector::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
    }
    unsafe {
      let f: Box_<F> = Box_::new(f);
      connect_raw(
        self.as_ptr() as *mut _,
        b"detach\0".as_ptr() as *const _,
        Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
          detach_trampoline::<Self, F> as *const (),
        )),
        Box_::into_raw(f),
      )
    }
  }

  #[doc(alias = "open-window")]
  fn connect_open_window<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
    unsafe extern "C" fn open_window_trampoline<
      P: IsA<WebInspector>,
      F: Fn(&P) -> bool + 'static,
    >(
      this: *mut ffi::WebKitWebInspector,
      f: glib::ffi::gpointer,
    ) -> glib::ffi::gboolean {
      let f: &F = &*(f as *const F);
      f(WebInspector::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
    }
    unsafe {
      let f: Box_<F> = Box_::new(f);
      connect_raw(
        self.as_ptr() as *mut _,
        b"open-window\0".as_ptr() as *const _,
        Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
          open_window_trampoline::<Self, F> as *const (),
        )),
        Box_::into_raw(f),
      )
    }
  }

  #[doc(alias = "attached-height")]
  fn connect_attached_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
    unsafe extern "C" fn notify_attached_height_trampoline<
      P: IsA<WebInspector>,
      F: Fn(&P) + 'static,
    >(
      this: *mut ffi::WebKitWebInspector,
      _param_spec: glib::ffi::gpointer,
      f: glib::ffi::gpointer,
    ) {
      let f: &F = &*(f as *const F);
      f(WebInspector::from_glib_borrow(this).unsafe_cast_ref())
    }
    unsafe {
      let f: Box_<F> = Box_::new(f);
      connect_raw(
        self.as_ptr() as *mut _,
        b"notify::attached-height\0".as_ptr() as *const _,
        Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
          notify_attached_height_trampoline::<Self, F> as *const (),
        )),
        Box_::into_raw(f),
      )
    }
  }

  #[cfg(feature = "v2_8")]
  #[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
  #[doc(alias = "can-attach")]
  fn connect_can_attach_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
    unsafe extern "C" fn notify_can_attach_trampoline<P: IsA<WebInspector>, F: Fn(&P) + 'static>(
      this: *mut ffi::WebKitWebInspector,
      _param_spec: glib::ffi::gpointer,
      f: glib::ffi::gpointer,
    ) {
      let f: &F = &*(f as *const F);
      f(WebInspector::from_glib_borrow(this).unsafe_cast_ref())
    }
    unsafe {
      let f: Box_<F> = Box_::new(f);
      connect_raw(
        self.as_ptr() as *mut _,
        b"notify::can-attach\0".as_ptr() as *const _,
        Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
          notify_can_attach_trampoline::<Self, F> as *const (),
        )),
        Box_::into_raw(f),
      )
    }
  }

  #[doc(alias = "inspected-uri")]
  fn connect_inspected_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
    unsafe extern "C" fn notify_inspected_uri_trampoline<
      P: IsA<WebInspector>,
      F: Fn(&P) + 'static,
    >(
      this: *mut ffi::WebKitWebInspector,
      _param_spec: glib::ffi::gpointer,
      f: glib::ffi::gpointer,
    ) {
      let f: &F = &*(f as *const F);
      f(WebInspector::from_glib_borrow(this).unsafe_cast_ref())
    }
    unsafe {
      let f: Box_<F> = Box_::new(f);
      connect_raw(
        self.as_ptr() as *mut _,
        b"notify::inspected-uri\0".as_ptr() as *const _,
        Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
          notify_inspected_uri_trampoline::<Self, F> as *const (),
        )),
        Box_::into_raw(f),
      )
    }
  }
}

impl<O: IsA<WebInspector>> WebInspectorExt for O {}