// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Accessible;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::Widget;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;

glib::glib_wrapper! {
    pub struct Native(Interface<ffi::GtkNative>) @requires Widget, Accessible, Buildable, ConstraintTarget;

    match fn {
        get_type => || ffi::gtk_native_get_type(),
    }
}

impl Native {
    #[doc(alias = "gtk_native_get_for_surface")]
    pub fn get_for_surface(surface: &gdk::Surface) -> Option<Native> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_native_get_for_surface(surface.to_glib_none().0)) }
    }
}

pub const NONE_NATIVE: Option<&Native> = None;

pub trait NativeExt: 'static {
    #[doc(alias = "gtk_native_check_resize")]
    fn check_resize(&self);

    #[doc(alias = "gtk_native_get_renderer")]
    fn get_renderer(&self) -> Option<gsk::Renderer>;

    #[doc(alias = "gtk_native_get_surface")]
    fn get_surface(&self) -> Option<gdk::Surface>;

    #[doc(alias = "gtk_native_get_surface_transform")]
    fn get_surface_transform(&self) -> (f64, f64);
}

impl<O: IsA<Native>> NativeExt for O {
    fn check_resize(&self) {
        unsafe {
            ffi::gtk_native_check_resize(self.as_ref().to_glib_none().0);
        }
    }

    fn get_renderer(&self) -> Option<gsk::Renderer> {
        unsafe { from_glib_none(ffi::gtk_native_get_renderer(self.as_ref().to_glib_none().0)) }
    }

    fn get_surface(&self) -> Option<gdk::Surface> {
        unsafe { from_glib_none(ffi::gtk_native_get_surface(self.as_ref().to_glib_none().0)) }
    }

    fn get_surface_transform(&self) -> (f64, f64) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            ffi::gtk_native_get_surface_transform(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            );
            let x = x.assume_init();
            let y = y.assume_init();
            (x, y)
        }
    }
}

impl fmt::Display for Native {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Native")
    }
}
