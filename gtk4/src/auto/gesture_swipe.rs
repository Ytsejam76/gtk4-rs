// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, EventController, Gesture, GestureSingle, PropagationLimit, PropagationPhase};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkGestureSwipe")]
    pub struct GestureSwipe(Object<ffi::GtkGestureSwipe, ffi::GtkGestureSwipeClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_swipe_get_type(),
    }
}

impl GestureSwipe {
    #[doc(alias = "gtk_gesture_swipe_new")]
    pub fn new() -> GestureSwipe {
        assert_initialized_main_thread!();
        unsafe { Gesture::from_glib_full(ffi::gtk_gesture_swipe_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`GestureSwipe`] objects.
    ///
    /// This method returns an instance of [`GestureSwipeBuilder`](crate::builders::GestureSwipeBuilder) which can be used to create [`GestureSwipe`] objects.
    pub fn builder() -> GestureSwipeBuilder {
        GestureSwipeBuilder::new()
    }

    #[doc(alias = "gtk_gesture_swipe_get_velocity")]
    #[doc(alias = "get_velocity")]
    pub fn velocity(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut velocity_x = std::mem::MaybeUninit::uninit();
            let mut velocity_y = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_gesture_swipe_get_velocity(
                self.to_glib_none().0,
                velocity_x.as_mut_ptr(),
                velocity_y.as_mut_ptr(),
            ));
            if ret {
                Some((velocity_x.assume_init(), velocity_y.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "swipe")]
    pub fn connect_swipe<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn swipe_trampoline<F: Fn(&GestureSwipe, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureSwipe,
            velocity_x: std::ffi::c_double,
            velocity_y: std::ffi::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), velocity_x, velocity_y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"swipe".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    swipe_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureSwipe {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`GestureSwipe`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GestureSwipeBuilder {
    builder: glib::object::ObjectBuilder<'static, GestureSwipe>,
}

impl GestureSwipeBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
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

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn propagation_limit(self, propagation_limit: PropagationLimit) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-limit", propagation_limit),
        }
    }

    pub fn propagation_phase(self, propagation_phase: PropagationPhase) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-phase", propagation_phase),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GestureSwipe`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> GestureSwipe {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
