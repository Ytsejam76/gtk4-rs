// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::CellArea;
use crate::CellRenderer;
use crate::TreeIter;
use crate::TreeModel;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;

glib::glib_wrapper! {
    pub struct CellLayout(Interface<ffi::GtkCellLayout>);

    match fn {
        get_type => || ffi::gtk_cell_layout_get_type(),
    }
}

pub const NONE_CELL_LAYOUT: Option<&CellLayout> = None;

pub trait CellLayoutExt: 'static {
    #[doc(alias = "gtk_cell_layout_add_attribute")]
    fn add_attribute<P: IsA<CellRenderer>>(&self, cell: &P, attribute: &str, column: i32);

    #[doc(alias = "gtk_cell_layout_clear")]
    fn clear(&self);

    #[doc(alias = "gtk_cell_layout_clear_attributes")]
    fn clear_attributes<P: IsA<CellRenderer>>(&self, cell: &P);

    #[doc(alias = "gtk_cell_layout_get_area")]
    fn get_area(&self) -> Option<CellArea>;

    #[doc(alias = "gtk_cell_layout_get_cells")]
    fn get_cells(&self) -> Vec<CellRenderer>;

    #[doc(alias = "gtk_cell_layout_pack_end")]
    fn pack_end<P: IsA<CellRenderer>>(&self, cell: &P, expand: bool);

    #[doc(alias = "gtk_cell_layout_pack_start")]
    fn pack_start<P: IsA<CellRenderer>>(&self, cell: &P, expand: bool);

    #[doc(alias = "gtk_cell_layout_reorder")]
    fn reorder<P: IsA<CellRenderer>>(&self, cell: &P, position: i32);

    //#[doc(alias = "gtk_cell_layout_set_attributes")]
    //fn set_attributes<P: IsA<CellRenderer>>(&self, cell: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[doc(alias = "gtk_cell_layout_set_cell_data_func")]
    fn set_cell_data_func<P: IsA<CellRenderer>>(
        &self,
        cell: &P,
        func: Option<Box_<dyn Fn(&CellLayout, &CellRenderer, &TreeModel, &TreeIter) + 'static>>,
    );
}

impl<O: IsA<CellLayout>> CellLayoutExt for O {
    fn add_attribute<P: IsA<CellRenderer>>(&self, cell: &P, attribute: &str, column: i32) {
        unsafe {
            ffi::gtk_cell_layout_add_attribute(
                self.as_ref().to_glib_none().0,
                cell.as_ref().to_glib_none().0,
                attribute.to_glib_none().0,
                column,
            );
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::gtk_cell_layout_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn clear_attributes<P: IsA<CellRenderer>>(&self, cell: &P) {
        unsafe {
            ffi::gtk_cell_layout_clear_attributes(
                self.as_ref().to_glib_none().0,
                cell.as_ref().to_glib_none().0,
            );
        }
    }

    fn get_area(&self) -> Option<CellArea> {
        unsafe {
            from_glib_none(ffi::gtk_cell_layout_get_area(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_cells(&self) -> Vec<CellRenderer> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_cell_layout_get_cells(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pack_end<P: IsA<CellRenderer>>(&self, cell: &P, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_end(
                self.as_ref().to_glib_none().0,
                cell.as_ref().to_glib_none().0,
                expand.to_glib(),
            );
        }
    }

    fn pack_start<P: IsA<CellRenderer>>(&self, cell: &P, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_start(
                self.as_ref().to_glib_none().0,
                cell.as_ref().to_glib_none().0,
                expand.to_glib(),
            );
        }
    }

    fn reorder<P: IsA<CellRenderer>>(&self, cell: &P, position: i32) {
        unsafe {
            ffi::gtk_cell_layout_reorder(
                self.as_ref().to_glib_none().0,
                cell.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    //fn set_attributes<P: IsA<CellRenderer>>(&self, cell: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_cell_layout_set_attributes() }
    //}

    fn set_cell_data_func<P: IsA<CellRenderer>>(
        &self,
        cell: &P,
        func: Option<Box_<dyn Fn(&CellLayout, &CellRenderer, &TreeModel, &TreeIter) + 'static>>,
    ) {
        let func_data: Box_<
            Option<Box_<dyn Fn(&CellLayout, &CellRenderer, &TreeModel, &TreeIter) + 'static>>,
        > = Box_::new(func);
        unsafe extern "C" fn func_func<P: IsA<CellRenderer>>(
            cell_layout: *mut ffi::GtkCellLayout,
            cell: *mut ffi::GtkCellRenderer,
            tree_model: *mut ffi::GtkTreeModel,
            iter: *mut ffi::GtkTreeIter,
            data: glib::ffi::gpointer,
        ) {
            let cell_layout = from_glib_borrow(cell_layout);
            let cell = from_glib_borrow(cell);
            let tree_model = from_glib_borrow(tree_model);
            let iter = from_glib_borrow(iter);
            let callback: &Option<
                Box_<dyn Fn(&CellLayout, &CellRenderer, &TreeModel, &TreeIter) + 'static>,
            > = &*(data as *mut _);
            if let Some(ref callback) = *callback {
                callback(&cell_layout, &cell, &tree_model, &iter)
            } else {
                panic!("cannot get closure...")
            };
        }
        let func = if func_data.is_some() {
            Some(func_func::<P> as _)
        } else {
            None
        };
        unsafe extern "C" fn destroy_func<P: IsA<CellRenderer>>(data: glib::ffi::gpointer) {
            let _callback: Box_<
                Option<Box_<dyn Fn(&CellLayout, &CellRenderer, &TreeModel, &TreeIter) + 'static>>,
            > = Box_::from_raw(data as *mut _);
        }
        let destroy_call4 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<
            Option<Box_<dyn Fn(&CellLayout, &CellRenderer, &TreeModel, &TreeIter) + 'static>>,
        > = func_data;
        unsafe {
            ffi::gtk_cell_layout_set_cell_data_func(
                self.as_ref().to_glib_none().0,
                cell.as_ref().to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call4,
            );
        }
    }
}

impl fmt::Display for CellLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellLayout")
    }
}
