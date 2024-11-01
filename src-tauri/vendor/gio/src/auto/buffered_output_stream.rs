// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{FilterOutputStream, OutputStream, Seekable};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GBufferedOutputStream")]
    pub struct BufferedOutputStream(Object<ffi::GBufferedOutputStream, ffi::GBufferedOutputStreamClass>) @extends FilterOutputStream, OutputStream, @implements Seekable;

    match fn {
        type_ => || ffi::g_buffered_output_stream_get_type(),
    }
}

impl BufferedOutputStream {
    pub const NONE: Option<&'static BufferedOutputStream> = None;

    #[doc(alias = "g_buffered_output_stream_new")]
    pub fn new(base_stream: &impl IsA<OutputStream>) -> BufferedOutputStream {
        unsafe {
            OutputStream::from_glib_full(ffi::g_buffered_output_stream_new(
                base_stream.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "g_buffered_output_stream_new_sized")]
    pub fn new_sized(base_stream: &impl IsA<OutputStream>, size: usize) -> BufferedOutputStream {
        unsafe {
            OutputStream::from_glib_full(ffi::g_buffered_output_stream_new_sized(
                base_stream.as_ref().to_glib_none().0,
                size,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`BufferedOutputStream`] objects.
    ///
    /// This method returns an instance of [`BufferedOutputStreamBuilder`](crate::builders::BufferedOutputStreamBuilder) which can be used to create [`BufferedOutputStream`] objects.
    pub fn builder() -> BufferedOutputStreamBuilder {
        BufferedOutputStreamBuilder::new()
    }
}

impl Default for BufferedOutputStream {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`BufferedOutputStream`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BufferedOutputStreamBuilder {
    builder: glib::object::ObjectBuilder<'static, BufferedOutputStream>,
}

impl BufferedOutputStreamBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn auto_grow(self, auto_grow: bool) -> Self {
        Self {
            builder: self.builder.property("auto-grow", auto_grow),
        }
    }

    pub fn buffer_size(self, buffer_size: u32) -> Self {
        Self {
            builder: self.builder.property("buffer-size", buffer_size),
        }
    }

    pub fn base_stream(self, base_stream: &impl IsA<OutputStream>) -> Self {
        Self {
            builder: self
                .builder
                .property("base-stream", base_stream.clone().upcast()),
        }
    }

    pub fn close_base_stream(self, close_base_stream: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("close-base-stream", close_base_stream),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`BufferedOutputStream`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> BufferedOutputStream {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::BufferedOutputStream>> Sealed for T {}
}

pub trait BufferedOutputStreamExt: IsA<BufferedOutputStream> + sealed::Sealed + 'static {
    #[doc(alias = "g_buffered_output_stream_get_auto_grow")]
    #[doc(alias = "get_auto_grow")]
    fn auto_grows(&self) -> bool {
        unsafe {
            from_glib(ffi::g_buffered_output_stream_get_auto_grow(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_buffered_output_stream_get_buffer_size")]
    #[doc(alias = "get_buffer_size")]
    fn buffer_size(&self) -> usize {
        unsafe { ffi::g_buffered_output_stream_get_buffer_size(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "g_buffered_output_stream_set_auto_grow")]
    fn set_auto_grow(&self, auto_grow: bool) {
        unsafe {
            ffi::g_buffered_output_stream_set_auto_grow(
                self.as_ref().to_glib_none().0,
                auto_grow.into_glib(),
            );
        }
    }

    #[doc(alias = "g_buffered_output_stream_set_buffer_size")]
    fn set_buffer_size(&self, size: usize) {
        unsafe {
            ffi::g_buffered_output_stream_set_buffer_size(self.as_ref().to_glib_none().0, size);
        }
    }

    #[doc(alias = "auto-grow")]
    fn connect_auto_grow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_grow_trampoline<
            P: IsA<BufferedOutputStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GBufferedOutputStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BufferedOutputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::auto-grow\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_auto_grow_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "buffer-size")]
    fn connect_buffer_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_size_trampoline<
            P: IsA<BufferedOutputStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GBufferedOutputStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BufferedOutputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::buffer-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_buffer_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<BufferedOutputStream>> BufferedOutputStreamExt for O {}

impl fmt::Display for BufferedOutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BufferedOutputStream")
    }
}
