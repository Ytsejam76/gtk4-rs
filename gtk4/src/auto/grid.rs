// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Align, BaselinePosition, Buildable, ConstraintTarget,
    LayoutManager, Orientable, Orientation, Overflow, PositionType, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkGrid")]
    pub struct Grid(Object<ffi::GtkGrid, ffi::GtkGridClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Orientable;

    match fn {
        type_ => || ffi::gtk_grid_get_type(),
    }
}

impl Grid {
    pub const NONE: Option<&'static Grid> = None;

    #[doc(alias = "gtk_grid_new")]
    pub fn new() -> Grid {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_grid_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Grid`] objects.
    ///
    /// This method returns an instance of [`GridBuilder`](crate::builders::GridBuilder) which can be used to create [`Grid`] objects.
    pub fn builder() -> GridBuilder {
        GridBuilder::new()
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Grid`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GridBuilder {
    builder: glib::object::ObjectBuilder<'static, Grid>,
}

impl GridBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn baseline_row(self, baseline_row: i32) -> Self {
        Self {
            builder: self.builder.property("baseline-row", baseline_row),
        }
    }

    pub fn column_homogeneous(self, column_homogeneous: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("column-homogeneous", column_homogeneous),
        }
    }

    pub fn column_spacing(self, column_spacing: i32) -> Self {
        Self {
            builder: self.builder.property("column-spacing", column_spacing),
        }
    }

    pub fn row_homogeneous(self, row_homogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("row-homogeneous", row_homogeneous),
        }
    }

    pub fn row_spacing(self, row_spacing: i32) -> Self {
        Self {
            builder: self.builder.property("row-spacing", row_spacing),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Grid`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Grid {
        self.builder.build()
    }
}

pub trait GridExt: IsA<Grid> + 'static {
    #[doc(alias = "gtk_grid_attach")]
    fn attach(&self, child: &impl IsA<Widget>, column: i32, row: i32, width: i32, height: i32) {
        unsafe {
            ffi::gtk_grid_attach(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                column,
                row,
                width,
                height,
            );
        }
    }

    #[doc(alias = "gtk_grid_attach_next_to")]
    fn attach_next_to(
        &self,
        child: &impl IsA<Widget>,
        sibling: Option<&impl IsA<Widget>>,
        side: PositionType,
        width: i32,
        height: i32,
    ) {
        unsafe {
            ffi::gtk_grid_attach_next_to(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                sibling.map(|p| p.as_ref()).to_glib_none().0,
                side.into_glib(),
                width,
                height,
            );
        }
    }

    #[doc(alias = "gtk_grid_get_baseline_row")]
    #[doc(alias = "get_baseline_row")]
    #[doc(alias = "baseline-row")]
    fn baseline_row(&self) -> i32 {
        unsafe { ffi::gtk_grid_get_baseline_row(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gtk_grid_get_child_at")]
    #[doc(alias = "get_child_at")]
    fn child_at(&self, column: i32, row: i32) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_grid_get_child_at(
                self.as_ref().to_glib_none().0,
                column,
                row,
            ))
        }
    }

    #[doc(alias = "gtk_grid_get_column_homogeneous")]
    #[doc(alias = "get_column_homogeneous")]
    #[doc(alias = "column-homogeneous")]
    fn is_column_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_grid_get_column_homogeneous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_grid_get_column_spacing")]
    #[doc(alias = "get_column_spacing")]
    #[doc(alias = "column-spacing")]
    fn column_spacing(&self) -> u32 {
        unsafe { ffi::gtk_grid_get_column_spacing(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gtk_grid_get_row_baseline_position")]
    #[doc(alias = "get_row_baseline_position")]
    fn row_baseline_position(&self, row: i32) -> BaselinePosition {
        unsafe {
            from_glib(ffi::gtk_grid_get_row_baseline_position(
                self.as_ref().to_glib_none().0,
                row,
            ))
        }
    }

    #[doc(alias = "gtk_grid_get_row_homogeneous")]
    #[doc(alias = "get_row_homogeneous")]
    #[doc(alias = "row-homogeneous")]
    fn is_row_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_grid_get_row_homogeneous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_grid_get_row_spacing")]
    #[doc(alias = "get_row_spacing")]
    #[doc(alias = "row-spacing")]
    fn row_spacing(&self) -> u32 {
        unsafe { ffi::gtk_grid_get_row_spacing(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gtk_grid_insert_column")]
    fn insert_column(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_insert_column(self.as_ref().to_glib_none().0, position);
        }
    }

    #[doc(alias = "gtk_grid_insert_next_to")]
    fn insert_next_to(&self, sibling: &impl IsA<Widget>, side: PositionType) {
        unsafe {
            ffi::gtk_grid_insert_next_to(
                self.as_ref().to_glib_none().0,
                sibling.as_ref().to_glib_none().0,
                side.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_grid_insert_row")]
    fn insert_row(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_insert_row(self.as_ref().to_glib_none().0, position);
        }
    }

    #[doc(alias = "gtk_grid_query_child")]
    fn query_child(&self, child: &impl IsA<Widget>) -> (i32, i32, i32, i32) {
        unsafe {
            let mut column = std::mem::MaybeUninit::uninit();
            let mut row = std::mem::MaybeUninit::uninit();
            let mut width = std::mem::MaybeUninit::uninit();
            let mut height = std::mem::MaybeUninit::uninit();
            ffi::gtk_grid_query_child(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                column.as_mut_ptr(),
                row.as_mut_ptr(),
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (
                column.assume_init(),
                row.assume_init(),
                width.assume_init(),
                height.assume_init(),
            )
        }
    }

    #[doc(alias = "gtk_grid_remove")]
    fn remove(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_grid_remove(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_grid_remove_column")]
    fn remove_column(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_remove_column(self.as_ref().to_glib_none().0, position);
        }
    }

    #[doc(alias = "gtk_grid_remove_row")]
    fn remove_row(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_remove_row(self.as_ref().to_glib_none().0, position);
        }
    }

    #[doc(alias = "gtk_grid_set_baseline_row")]
    #[doc(alias = "baseline-row")]
    fn set_baseline_row(&self, row: i32) {
        unsafe {
            ffi::gtk_grid_set_baseline_row(self.as_ref().to_glib_none().0, row);
        }
    }

    #[doc(alias = "gtk_grid_set_column_homogeneous")]
    #[doc(alias = "column-homogeneous")]
    fn set_column_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_grid_set_column_homogeneous(
                self.as_ref().to_glib_none().0,
                homogeneous.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_grid_set_column_spacing")]
    #[doc(alias = "column-spacing")]
    fn set_column_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_grid_set_column_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    #[doc(alias = "gtk_grid_set_row_baseline_position")]
    fn set_row_baseline_position(&self, row: i32, pos: BaselinePosition) {
        unsafe {
            ffi::gtk_grid_set_row_baseline_position(
                self.as_ref().to_glib_none().0,
                row,
                pos.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_grid_set_row_homogeneous")]
    #[doc(alias = "row-homogeneous")]
    fn set_row_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_grid_set_row_homogeneous(
                self.as_ref().to_glib_none().0,
                homogeneous.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_grid_set_row_spacing")]
    #[doc(alias = "row-spacing")]
    fn set_row_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_grid_set_row_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    #[doc(alias = "baseline-row")]
    fn connect_baseline_row_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_baseline_row_trampoline<P: IsA<Grid>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGrid,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Grid::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::baseline-row\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_baseline_row_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "column-homogeneous")]
    fn connect_column_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_homogeneous_trampoline<
            P: IsA<Grid>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkGrid,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Grid::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::column-homogeneous\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_column_homogeneous_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "column-spacing")]
    fn connect_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_spacing_trampoline<P: IsA<Grid>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGrid,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Grid::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::column-spacing\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_column_spacing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "row-homogeneous")]
    fn connect_row_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_homogeneous_trampoline<
            P: IsA<Grid>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkGrid,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Grid::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::row-homogeneous\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_row_homogeneous_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "row-spacing")]
    fn connect_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_spacing_trampoline<P: IsA<Grid>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGrid,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Grid::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::row-spacing\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_row_spacing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Grid>> GridExt for O {}
