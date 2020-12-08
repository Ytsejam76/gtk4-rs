// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Filter;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;

glib::glib_wrapper! {
    pub struct CustomFilter(Object<ffi::GtkCustomFilter, ffi::GtkCustomFilterClass>) @extends Filter;

    match fn {
        get_type => || ffi::gtk_custom_filter_get_type(),
    }
}

impl CustomFilter {
    #[doc(alias = "gtk_custom_filter_new")]
    pub fn new(match_func: Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>) -> CustomFilter {
        assert_initialized_main_thread!();
        let match_func_data: Box_<Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>> =
            Box_::new(match_func);
        unsafe extern "C" fn match_func_func(
            item: *mut glib::gobject_ffi::GObject,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let item = from_glib_borrow(item);
            let callback: &Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>> =
                &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&item)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let match_func = if match_func_data.is_some() {
            Some(match_func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn user_destroy_func(data: glib::ffi::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>> =
                Box_::from_raw(data as *mut _);
        }
        let destroy_call2 = Some(user_destroy_func as _);
        let super_callback0: Box_<Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>> =
            match_func_data;
        unsafe {
            from_glib_full(ffi::gtk_custom_filter_new(
                match_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call2,
            ))
        }
    }

    #[doc(alias = "gtk_custom_filter_set_filter_func")]
    pub fn set_filter_func(
        &self,
        match_func: Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>,
    ) {
        let match_func_data: Box_<Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>> =
            Box_::new(match_func);
        unsafe extern "C" fn match_func_func(
            item: *mut glib::gobject_ffi::GObject,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let item = from_glib_borrow(item);
            let callback: &Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>> =
                &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&item)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let match_func = if match_func_data.is_some() {
            Some(match_func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn user_destroy_func(data: glib::ffi::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>> =
                Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(user_destroy_func as _);
        let super_callback0: Box_<Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>> =
            match_func_data;
        unsafe {
            ffi::gtk_custom_filter_set_filter_func(
                self.to_glib_none().0,
                match_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }
}

impl fmt::Display for CustomFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CustomFilter")
    }
}
