// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Adjustment, Align, Buildable, Container, Orientable, Orientation, PositionType, Range,
    SensitivityType, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkScale")]
    pub struct Scale(Object<ffi::GtkScale, ffi::GtkScaleClass>) @extends Range, Widget, @implements Buildable, Orientable;

    match fn {
        type_ => || ffi::gtk_scale_get_type(),
    }
}

impl Scale {
    pub const NONE: Option<&'static Scale> = None;

    #[doc(alias = "gtk_scale_new")]
    pub fn new(orientation: Orientation, adjustment: Option<&impl IsA<Adjustment>>) -> Scale {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scale_new(
                orientation.into_glib(),
                adjustment.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_scale_new_with_range")]
    #[doc(alias = "new_with_range")]
    pub fn with_range(orientation: Orientation, min: f64, max: f64, step: f64) -> Scale {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scale_new_with_range(
                orientation.into_glib(),
                min,
                max,
                step,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Scale`] objects.
    ///
    /// This method returns an instance of [`ScaleBuilder`](crate::builders::ScaleBuilder) which can be used to create [`Scale`] objects.
    pub fn builder() -> ScaleBuilder {
        ScaleBuilder::new()
    }
}

impl Default for Scale {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Scale`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ScaleBuilder {
    builder: glib::object::ObjectBuilder<'static, Scale>,
}

impl ScaleBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn digits(self, digits: i32) -> Self {
        Self {
            builder: self.builder.property("digits", digits),
        }
    }

    pub fn draw_value(self, draw_value: bool) -> Self {
        Self {
            builder: self.builder.property("draw-value", draw_value),
        }
    }

    pub fn has_origin(self, has_origin: bool) -> Self {
        Self {
            builder: self.builder.property("has-origin", has_origin),
        }
    }

    pub fn value_pos(self, value_pos: PositionType) -> Self {
        Self {
            builder: self.builder.property("value-pos", value_pos),
        }
    }

    pub fn adjustment(self, adjustment: &impl IsA<Adjustment>) -> Self {
        Self {
            builder: self
                .builder
                .property("adjustment", adjustment.clone().upcast()),
        }
    }

    pub fn fill_level(self, fill_level: f64) -> Self {
        Self {
            builder: self.builder.property("fill-level", fill_level),
        }
    }

    pub fn inverted(self, inverted: bool) -> Self {
        Self {
            builder: self.builder.property("inverted", inverted),
        }
    }

    pub fn lower_stepper_sensitivity(self, lower_stepper_sensitivity: SensitivityType) -> Self {
        Self {
            builder: self
                .builder
                .property("lower-stepper-sensitivity", lower_stepper_sensitivity),
        }
    }

    pub fn restrict_to_fill_level(self, restrict_to_fill_level: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("restrict-to-fill-level", restrict_to_fill_level),
        }
    }

    pub fn round_digits(self, round_digits: i32) -> Self {
        Self {
            builder: self.builder.property("round-digits", round_digits),
        }
    }

    pub fn show_fill_level(self, show_fill_level: bool) -> Self {
        Self {
            builder: self.builder.property("show-fill-level", show_fill_level),
        }
    }

    pub fn upper_stepper_sensitivity(self, upper_stepper_sensitivity: SensitivityType) -> Self {
        Self {
            builder: self
                .builder
                .property("upper-stepper-sensitivity", upper_stepper_sensitivity),
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

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Scale`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Scale {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Scale>> Sealed for T {}
}

pub trait ScaleExt: IsA<Scale> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_scale_add_mark")]
    fn add_mark(&self, value: f64, position: PositionType, markup: Option<&str>) {
        unsafe {
            ffi::gtk_scale_add_mark(
                self.as_ref().to_glib_none().0,
                value,
                position.into_glib(),
                markup.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_scale_clear_marks")]
    fn clear_marks(&self) {
        unsafe {
            ffi::gtk_scale_clear_marks(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_scale_get_digits")]
    #[doc(alias = "get_digits")]
    fn digits(&self) -> i32 {
        unsafe { ffi::gtk_scale_get_digits(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gtk_scale_get_draw_value")]
    #[doc(alias = "get_draw_value")]
    fn draws_value(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scale_get_draw_value(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scale_get_has_origin")]
    #[doc(alias = "get_has_origin")]
    fn has_origin(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scale_get_has_origin(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scale_get_layout")]
    #[doc(alias = "get_layout")]
    fn layout(&self) -> Option<pango::Layout> {
        unsafe { from_glib_none(ffi::gtk_scale_get_layout(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_scale_get_layout_offsets")]
    #[doc(alias = "get_layout_offsets")]
    fn layout_offsets(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            ffi::gtk_scale_get_layout_offsets(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            );
            (x.assume_init(), y.assume_init())
        }
    }

    #[doc(alias = "gtk_scale_get_value_pos")]
    #[doc(alias = "get_value_pos")]
    fn value_pos(&self) -> PositionType {
        unsafe { from_glib(ffi::gtk_scale_get_value_pos(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_scale_set_digits")]
    fn set_digits(&self, digits: i32) {
        unsafe {
            ffi::gtk_scale_set_digits(self.as_ref().to_glib_none().0, digits);
        }
    }

    #[doc(alias = "gtk_scale_set_draw_value")]
    fn set_draw_value(&self, draw_value: bool) {
        unsafe {
            ffi::gtk_scale_set_draw_value(self.as_ref().to_glib_none().0, draw_value.into_glib());
        }
    }

    #[doc(alias = "gtk_scale_set_has_origin")]
    fn set_has_origin(&self, has_origin: bool) {
        unsafe {
            ffi::gtk_scale_set_has_origin(self.as_ref().to_glib_none().0, has_origin.into_glib());
        }
    }

    #[doc(alias = "gtk_scale_set_value_pos")]
    fn set_value_pos(&self, pos: PositionType) {
        unsafe {
            ffi::gtk_scale_set_value_pos(self.as_ref().to_glib_none().0, pos.into_glib());
        }
    }

    #[doc(alias = "format-value")]
    fn connect_format_value<F: Fn(&Self, f64) -> String + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn format_value_trampoline<
            P: IsA<Scale>,
            F: Fn(&P, f64) -> String + 'static,
        >(
            this: *mut ffi::GtkScale,
            value: libc::c_double,
            f: glib::ffi::gpointer,
        ) -> *mut libc::c_char {
            let f: &F = &*(f as *const F);
            f(Scale::from_glib_borrow(this).unsafe_cast_ref(), value).to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"format-value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    format_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "digits")]
    fn connect_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_digits_trampoline<P: IsA<Scale>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkScale,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Scale::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::digits\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_digits_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "draw-value")]
    fn connect_draw_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_draw_value_trampoline<P: IsA<Scale>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkScale,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Scale::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::draw-value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_draw_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "has-origin")]
    fn connect_has_origin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_origin_trampoline<P: IsA<Scale>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkScale,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Scale::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-origin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_origin_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "value-pos")]
    fn connect_value_pos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_pos_trampoline<P: IsA<Scale>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkScale,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Scale::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value-pos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_pos_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Scale>> ScaleExt for O {}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Scale")
    }
}