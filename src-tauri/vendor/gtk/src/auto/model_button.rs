// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Actionable, Align, Bin, Buildable, Button, ButtonRole, Container, PositionType, ReliefStyle,
    ResizeMode, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkModelButton")]
    pub struct ModelButton(Object<ffi::GtkModelButton>) @extends Button, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        type_ => || ffi::gtk_model_button_get_type(),
    }
}

impl ModelButton {
    #[doc(alias = "gtk_model_button_new")]
    pub fn new() -> ModelButton {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_model_button_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ModelButton`] objects.
    ///
    /// This method returns an instance of [`ModelButtonBuilder`](crate::builders::ModelButtonBuilder) which can be used to create [`ModelButton`] objects.
    pub fn builder() -> ModelButtonBuilder {
        ModelButtonBuilder::new()
    }

    pub fn is_active(&self) -> bool {
        ObjectExt::property(self, "active")
    }

    pub fn set_active(&self, active: bool) {
        ObjectExt::set_property(self, "active", active)
    }

    pub fn is_centered(&self) -> bool {
        ObjectExt::property(self, "centered")
    }

    pub fn set_centered(&self, centered: bool) {
        ObjectExt::set_property(self, "centered", centered)
    }

    pub fn icon(&self) -> Option<gio::Icon> {
        ObjectExt::property(self, "icon")
    }

    pub fn set_icon<P: IsA<gio::Icon>>(&self, icon: Option<&P>) {
        ObjectExt::set_property(self, "icon", icon)
    }

    pub fn is_iconic(&self) -> bool {
        ObjectExt::property(self, "iconic")
    }

    pub fn set_iconic(&self, iconic: bool) {
        ObjectExt::set_property(self, "iconic", iconic)
    }

    pub fn is_inverted(&self) -> bool {
        ObjectExt::property(self, "inverted")
    }

    pub fn set_inverted(&self, inverted: bool) {
        ObjectExt::set_property(self, "inverted", inverted)
    }

    #[doc(alias = "menu-name")]
    pub fn menu_name(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "menu-name")
    }

    #[doc(alias = "menu-name")]
    pub fn set_menu_name(&self, menu_name: Option<&str>) {
        ObjectExt::set_property(self, "menu-name", menu_name)
    }

    pub fn role(&self) -> ButtonRole {
        ObjectExt::property(self, "role")
    }

    pub fn set_role(&self, role: ButtonRole) {
        ObjectExt::set_property(self, "role", role)
    }

    pub fn text(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "text")
    }

    pub fn set_text(&self, text: Option<&str>) {
        ObjectExt::set_property(self, "text", text)
    }

    #[cfg(feature = "v3_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_24")))]
    #[doc(alias = "use-markup")]
    pub fn uses_markup(&self) -> bool {
        ObjectExt::property(self, "use-markup")
    }

    #[cfg(feature = "v3_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_24")))]
    #[doc(alias = "use-markup")]
    pub fn set_use_markup(&self, use_markup: bool) {
        ObjectExt::set_property(self, "use-markup", use_markup)
    }

    #[doc(alias = "active")]
    pub fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut ffi::GtkModelButton,
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
                b"notify::active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "centered")]
    pub fn connect_centered_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_centered_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut ffi::GtkModelButton,
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
                b"notify::centered\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_centered_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon")]
    pub fn connect_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut ffi::GtkModelButton,
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
                b"notify::icon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "iconic")]
    pub fn connect_iconic_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_iconic_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut ffi::GtkModelButton,
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
                b"notify::iconic\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_iconic_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "inverted")]
    pub fn connect_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut ffi::GtkModelButton,
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
                b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inverted_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "menu-name")]
    pub fn connect_menu_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menu_name_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut ffi::GtkModelButton,
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
                b"notify::menu-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_menu_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "role")]
    pub fn connect_role_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_role_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut ffi::GtkModelButton,
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
                b"notify::role\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_role_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "text")]
    pub fn connect_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut ffi::GtkModelButton,
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
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v3_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_24")))]
    #[doc(alias = "use-markup")]
    pub fn connect_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_markup_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut ffi::GtkModelButton,
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
                b"notify::use-markup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_markup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ModelButton {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ModelButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ModelButtonBuilder {
    builder: glib::object::ObjectBuilder<'static, ModelButton>,
}

impl ModelButtonBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn active(self, active: bool) -> Self {
        Self {
            builder: self.builder.property("active", active),
        }
    }

    pub fn centered(self, centered: bool) -> Self {
        Self {
            builder: self.builder.property("centered", centered),
        }
    }

    pub fn icon(self, icon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self.builder.property("icon", icon.clone().upcast()),
        }
    }

    pub fn iconic(self, iconic: bool) -> Self {
        Self {
            builder: self.builder.property("iconic", iconic),
        }
    }

    pub fn inverted(self, inverted: bool) -> Self {
        Self {
            builder: self.builder.property("inverted", inverted),
        }
    }

    pub fn menu_name(self, menu_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("menu-name", menu_name.into()),
        }
    }

    pub fn role(self, role: ButtonRole) -> Self {
        Self {
            builder: self.builder.property("role", role),
        }
    }

    pub fn text(self, text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("text", text.into()),
        }
    }

    #[cfg(feature = "v3_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_24")))]
    pub fn use_markup(self, use_markup: bool) -> Self {
        Self {
            builder: self.builder.property("use-markup", use_markup),
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
    /// Build the [`ModelButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ModelButton {
        self.builder.build()
    }
}

impl fmt::Display for ModelButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ModelButton")
    }
}
