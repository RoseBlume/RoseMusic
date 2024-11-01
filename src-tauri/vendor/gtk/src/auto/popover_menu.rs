// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Align, Bin, Buildable, Container, Popover, PopoverConstraint, PositionType, ResizeMode, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkPopoverMenu")]
    pub struct PopoverMenu(Object<ffi::GtkPopoverMenu, ffi::GtkPopoverMenuClass>) @extends Popover, Bin, Container, Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_popover_menu_get_type(),
    }
}

impl PopoverMenu {
    #[doc(alias = "gtk_popover_menu_new")]
    pub fn new() -> PopoverMenu {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_popover_menu_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`PopoverMenu`] objects.
    ///
    /// This method returns an instance of [`PopoverMenuBuilder`](crate::builders::PopoverMenuBuilder) which can be used to create [`PopoverMenu`] objects.
    pub fn builder() -> PopoverMenuBuilder {
        PopoverMenuBuilder::new()
    }

    #[doc(alias = "gtk_popover_menu_open_submenu")]
    pub fn open_submenu(&self, name: &str) {
        unsafe {
            ffi::gtk_popover_menu_open_submenu(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "visible-submenu")]
    pub fn visible_submenu(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "visible-submenu")
    }

    #[doc(alias = "visible-submenu")]
    pub fn set_visible_submenu(&self, visible_submenu: Option<&str>) {
        ObjectExt::set_property(self, "visible-submenu", visible_submenu)
    }

    pub fn child_position<T: IsA<crate::Widget>>(&self, item: &T) -> i32 {
        crate::prelude::ContainerExtManual::child_property(self, &item.clone().upcast(), "position")
    }

    pub fn set_child_position<T: IsA<crate::Widget>>(&self, item: &T, position: i32) {
        crate::prelude::ContainerExtManual::child_set_property(
            self,
            &item.clone().upcast(),
            "position",
            &position,
        )
    }

    pub fn child_submenu<T: IsA<crate::Widget>>(&self, item: &T) -> Option<glib::GString> {
        crate::prelude::ContainerExtManual::child_property(self, &item.clone().upcast(), "submenu")
    }

    pub fn set_child_submenu<T: IsA<crate::Widget>>(&self, item: &T, submenu: Option<&str>) {
        crate::prelude::ContainerExtManual::child_set_property(
            self,
            &item.clone().upcast(),
            "submenu",
            &submenu,
        )
    }

    #[doc(alias = "visible-submenu")]
    pub fn connect_visible_submenu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_submenu_trampoline<F: Fn(&PopoverMenu) + 'static>(
            this: *mut ffi::GtkPopoverMenu,
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
                b"notify::visible-submenu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_submenu_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for PopoverMenu {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`PopoverMenu`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PopoverMenuBuilder {
    builder: glib::object::ObjectBuilder<'static, PopoverMenu>,
}

impl PopoverMenuBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn visible_submenu(self, visible_submenu: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("visible-submenu", visible_submenu.into()),
        }
    }

    pub fn constrain_to(self, constrain_to: PopoverConstraint) -> Self {
        Self {
            builder: self.builder.property("constrain-to", constrain_to),
        }
    }

    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    pub fn pointing_to(self, pointing_to: &gdk::Rectangle) -> Self {
        Self {
            builder: self.builder.property("pointing-to", pointing_to),
        }
    }

    pub fn position(self, position: PositionType) -> Self {
        Self {
            builder: self.builder.property("position", position),
        }
    }

    pub fn relative_to(self, relative_to: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("relative-to", relative_to.clone().upcast()),
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

    // rustdoc-stripper-ignore-next
    /// Build the [`PopoverMenu`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> PopoverMenu {
        self.builder.build()
    }
}

impl fmt::Display for PopoverMenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PopoverMenu")
    }
}
