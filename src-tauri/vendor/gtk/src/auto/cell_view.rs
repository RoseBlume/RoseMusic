// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Align, Buildable, CellArea, CellAreaContext, CellLayout, Container, Orientable, Orientation,
    TreeModel, TreePath, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkCellView")]
    pub struct CellView(Object<ffi::GtkCellView, ffi::GtkCellViewClass>) @extends Widget, @implements Buildable, CellLayout, Orientable;

    match fn {
        type_ => || ffi::gtk_cell_view_get_type(),
    }
}

impl CellView {
    pub const NONE: Option<&'static CellView> = None;

    #[doc(alias = "gtk_cell_view_new")]
    pub fn new() -> CellView {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_cell_view_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_cell_view_new_with_context")]
    #[doc(alias = "new_with_context")]
    pub fn with_context(
        area: &impl IsA<CellArea>,
        context: &impl IsA<CellAreaContext>,
    ) -> CellView {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_context(
                area.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_cell_view_new_with_markup")]
    #[doc(alias = "new_with_markup")]
    pub fn with_markup(markup: &str) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_markup(markup.to_glib_none().0))
                .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_cell_view_new_with_pixbuf")]
    #[doc(alias = "new_with_pixbuf")]
    pub fn with_pixbuf(pixbuf: &gdk_pixbuf::Pixbuf) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_pixbuf(pixbuf.to_glib_none().0))
                .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_cell_view_new_with_text")]
    #[doc(alias = "new_with_text")]
    pub fn with_text(text: &str) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_text(text.to_glib_none().0))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CellView`] objects.
    ///
    /// This method returns an instance of [`CellViewBuilder`](crate::builders::CellViewBuilder) which can be used to create [`CellView`] objects.
    pub fn builder() -> CellViewBuilder {
        CellViewBuilder::new()
    }
}

impl Default for CellView {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CellView`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CellViewBuilder {
    builder: glib::object::ObjectBuilder<'static, CellView>,
}

impl CellViewBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn background(self, background: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("background", background.into()),
        }
    }

    pub fn background_rgba(self, background_rgba: &gdk::RGBA) -> Self {
        Self {
            builder: self.builder.property("background-rgba", background_rgba),
        }
    }

    pub fn background_set(self, background_set: bool) -> Self {
        Self {
            builder: self.builder.property("background-set", background_set),
        }
    }

    pub fn cell_area(self, cell_area: &impl IsA<CellArea>) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-area", cell_area.clone().upcast()),
        }
    }

    pub fn cell_area_context(self, cell_area_context: &impl IsA<CellAreaContext>) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-area-context", cell_area_context.clone().upcast()),
        }
    }

    pub fn draw_sensitive(self, draw_sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("draw-sensitive", draw_sensitive),
        }
    }

    pub fn fit_model(self, fit_model: bool) -> Self {
        Self {
            builder: self.builder.property("fit-model", fit_model),
        }
    }

    pub fn model(self, model: &impl IsA<TreeModel>) -> Self {
        Self {
            builder: self.builder.property("model", model.clone().upcast()),
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
    /// Build the [`CellView`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CellView {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::CellView>> Sealed for T {}
}

pub trait CellViewExt: IsA<CellView> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_cell_view_get_displayed_row")]
    #[doc(alias = "get_displayed_row")]
    fn displayed_row(&self) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_cell_view_get_displayed_row(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_cell_view_get_draw_sensitive")]
    #[doc(alias = "get_draw_sensitive")]
    fn draws_sensitive(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_view_get_draw_sensitive(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_cell_view_get_fit_model")]
    #[doc(alias = "get_fit_model")]
    fn fits_model(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_view_get_fit_model(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_cell_view_get_model")]
    #[doc(alias = "get_model")]
    fn model(&self) -> Option<TreeModel> {
        unsafe { from_glib_none(ffi::gtk_cell_view_get_model(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_cell_view_set_background_rgba")]
    fn set_background_rgba(&self, rgba: &gdk::RGBA) {
        unsafe {
            ffi::gtk_cell_view_set_background_rgba(
                self.as_ref().to_glib_none().0,
                rgba.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_cell_view_set_displayed_row")]
    fn set_displayed_row(&self, path: &mut TreePath) {
        unsafe {
            ffi::gtk_cell_view_set_displayed_row(
                self.as_ref().to_glib_none().0,
                path.to_glib_none_mut().0,
            );
        }
    }

    #[doc(alias = "gtk_cell_view_set_draw_sensitive")]
    fn set_draw_sensitive(&self, draw_sensitive: bool) {
        unsafe {
            ffi::gtk_cell_view_set_draw_sensitive(
                self.as_ref().to_glib_none().0,
                draw_sensitive.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_cell_view_set_fit_model")]
    fn set_fit_model(&self, fit_model: bool) {
        unsafe {
            ffi::gtk_cell_view_set_fit_model(self.as_ref().to_glib_none().0, fit_model.into_glib());
        }
    }

    #[doc(alias = "gtk_cell_view_set_model")]
    fn set_model(&self, model: Option<&impl IsA<TreeModel>>) {
        unsafe {
            ffi::gtk_cell_view_set_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_background(&self, background: Option<&str>) {
        ObjectExt::set_property(self.as_ref(), "background", background)
    }

    #[doc(alias = "background-rgba")]
    fn background_rgba(&self) -> Option<gdk::RGBA> {
        ObjectExt::property(self.as_ref(), "background-rgba")
    }

    #[doc(alias = "background-set")]
    fn is_background_set(&self) -> bool {
        ObjectExt::property(self.as_ref(), "background-set")
    }

    #[doc(alias = "background-set")]
    fn set_background_set(&self, background_set: bool) {
        ObjectExt::set_property(self.as_ref(), "background-set", background_set)
    }

    #[doc(alias = "cell-area")]
    fn cell_area(&self) -> Option<CellArea> {
        ObjectExt::property(self.as_ref(), "cell-area")
    }

    #[doc(alias = "cell-area-context")]
    fn cell_area_context(&self) -> Option<CellAreaContext> {
        ObjectExt::property(self.as_ref(), "cell-area-context")
    }

    #[doc(alias = "background")]
    fn connect_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_trampoline<P: IsA<CellView>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellView::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::background\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_background_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "background-rgba")]
    fn connect_background_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_rgba_trampoline<
            P: IsA<CellView>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellView::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::background-rgba\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_background_rgba_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "background-set")]
    fn connect_background_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_set_trampoline<
            P: IsA<CellView>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellView::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::background-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_background_set_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "draw-sensitive")]
    fn connect_draw_sensitive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_draw_sensitive_trampoline<
            P: IsA<CellView>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellView::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::draw-sensitive\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_draw_sensitive_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "fit-model")]
    fn connect_fit_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fit_model_trampoline<P: IsA<CellView>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellView::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::fit-model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fit_model_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "model")]
    fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<P: IsA<CellView>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellView::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<CellView>> CellViewExt for O {}

impl fmt::Display for CellView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellView")
    }
}