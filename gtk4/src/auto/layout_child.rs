// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, LayoutManager, Widget};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GtkLayoutChild")]
    pub struct LayoutChild(Object<ffi::GtkLayoutChild, ffi::GtkLayoutChildClass>);

    match fn {
        type_ => || ffi::gtk_layout_child_get_type(),
    }
}

impl LayoutChild {
    pub const NONE: Option<&'static LayoutChild> = None;
}

pub trait LayoutChildExt: IsA<LayoutChild> + 'static {
    #[doc(alias = "gtk_layout_child_get_child_widget")]
    #[doc(alias = "get_child_widget")]
    #[doc(alias = "child-widget")]
    fn child_widget(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_layout_child_get_child_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_layout_child_get_layout_manager")]
    #[doc(alias = "get_layout_manager")]
    #[doc(alias = "layout-manager")]
    fn layout_manager(&self) -> LayoutManager {
        unsafe {
            from_glib_none(ffi::gtk_layout_child_get_layout_manager(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<LayoutChild>> LayoutChildExt for O {}
