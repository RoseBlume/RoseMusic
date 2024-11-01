// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{EventController, Gesture, GestureSingle, PropagationPhase, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkGestureLongPress")]
    pub struct GestureLongPress(Object<ffi::GtkGestureLongPress, ffi::GtkGestureLongPressClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_long_press_get_type(),
    }
}

impl GestureLongPress {
    #[doc(alias = "gtk_gesture_long_press_new")]
    pub fn new(widget: &impl IsA<Widget>) -> GestureLongPress {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_long_press_new(
                widget.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`GestureLongPress`] objects.
    ///
    /// This method returns an instance of [`GestureLongPressBuilder`](crate::builders::GestureLongPressBuilder) which can be used to create [`GestureLongPress`] objects.
    pub fn builder() -> GestureLongPressBuilder {
        GestureLongPressBuilder::new()
    }

    #[doc(alias = "delay-factor")]
    pub fn delay_factor(&self) -> f64 {
        ObjectExt::property(self, "delay-factor")
    }

    #[doc(alias = "delay-factor")]
    pub fn set_delay_factor(&self, delay_factor: f64) {
        ObjectExt::set_property(self, "delay-factor", delay_factor)
    }

    #[doc(alias = "cancelled")]
    pub fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancelled_trampoline<F: Fn(&GestureLongPress) + 'static>(
            this: *mut ffi::GtkGestureLongPress,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancelled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancelled_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pressed")]
    pub fn connect_pressed<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn pressed_trampoline<F: Fn(&GestureLongPress, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureLongPress,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pressed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    pressed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "delay-factor")]
    pub fn connect_delay_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_delay_factor_trampoline<F: Fn(&GestureLongPress) + 'static>(
            this: *mut ffi::GtkGestureLongPress,
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
                b"notify::delay-factor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_delay_factor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureLongPress {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`GestureLongPress`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GestureLongPressBuilder {
    builder: glib::object::ObjectBuilder<'static, GestureLongPress>,
}

impl GestureLongPressBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn delay_factor(self, delay_factor: f64) -> Self {
        Self {
            builder: self.builder.property("delay-factor", delay_factor),
        }
    }

    pub fn button(self, button: u32) -> Self {
        Self {
            builder: self.builder.property("button", button),
        }
    }

    pub fn exclusive(self, exclusive: bool) -> Self {
        Self {
            builder: self.builder.property("exclusive", exclusive),
        }
    }

    pub fn touch_only(self, touch_only: bool) -> Self {
        Self {
            builder: self.builder.property("touch-only", touch_only),
        }
    }

    pub fn n_points(self, n_points: u32) -> Self {
        Self {
            builder: self.builder.property("n-points", n_points),
        }
    }

    pub fn window(self, window: &gdk::Window) -> Self {
        Self {
            builder: self.builder.property("window", window.clone()),
        }
    }

    pub fn propagation_phase(self, propagation_phase: PropagationPhase) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-phase", propagation_phase),
        }
    }

    pub fn widget(self, widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("widget", widget.clone().upcast()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GestureLongPress`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> GestureLongPress {
        self.builder.build()
    }
}

impl fmt::Display for GestureLongPress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GestureLongPress")
    }
}
