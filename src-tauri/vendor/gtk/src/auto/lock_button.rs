// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Actionable, Align, Bin, Buildable, Button, Container, PositionType, ReliefStyle, ResizeMode,
    Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkLockButton")]
    pub struct LockButton(Object<ffi::GtkLockButton, ffi::GtkLockButtonClass>) @extends Button, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        type_ => || ffi::gtk_lock_button_get_type(),
    }
}

impl LockButton {
    pub const NONE: Option<&'static LockButton> = None;

    #[doc(alias = "gtk_lock_button_new")]
    pub fn new(permission: Option<&impl IsA<gio::Permission>>) -> LockButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_lock_button_new(
                permission.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`LockButton`] objects.
    ///
    /// This method returns an instance of [`LockButtonBuilder`](crate::builders::LockButtonBuilder) which can be used to create [`LockButton`] objects.
    pub fn builder() -> LockButtonBuilder {
        LockButtonBuilder::new()
    }
}

impl Default for LockButton {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`LockButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct LockButtonBuilder {
    builder: glib::object::ObjectBuilder<'static, LockButton>,
}

impl LockButtonBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn permission(self, permission: &impl IsA<gio::Permission>) -> Self {
        Self {
            builder: self
                .builder
                .property("permission", permission.clone().upcast()),
        }
    }

    pub fn text_lock(self, text_lock: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("text-lock", text_lock.into()),
        }
    }

    pub fn text_unlock(self, text_unlock: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("text-unlock", text_unlock.into()),
        }
    }

    pub fn tooltip_lock(self, tooltip_lock: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-lock", tooltip_lock.into()),
        }
    }

    pub fn tooltip_not_authorized(self, tooltip_not_authorized: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-not-authorized", tooltip_not_authorized.into()),
        }
    }

    pub fn tooltip_unlock(self, tooltip_unlock: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-unlock", tooltip_unlock.into()),
        }
    }

    pub fn always_show_image(self, always_show_image: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("always-show-image", always_show_image),
        }
    }

    pub fn image(self, image: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("image", image.clone().upcast()),
        }
    }

    pub fn image_position(self, image_position: PositionType) -> Self {
        Self {
            builder: self.builder.property("image-position", image_position),
        }
    }

    pub fn label(self, label: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("label", label.into()),
        }
    }

    pub fn relief(self, relief: ReliefStyle) -> Self {
        Self {
            builder: self.builder.property("relief", relief),
        }
    }

    pub fn use_underline(self, use_underline: bool) -> Self {
        Self {
            builder: self.builder.property("use-underline", use_underline),
        }
    }

    pub fn border_width(self, border_width: u32) -> Self {
        Self {
            builder: self.builder.property("border-width", border_width),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn resize_mode(self, resize_mode: ResizeMode) -> Self {
        Self {
            builder: self.builder.property("resize-mode", resize_mode),
        }
    }

    pub fn app_paintable(self, app_paintable: bool) -> Self {
        Self {
            builder: self.builder.property("app-paintable", app_paintable),
        }
    }

    pub fn can_default(self, can_default: bool) -> Self {
        Self {
            builder: self.builder.property("can-default", can_default),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn events(self, events: gdk::EventMask) -> Self {
        Self {
            builder: self.builder.property("events", events),
        }
    }

    pub fn expand(self, expand: bool) -> Self {
        Self {
            builder: self.builder.property("expand", expand),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_default(self, has_default: bool) -> Self {
        Self {
            builder: self.builder.property("has-default", has_default),
        }
    }

    pub fn has_focus(self, has_focus: bool) -> Self {
        Self {
            builder: self.builder.property("has-focus", has_focus),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn is_focus(self, is_focus: bool) -> Self {
        Self {
            builder: self.builder.property("is-focus", is_focus),
        }
    }

    pub fn margin(self, margin: i32) -> Self {
        Self {
            builder: self.builder.property("margin", margin),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn no_show_all(self, no_show_all: bool) -> Self {
        Self {
            builder: self.builder.property("no-show-all", no_show_all),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn parent(self, parent: &impl IsA<Container>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn action_name(self, action_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("action-name", action_name.into()),
        }
    }

    pub fn action_target(self, action_target: &glib::Variant) -> Self {
        Self {
            builder: self
                .builder
                .property("action-target", action_target.clone()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`LockButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> LockButton {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::LockButton>> Sealed for T {}
}

pub trait LockButtonExt: IsA<LockButton> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_lock_button_get_permission")]
    #[doc(alias = "get_permission")]
    fn permission(&self) -> Option<gio::Permission> {
        unsafe {
            from_glib_none(ffi::gtk_lock_button_get_permission(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_lock_button_set_permission")]
    fn set_permission(&self, permission: Option<&impl IsA<gio::Permission>>) {
        unsafe {
            ffi::gtk_lock_button_set_permission(
                self.as_ref().to_glib_none().0,
                permission.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "text-lock")]
    fn text_lock(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "text-lock")
    }

    #[doc(alias = "text-lock")]
    fn set_text_lock(&self, text_lock: Option<&str>) {
        ObjectExt::set_property(self.as_ref(), "text-lock", text_lock)
    }

    #[doc(alias = "text-unlock")]
    fn text_unlock(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "text-unlock")
    }

    #[doc(alias = "text-unlock")]
    fn set_text_unlock(&self, text_unlock: Option<&str>) {
        ObjectExt::set_property(self.as_ref(), "text-unlock", text_unlock)
    }

    #[doc(alias = "tooltip-lock")]
    fn tooltip_lock(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "tooltip-lock")
    }

    #[doc(alias = "tooltip-lock")]
    fn set_tooltip_lock(&self, tooltip_lock: Option<&str>) {
        ObjectExt::set_property(self.as_ref(), "tooltip-lock", tooltip_lock)
    }

    #[doc(alias = "tooltip-not-authorized")]
    fn tooltip_not_authorized(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "tooltip-not-authorized")
    }

    #[doc(alias = "tooltip-not-authorized")]
    fn set_tooltip_not_authorized(&self, tooltip_not_authorized: Option<&str>) {
        ObjectExt::set_property(
            self.as_ref(),
            "tooltip-not-authorized",
            tooltip_not_authorized,
        )
    }

    #[doc(alias = "tooltip-unlock")]
    fn tooltip_unlock(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "tooltip-unlock")
    }

    #[doc(alias = "tooltip-unlock")]
    fn set_tooltip_unlock(&self, tooltip_unlock: Option<&str>) {
        ObjectExt::set_property(self.as_ref(), "tooltip-unlock", tooltip_unlock)
    }

    #[doc(alias = "permission")]
    fn connect_permission_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_permission_trampoline<
            P: IsA<LockButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkLockButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LockButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::permission\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_permission_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "text-lock")]
    fn connect_text_lock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_lock_trampoline<
            P: IsA<LockButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkLockButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LockButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text-lock\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_lock_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "text-unlock")]
    fn connect_text_unlock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_unlock_trampoline<
            P: IsA<LockButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkLockButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LockButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text-unlock\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_unlock_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tooltip-lock")]
    fn connect_tooltip_lock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tooltip_lock_trampoline<
            P: IsA<LockButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkLockButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LockButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tooltip-lock\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tooltip_lock_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tooltip-not-authorized")]
    fn connect_tooltip_not_authorized_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_tooltip_not_authorized_trampoline<
            P: IsA<LockButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkLockButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LockButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tooltip-not-authorized\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tooltip_not_authorized_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tooltip-unlock")]
    fn connect_tooltip_unlock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tooltip_unlock_trampoline<
            P: IsA<LockButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkLockButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LockButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tooltip-unlock\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tooltip_unlock_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<LockButton>> LockButtonExt for O {}

impl fmt::Display for LockButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("LockButton")
    }
}