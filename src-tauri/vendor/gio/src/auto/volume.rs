// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    AsyncResult, Cancellable, Drive, File, Icon, Mount, MountMountFlags, MountOperation,
    MountUnmountFlags,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "GVolume")]
    pub struct Volume(Interface<ffi::GVolume, ffi::GVolumeIface>);

    match fn {
        type_ => || ffi::g_volume_get_type(),
    }
}

impl Volume {
    pub const NONE: Option<&'static Volume> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Volume>> Sealed for T {}
}

pub trait VolumeExt: IsA<Volume> + sealed::Sealed + 'static {
    #[doc(alias = "g_volume_can_eject")]
    fn can_eject(&self) -> bool {
        unsafe { from_glib(ffi::g_volume_can_eject(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_volume_can_mount")]
    fn can_mount(&self) -> bool {
        unsafe { from_glib(ffi::g_volume_can_mount(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_volume_eject_with_operation")]
    fn eject_with_operation<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&impl IsA<MountOperation>>,
        cancellable: Option<&impl IsA<Cancellable>>,
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
        unsafe extern "C" fn eject_with_operation_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_volume_eject_with_operation_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
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
        let callback = eject_with_operation_trampoline::<P>;
        unsafe {
            ffi::g_volume_eject_with_operation(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn eject_with_operation_future(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&(impl IsA<MountOperation> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.eject_with_operation(
                    flags,
                    mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    #[doc(alias = "g_volume_enumerate_identifiers")]
    fn enumerate_identifiers(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_volume_enumerate_identifiers(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_volume_get_activation_root")]
    #[doc(alias = "get_activation_root")]
    fn activation_root(&self) -> Option<File> {
        unsafe {
            from_glib_full(ffi::g_volume_get_activation_root(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_volume_get_drive")]
    #[doc(alias = "get_drive")]
    fn drive(&self) -> Option<Drive> {
        unsafe { from_glib_full(ffi::g_volume_get_drive(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_volume_get_icon")]
    #[doc(alias = "get_icon")]
    fn icon(&self) -> Icon {
        unsafe { from_glib_full(ffi::g_volume_get_icon(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_volume_get_identifier")]
    #[doc(alias = "get_identifier")]
    fn identifier(&self, kind: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_volume_get_identifier(
                self.as_ref().to_glib_none().0,
                kind.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_volume_get_mount")]
    fn get_mount(&self) -> Option<Mount> {
        unsafe { from_glib_full(ffi::g_volume_get_mount(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_volume_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::g_volume_get_name(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_volume_get_sort_key")]
    #[doc(alias = "get_sort_key")]
    fn sort_key(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_volume_get_sort_key(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_volume_get_symbolic_icon")]
    #[doc(alias = "get_symbolic_icon")]
    fn symbolic_icon(&self) -> Icon {
        unsafe {
            from_glib_full(ffi::g_volume_get_symbolic_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_volume_get_uuid")]
    #[doc(alias = "get_uuid")]
    fn uuid(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::g_volume_get_uuid(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_volume_mount")]
    fn mount<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        flags: MountMountFlags,
        mount_operation: Option<&impl IsA<MountOperation>>,
        cancellable: Option<&impl IsA<Cancellable>>,
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
        unsafe extern "C" fn mount_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_volume_mount_finish(_source_object as *mut _, res, &mut error);
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
        let callback = mount_trampoline::<P>;
        unsafe {
            ffi::g_volume_mount(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn mount_future(
        &self,
        flags: MountMountFlags,
        mount_operation: Option<&(impl IsA<MountOperation> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.mount(
                    flags,
                    mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    #[doc(alias = "g_volume_should_automount")]
    fn should_automount(&self) -> bool {
        unsafe {
            from_glib(ffi::g_volume_should_automount(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P: IsA<Volume>, F: Fn(&P) + 'static>(
            this: *mut ffi::GVolume,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Volume::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "removed")]
    fn connect_removed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn removed_trampoline<P: IsA<Volume>, F: Fn(&P) + 'static>(
            this: *mut ffi::GVolume,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Volume::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Volume>> VolumeExt for O {}

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Volume")
    }
}
