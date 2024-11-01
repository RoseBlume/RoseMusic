// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Align, Bin, Buildable, Container, ResizeMode, Widget};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkOverlay")]
    pub struct Overlay(Object<ffi::GtkOverlay, ffi::GtkOverlayClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_overlay_get_type(),
    }
}

impl Overlay {
    pub const NONE: Option<&'static Overlay> = None;

    #[doc(alias = "gtk_overlay_new")]
    pub fn new() -> Overlay {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_overlay_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Overlay`] objects.
    ///
    /// This method returns an instance of [`OverlayBuilder`](crate::builders::OverlayBuilder) which can be used to create [`Overlay`] objects.
    pub fn builder() -> OverlayBuilder {
        OverlayBuilder::new()
    }
}

impl Default for Overlay {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Overlay`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct OverlayBuilder {
    builder: glib::object::ObjectBuilder<'static, Overlay>,
}

impl OverlayBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
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
    /// Build the [`Overlay`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Overlay {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Overlay>> Sealed for T {}
}

pub trait OverlayExt: IsA<Overlay> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_overlay_add_overlay")]
    fn add_overlay(&self, widget: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_overlay_add_overlay(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_overlay_get_overlay_pass_through")]
    #[doc(alias = "get_overlay_pass_through")]
    fn is_overlay_pass_through(&self, widget: &impl IsA<Widget>) -> bool {
        unsafe {
            from_glib(ffi::gtk_overlay_get_overlay_pass_through(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_overlay_reorder_overlay")]
    fn reorder_overlay(&self, child: &impl IsA<Widget>, index_: i32) {
        unsafe {
            ffi::gtk_overlay_reorder_overlay(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                index_,
            );
        }
    }

    #[doc(alias = "gtk_overlay_set_overlay_pass_through")]
    fn set_overlay_pass_through(&self, widget: &impl IsA<Widget>, pass_through: bool) {
        unsafe {
            ffi::gtk_overlay_set_overlay_pass_through(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                pass_through.into_glib(),
            );
        }
    }

    fn child_index<T: IsA<crate::Widget>>(&self, item: &T) -> i32 {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "index",
        )
    }

    fn set_child_index<T: IsA<crate::Widget>>(&self, item: &T, index: i32) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "index",
            &index,
        )
    }

    //#[doc(alias = "get-child-position")]
    //fn connect_get_child_position<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Out allocation: Gdk.Rectangle
    //}
}

impl<O: IsA<Overlay>> OverlayExt for O {}

impl fmt::Display for Overlay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Overlay")
    }
}
