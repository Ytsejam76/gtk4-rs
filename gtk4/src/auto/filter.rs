// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, FilterChange, FilterMatch};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkFilter")]
    pub struct Filter(Object<ffi::GtkFilter, ffi::GtkFilterClass>);

    match fn {
        type_ => || ffi::gtk_filter_get_type(),
    }
}

impl Filter {
    pub const NONE: Option<&'static Filter> = None;
}

pub trait FilterExt: IsA<Filter> + 'static {
    #[doc(alias = "gtk_filter_changed")]
    fn changed(&self, change: FilterChange) {
        unsafe {
            ffi::gtk_filter_changed(self.as_ref().to_glib_none().0, change.into_glib());
        }
    }

    #[doc(alias = "gtk_filter_get_strictness")]
    #[doc(alias = "get_strictness")]
    fn strictness(&self) -> FilterMatch {
        unsafe {
            from_glib(ffi::gtk_filter_get_strictness(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_filter_match")]
    #[doc(alias = "match")]
    fn match_(&self, item: &impl IsA<glib::Object>) -> bool {
        unsafe {
            from_glib(ffi::gtk_filter_match(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self, FilterChange) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<
            P: IsA<Filter>,
            F: Fn(&P, FilterChange) + 'static,
        >(
            this: *mut ffi::GtkFilter,
            change: ffi::GtkFilterChange,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Filter::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(change),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"changed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Filter>> FilterExt for O {}
