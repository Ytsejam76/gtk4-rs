// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Bitset};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkSelectionModel")]
    pub struct SelectionModel(Interface<ffi::GtkSelectionModel, ffi::GtkSelectionModelInterface>) @requires gio::ListModel;

    match fn {
        type_ => || ffi::gtk_selection_model_get_type(),
    }
}

impl SelectionModel {
    pub const NONE: Option<&'static SelectionModel> = None;
}

pub trait SelectionModelExt: IsA<SelectionModel> + 'static {
    #[doc(alias = "gtk_selection_model_get_selection")]
    #[doc(alias = "get_selection")]
    fn selection(&self) -> Bitset {
        unsafe {
            from_glib_full(ffi::gtk_selection_model_get_selection(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_selection_model_get_selection_in_range")]
    #[doc(alias = "get_selection_in_range")]
    fn selection_in_range(&self, position: u32, n_items: u32) -> Bitset {
        unsafe {
            from_glib_full(ffi::gtk_selection_model_get_selection_in_range(
                self.as_ref().to_glib_none().0,
                position,
                n_items,
            ))
        }
    }

    #[doc(alias = "gtk_selection_model_is_selected")]
    fn is_selected(&self, position: u32) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_is_selected(
                self.as_ref().to_glib_none().0,
                position,
            ))
        }
    }

    #[doc(alias = "gtk_selection_model_select_all")]
    fn select_all(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_select_all(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_selection_model_select_item")]
    fn select_item(&self, position: u32, unselect_rest: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_select_item(
                self.as_ref().to_glib_none().0,
                position,
                unselect_rest.into_glib(),
            ))
        }
    }

    #[doc(alias = "gtk_selection_model_select_range")]
    fn select_range(&self, position: u32, n_items: u32, unselect_rest: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_select_range(
                self.as_ref().to_glib_none().0,
                position,
                n_items,
                unselect_rest.into_glib(),
            ))
        }
    }

    #[doc(alias = "gtk_selection_model_selection_changed")]
    fn selection_changed(&self, position: u32, n_items: u32) {
        unsafe {
            ffi::gtk_selection_model_selection_changed(
                self.as_ref().to_glib_none().0,
                position,
                n_items,
            );
        }
    }

    #[doc(alias = "gtk_selection_model_set_selection")]
    fn set_selection(&self, selected: &Bitset, mask: &Bitset) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_set_selection(
                self.as_ref().to_glib_none().0,
                selected.to_glib_none().0,
                mask.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_selection_model_unselect_all")]
    fn unselect_all(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_unselect_all(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_selection_model_unselect_item")]
    fn unselect_item(&self, position: u32) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_unselect_item(
                self.as_ref().to_glib_none().0,
                position,
            ))
        }
    }

    #[doc(alias = "gtk_selection_model_unselect_range")]
    fn unselect_range(&self, position: u32, n_items: u32) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_unselect_range(
                self.as_ref().to_glib_none().0,
                position,
                n_items,
            ))
        }
    }

    #[doc(alias = "selection-changed")]
    fn connect_selection_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn selection_changed_trampoline<
            P: IsA<SelectionModel>,
            F: Fn(&P, u32, u32) + 'static,
        >(
            this: *mut ffi::GtkSelectionModel,
            position: std::ffi::c_uint,
            n_items: std::ffi::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                SelectionModel::from_glib_borrow(this).unsafe_cast_ref(),
                position,
                n_items,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"selection-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    selection_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<SelectionModel>> SelectionModelExt for O {}
