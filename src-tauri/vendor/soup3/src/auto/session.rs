// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Message, SessionFeature, WebsocketConnection};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "SoupSession")]
    pub struct Session(Object<ffi::SoupSession, ffi::SoupSessionClass>);

    match fn {
        type_ => || ffi::soup_session_get_type(),
    }
}

impl Session {
    pub const NONE: Option<&'static Session> = None;

    #[doc(alias = "soup_session_new")]
    pub fn new() -> Session {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::soup_session_new()) }
    }

    //#[doc(alias = "soup_session_new_with_options")]
    //#[doc(alias = "new_with_options")]
    //pub fn with_options(optname1: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Session {
    //    unsafe { TODO: call ffi:soup_session_new_with_options() }
    //}

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Session`] objects.
    ///
    /// This method returns an instance of [`SessionBuilder`](crate::builders::SessionBuilder) which can be used to create [`Session`] objects.
    pub fn builder() -> SessionBuilder {
        SessionBuilder::new()
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Session`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SessionBuilder {
    builder: glib::object::ObjectBuilder<'static, Session>,
}

impl SessionBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn accept_language(self, accept_language: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("accept-language", accept_language.into()),
        }
    }

    pub fn accept_language_auto(self, accept_language_auto: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("accept-language-auto", accept_language_auto),
        }
    }

    pub fn idle_timeout(self, idle_timeout: u32) -> Self {
        Self {
            builder: self.builder.property("idle-timeout", idle_timeout),
        }
    }

    pub fn local_address(self, local_address: &impl IsA<gio::InetSocketAddress>) -> Self {
        Self {
            builder: self
                .builder
                .property("local-address", local_address.clone().upcast()),
        }
    }

    pub fn max_conns(self, max_conns: i32) -> Self {
        Self {
            builder: self.builder.property("max-conns", max_conns),
        }
    }

    pub fn max_conns_per_host(self, max_conns_per_host: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("max-conns-per-host", max_conns_per_host),
        }
    }

    pub fn proxy_resolver(self, proxy_resolver: &impl IsA<gio::ProxyResolver>) -> Self {
        Self {
            builder: self
                .builder
                .property("proxy-resolver", proxy_resolver.clone().upcast()),
        }
    }

    pub fn remote_connectable(self, remote_connectable: &impl IsA<gio::SocketConnectable>) -> Self {
        Self {
            builder: self
                .builder
                .property("remote-connectable", remote_connectable.clone().upcast()),
        }
    }

    pub fn timeout(self, timeout: u32) -> Self {
        Self {
            builder: self.builder.property("timeout", timeout),
        }
    }

    pub fn tls_database(self, tls_database: &impl IsA<gio::TlsDatabase>) -> Self {
        Self {
            builder: self
                .builder
                .property("tls-database", tls_database.clone().upcast()),
        }
    }

    pub fn tls_interaction(self, tls_interaction: &impl IsA<gio::TlsInteraction>) -> Self {
        Self {
            builder: self
                .builder
                .property("tls-interaction", tls_interaction.clone().upcast()),
        }
    }

    pub fn user_agent(self, user_agent: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("user-agent", user_agent.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Session`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Session {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Session>> Sealed for T {}
}

pub trait SessionExt: IsA<Session> + sealed::Sealed + 'static {
    #[doc(alias = "soup_session_abort")]
    fn abort(&self) {
        unsafe {
            ffi::soup_session_abort(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "soup_session_add_feature")]
    fn add_feature(&self, feature: &impl IsA<SessionFeature>) {
        unsafe {
            ffi::soup_session_add_feature(
                self.as_ref().to_glib_none().0,
                feature.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "soup_session_add_feature_by_type")]
    fn add_feature_by_type(&self, feature_type: glib::types::Type) {
        unsafe {
            ffi::soup_session_add_feature_by_type(
                self.as_ref().to_glib_none().0,
                feature_type.into_glib(),
            );
        }
    }

    #[doc(alias = "soup_session_get_accept_language")]
    #[doc(alias = "get_accept_language")]
    fn accept_language(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::soup_session_get_accept_language(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "soup_session_get_accept_language_auto")]
    #[doc(alias = "get_accept_language_auto")]
    fn accepts_language_auto(&self) -> bool {
        unsafe {
            from_glib(ffi::soup_session_get_accept_language_auto(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "soup_session_get_async_result_message")]
    #[doc(alias = "get_async_result_message")]
    fn async_result_message(&self, result: &impl IsA<gio::AsyncResult>) -> Option<Message> {
        unsafe {
            from_glib_none(ffi::soup_session_get_async_result_message(
                self.as_ref().to_glib_none().0,
                result.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "soup_session_get_feature")]
    #[doc(alias = "get_feature")]
    fn feature(&self, feature_type: glib::types::Type) -> Option<SessionFeature> {
        unsafe {
            from_glib_none(ffi::soup_session_get_feature(
                self.as_ref().to_glib_none().0,
                feature_type.into_glib(),
            ))
        }
    }

    #[doc(alias = "soup_session_get_feature_for_message")]
    #[doc(alias = "get_feature_for_message")]
    fn feature_for_message(
        &self,
        feature_type: glib::types::Type,
        msg: &Message,
    ) -> Option<SessionFeature> {
        unsafe {
            from_glib_none(ffi::soup_session_get_feature_for_message(
                self.as_ref().to_glib_none().0,
                feature_type.into_glib(),
                msg.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "soup_session_get_idle_timeout")]
    #[doc(alias = "get_idle_timeout")]
    fn idle_timeout(&self) -> u32 {
        unsafe { ffi::soup_session_get_idle_timeout(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "soup_session_get_local_address")]
    #[doc(alias = "get_local_address")]
    fn local_address(&self) -> Option<gio::InetSocketAddress> {
        unsafe {
            from_glib_none(ffi::soup_session_get_local_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "soup_session_get_max_conns")]
    #[doc(alias = "get_max_conns")]
    fn max_conns(&self) -> u32 {
        unsafe { ffi::soup_session_get_max_conns(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "soup_session_get_max_conns_per_host")]
    #[doc(alias = "get_max_conns_per_host")]
    fn max_conns_per_host(&self) -> u32 {
        unsafe { ffi::soup_session_get_max_conns_per_host(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "soup_session_get_proxy_resolver")]
    #[doc(alias = "get_proxy_resolver")]
    fn proxy_resolver(&self) -> Option<gio::ProxyResolver> {
        unsafe {
            from_glib_none(ffi::soup_session_get_proxy_resolver(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "soup_session_get_remote_connectable")]
    #[doc(alias = "get_remote_connectable")]
    fn remote_connectable(&self) -> Option<gio::SocketConnectable> {
        unsafe {
            from_glib_none(ffi::soup_session_get_remote_connectable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "soup_session_get_timeout")]
    #[doc(alias = "get_timeout")]
    fn timeout(&self) -> u32 {
        unsafe { ffi::soup_session_get_timeout(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "soup_session_get_tls_database")]
    #[doc(alias = "get_tls_database")]
    fn tls_database(&self) -> Option<gio::TlsDatabase> {
        unsafe {
            from_glib_none(ffi::soup_session_get_tls_database(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "soup_session_get_tls_interaction")]
    #[doc(alias = "get_tls_interaction")]
    fn tls_interaction(&self) -> Option<gio::TlsInteraction> {
        unsafe {
            from_glib_none(ffi::soup_session_get_tls_interaction(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "soup_session_get_user_agent")]
    #[doc(alias = "get_user_agent")]
    fn user_agent(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::soup_session_get_user_agent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "soup_session_has_feature")]
    fn has_feature(&self, feature_type: glib::types::Type) -> bool {
        unsafe {
            from_glib(ffi::soup_session_has_feature(
                self.as_ref().to_glib_none().0,
                feature_type.into_glib(),
            ))
        }
    }

    #[doc(alias = "soup_session_preconnect_async")]
    fn preconnect_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        msg: &Message,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn preconnect_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::soup_session_preconnect_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = preconnect_async_trampoline::<P>;
        unsafe {
            ffi::soup_session_preconnect_async(
                self.as_ref().to_glib_none().0,
                msg.to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn preconnect_future(
        &self,
        msg: &Message,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let msg = msg.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.preconnect_async(&msg, io_priority, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "soup_session_remove_feature")]
    fn remove_feature(&self, feature: &impl IsA<SessionFeature>) {
        unsafe {
            ffi::soup_session_remove_feature(
                self.as_ref().to_glib_none().0,
                feature.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "soup_session_remove_feature_by_type")]
    fn remove_feature_by_type(&self, feature_type: glib::types::Type) {
        unsafe {
            ffi::soup_session_remove_feature_by_type(
                self.as_ref().to_glib_none().0,
                feature_type.into_glib(),
            );
        }
    }

    #[doc(alias = "soup_session_send")]
    fn send(
        &self,
        msg: &Message,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<gio::InputStream, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::soup_session_send(
                self.as_ref().to_glib_none().0,
                msg.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "soup_session_send_and_read")]
    fn send_and_read(
        &self,
        msg: &Message,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<glib::Bytes, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::soup_session_send_and_read(
                self.as_ref().to_glib_none().0,
                msg.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "soup_session_send_and_read_async")]
    fn send_and_read_async<P: FnOnce(Result<glib::Bytes, glib::Error>) + 'static>(
        &self,
        msg: &Message,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn send_and_read_async_trampoline<
            P: FnOnce(Result<glib::Bytes, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::soup_session_send_and_read_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = send_and_read_async_trampoline::<P>;
        unsafe {
            ffi::soup_session_send_and_read_async(
                self.as_ref().to_glib_none().0,
                msg.to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn send_and_read_future(
        &self,
        msg: &Message,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Bytes, glib::Error>> + 'static>>
    {
        let msg = msg.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.send_and_read_async(&msg, io_priority, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[cfg(feature = "v3_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_4")))]
    #[doc(alias = "soup_session_send_and_splice")]
    fn send_and_splice(
        &self,
        msg: &Message,
        out_stream: &impl IsA<gio::OutputStream>,
        flags: gio::OutputStreamSpliceFlags,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<isize, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::soup_session_send_and_splice(
                self.as_ref().to_glib_none().0,
                msg.to_glib_none().0,
                out_stream.as_ref().to_glib_none().0,
                flags.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v3_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_4")))]
    #[doc(alias = "soup_session_send_and_splice_async")]
    fn send_and_splice_async<P: FnOnce(Result<isize, glib::Error>) + 'static>(
        &self,
        msg: &Message,
        out_stream: &impl IsA<gio::OutputStream>,
        flags: gio::OutputStreamSpliceFlags,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn send_and_splice_async_trampoline<
            P: FnOnce(Result<isize, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::soup_session_send_and_splice_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = send_and_splice_async_trampoline::<P>;
        unsafe {
            ffi::soup_session_send_and_splice_async(
                self.as_ref().to_glib_none().0,
                msg.to_glib_none().0,
                out_stream.as_ref().to_glib_none().0,
                flags.into_glib(),
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "v3_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_4")))]
    fn send_and_splice_future(
        &self,
        msg: &Message,
        out_stream: &(impl IsA<gio::OutputStream> + Clone + 'static),
        flags: gio::OutputStreamSpliceFlags,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>> {
        let msg = msg.clone();
        let out_stream = out_stream.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.send_and_splice_async(
                &msg,
                &out_stream,
                flags,
                io_priority,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "soup_session_send_async")]
    fn send_async<P: FnOnce(Result<gio::InputStream, glib::Error>) + 'static>(
        &self,
        msg: &Message,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn send_async_trampoline<
            P: FnOnce(Result<gio::InputStream, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::soup_session_send_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = send_async_trampoline::<P>;
        unsafe {
            ffi::soup_session_send_async(
                self.as_ref().to_glib_none().0,
                msg.to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn send_future(
        &self,
        msg: &Message,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<gio::InputStream, glib::Error>> + 'static>>
    {
        let msg = msg.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.send_async(&msg, io_priority, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "soup_session_set_accept_language")]
    fn set_accept_language(&self, accept_language: &str) {
        unsafe {
            ffi::soup_session_set_accept_language(
                self.as_ref().to_glib_none().0,
                accept_language.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "soup_session_set_accept_language_auto")]
    fn set_accept_language_auto(&self, accept_language_auto: bool) {
        unsafe {
            ffi::soup_session_set_accept_language_auto(
                self.as_ref().to_glib_none().0,
                accept_language_auto.into_glib(),
            );
        }
    }

    #[doc(alias = "soup_session_set_idle_timeout")]
    fn set_idle_timeout(&self, timeout: u32) {
        unsafe {
            ffi::soup_session_set_idle_timeout(self.as_ref().to_glib_none().0, timeout);
        }
    }

    #[doc(alias = "soup_session_set_proxy_resolver")]
    fn set_proxy_resolver(&self, proxy_resolver: Option<&impl IsA<gio::ProxyResolver>>) {
        unsafe {
            ffi::soup_session_set_proxy_resolver(
                self.as_ref().to_glib_none().0,
                proxy_resolver.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "soup_session_set_timeout")]
    fn set_timeout(&self, timeout: u32) {
        unsafe {
            ffi::soup_session_set_timeout(self.as_ref().to_glib_none().0, timeout);
        }
    }

    #[doc(alias = "soup_session_set_tls_database")]
    fn set_tls_database(&self, tls_database: Option<&impl IsA<gio::TlsDatabase>>) {
        unsafe {
            ffi::soup_session_set_tls_database(
                self.as_ref().to_glib_none().0,
                tls_database.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "soup_session_set_tls_interaction")]
    fn set_tls_interaction(&self, tls_interaction: Option<&impl IsA<gio::TlsInteraction>>) {
        unsafe {
            ffi::soup_session_set_tls_interaction(
                self.as_ref().to_glib_none().0,
                tls_interaction.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "soup_session_set_user_agent")]
    fn set_user_agent(&self, user_agent: &str) {
        unsafe {
            ffi::soup_session_set_user_agent(
                self.as_ref().to_glib_none().0,
                user_agent.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "request-queued")]
    fn connect_request_queued<F: Fn(&Self, &Message) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn request_queued_trampoline<
            P: IsA<Session>,
            F: Fn(&P, &Message) + 'static,
        >(
            this: *mut ffi::SoupSession,
            msg: *mut ffi::SoupMessage,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Session::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(msg),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"request-queued\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    request_queued_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "request-unqueued")]
    fn connect_request_unqueued<F: Fn(&Self, &Message) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn request_unqueued_trampoline<
            P: IsA<Session>,
            F: Fn(&P, &Message) + 'static,
        >(
            this: *mut ffi::SoupSession,
            msg: *mut ffi::SoupMessage,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Session::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(msg),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"request-unqueued\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    request_unqueued_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accept-language")]
    fn connect_accept_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accept_language_trampoline<
            P: IsA<Session>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::SoupSession,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Session::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accept-language\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accept_language_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accept-language-auto")]
    fn connect_accept_language_auto_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accept_language_auto_trampoline<
            P: IsA<Session>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::SoupSession,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Session::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accept-language-auto\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accept_language_auto_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "idle-timeout")]
    fn connect_idle_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_idle_timeout_trampoline<
            P: IsA<Session>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::SoupSession,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Session::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::idle-timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_idle_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "proxy-resolver")]
    fn connect_proxy_resolver_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_proxy_resolver_trampoline<
            P: IsA<Session>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::SoupSession,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Session::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::proxy-resolver\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_proxy_resolver_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "timeout")]
    fn connect_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_trampoline<P: IsA<Session>, F: Fn(&P) + 'static>(
            this: *mut ffi::SoupSession,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Session::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tls-database")]
    fn connect_tls_database_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tls_database_trampoline<
            P: IsA<Session>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::SoupSession,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Session::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tls-database\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tls_database_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tls-interaction")]
    fn connect_tls_interaction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tls_interaction_trampoline<
            P: IsA<Session>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::SoupSession,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Session::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tls-interaction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tls_interaction_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "user-agent")]
    fn connect_user_agent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_user_agent_trampoline<P: IsA<Session>, F: Fn(&P) + 'static>(
            this: *mut ffi::SoupSession,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Session::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::user-agent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_user_agent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Session>> SessionExt for O {}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Session")
    }
}