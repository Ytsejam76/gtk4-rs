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
    #[doc(alias = "GtkTreeListRow")]
    pub struct TreeListRow(Object<ffi::GtkTreeListRow, ffi::GtkTreeListRowClass>);

    match fn {
        type_ => || ffi::gtk_tree_list_row_get_type(),
    }
}

impl TreeListRow {
    #[doc(alias = "gtk_tree_list_row_get_child_row")]
    #[doc(alias = "get_child_row")]
    #[must_use]
    pub fn child_row(&self, position: u32) -> Option<TreeListRow> {
        unsafe {
            from_glib_full(ffi::gtk_tree_list_row_get_child_row(
                self.to_glib_none().0,
                position,
            ))
        }
    }

    #[doc(alias = "gtk_tree_list_row_get_children")]
    #[doc(alias = "get_children")]
    pub fn children(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::gtk_tree_list_row_get_children(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_list_row_get_depth")]
    #[doc(alias = "get_depth")]
    pub fn depth(&self) -> u32 {
        unsafe { ffi::gtk_tree_list_row_get_depth(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_tree_list_row_get_expanded")]
    #[doc(alias = "get_expanded")]
    #[doc(alias = "expanded")]
    pub fn is_expanded(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_list_row_get_expanded(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_list_row_get_item")]
    #[doc(alias = "get_item")]
    pub fn item(&self) -> Option<glib::Object> {
        unsafe { from_glib_full(ffi::gtk_tree_list_row_get_item(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_list_row_get_parent")]
    #[doc(alias = "get_parent")]
    #[must_use]
    pub fn parent(&self) -> Option<TreeListRow> {
        unsafe { from_glib_full(ffi::gtk_tree_list_row_get_parent(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_list_row_get_position")]
    #[doc(alias = "get_position")]
    pub fn position(&self) -> u32 {
        unsafe { ffi::gtk_tree_list_row_get_position(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_tree_list_row_is_expandable")]
    #[doc(alias = "expandable")]
    pub fn is_expandable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_list_row_is_expandable(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_tree_list_row_set_expanded")]
    #[doc(alias = "expanded")]
    pub fn set_expanded(&self, expanded: bool) {
        unsafe {
            ffi::gtk_tree_list_row_set_expanded(self.to_glib_none().0, expanded.into_glib());
        }
    }

    #[doc(alias = "children")]
    pub fn connect_children_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_children_trampoline<F: Fn(&TreeListRow) + 'static>(
            this: *mut ffi::GtkTreeListRow,
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
                c"notify::children".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_children_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "depth")]
    pub fn connect_depth_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_depth_trampoline<F: Fn(&TreeListRow) + 'static>(
            this: *mut ffi::GtkTreeListRow,
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
                c"notify::depth".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_depth_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "expandable")]
    pub fn connect_expandable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expandable_trampoline<F: Fn(&TreeListRow) + 'static>(
            this: *mut ffi::GtkTreeListRow,
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
                c"notify::expandable".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_expandable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "expanded")]
    pub fn connect_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expanded_trampoline<F: Fn(&TreeListRow) + 'static>(
            this: *mut ffi::GtkTreeListRow,
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
                c"notify::expanded".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_expanded_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "item")]
    pub fn connect_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_item_trampoline<F: Fn(&TreeListRow) + 'static>(
            this: *mut ffi::GtkTreeListRow,
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
                c"notify::item".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_item_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
