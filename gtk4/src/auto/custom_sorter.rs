// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Sorter;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct CustomSorter(Object<ffi::GtkCustomSorter, ffi::GtkCustomSorterClass>) @extends Sorter;

    match fn {
        get_type => || ffi::gtk_custom_sorter_get_type(),
    }
}

impl CustomSorter {
    //#[doc(alias = "gtk_custom_sorter_new")]
    //pub fn new(sort_func: /*Unimplemented*/Fn(/*Unimplemented*/Option<Fundamental: Pointer>, /*Unimplemented*/Option<Fundamental: Pointer>) -> i32, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> CustomSorter {
    //    unsafe { TODO: call ffi:gtk_custom_sorter_new() }
    //}

    //#[doc(alias = "gtk_custom_sorter_set_sort_func")]
    //pub fn set_sort_func(&self, sort_func: /*Unimplemented*/Fn(/*Unimplemented*/Option<Fundamental: Pointer>, /*Unimplemented*/Option<Fundamental: Pointer>) -> i32, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:gtk_custom_sorter_set_sort_func() }
    //}
}

impl fmt::Display for CustomSorter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CustomSorter")
    }
}
