// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkSectionModel")]
    pub struct SectionModel(Interface<ffi::GtkSectionModel, ffi::GtkSectionModelInterface>) @requires gio::ListModel;

    match fn {
        type_ => || ffi::gtk_section_model_get_type(),
    }
}

impl SectionModel {
    pub const NONE: Option<&'static SectionModel> = None;
}

pub trait SectionModelExt: IsA<SectionModel> + 'static {
    #[doc(alias = "gtk_section_model_get_section")]
    #[doc(alias = "get_section")]
    fn section(&self, position: u32) -> (u32, u32) {
        unsafe {
            let mut out_start = std::mem::MaybeUninit::uninit();
            let mut out_end = std::mem::MaybeUninit::uninit();
            ffi::gtk_section_model_get_section(
                self.as_ref().to_glib_none().0,
                position,
                out_start.as_mut_ptr(),
                out_end.as_mut_ptr(),
            );
            (out_start.assume_init(), out_end.assume_init())
        }
    }

    #[doc(alias = "gtk_section_model_sections_changed")]
    fn sections_changed(&self, position: u32, n_items: u32) {
        unsafe {
            ffi::gtk_section_model_sections_changed(
                self.as_ref().to_glib_none().0,
                position,
                n_items,
            );
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "sections-changed")]
    fn connect_sections_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn sections_changed_trampoline<
            P: IsA<SectionModel>,
            F: Fn(&P, u32, u32) + 'static,
        >(
            this: *mut ffi::GtkSectionModel,
            position: std::ffi::c_uint,
            n_items: std::ffi::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                SectionModel::from_glib_borrow(this).unsafe_cast_ref(),
                position,
                n_items,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"sections-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    sections_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<SectionModel>> SectionModelExt for O {}
