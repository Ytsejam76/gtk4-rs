// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::prelude::*;

glib::wrapper! {
    #[doc(alias = "GtkConstraintTarget")]
    pub struct ConstraintTarget(Interface<ffi::GtkConstraintTarget, ffi::GtkConstraintTargetInterface>);

    match fn {
        type_ => || ffi::gtk_constraint_target_get_type(),
    }
}

impl ConstraintTarget {
    pub const NONE: Option<&'static ConstraintTarget> = None;
}

pub trait ConstraintTargetExt: IsA<ConstraintTarget> + 'static {}

impl<O: IsA<ConstraintTarget>> ConstraintTargetExt for O {}
