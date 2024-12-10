// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, BaselinePosition, LayoutManager, Orientable, Orientation};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkBoxLayout")]
    pub struct BoxLayout(Object<ffi::GtkBoxLayout, ffi::GtkBoxLayoutClass>) @extends LayoutManager, @implements Orientable;

    match fn {
        type_ => || ffi::gtk_box_layout_get_type(),
    }
}

impl BoxLayout {
    #[doc(alias = "gtk_box_layout_new")]
    pub fn new(orientation: Orientation) -> BoxLayout {
        assert_initialized_main_thread!();
        unsafe {
            LayoutManager::from_glib_full(ffi::gtk_box_layout_new(orientation.into_glib()))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`BoxLayout`] objects.
    ///
    /// This method returns an instance of [`BoxLayoutBuilder`](crate::builders::BoxLayoutBuilder) which can be used to create [`BoxLayout`] objects.
    pub fn builder() -> BoxLayoutBuilder {
        BoxLayoutBuilder::new()
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_box_layout_get_baseline_child")]
    #[doc(alias = "get_baseline_child")]
    #[doc(alias = "baseline-child")]
    pub fn baseline_child(&self) -> i32 {
        unsafe { ffi::gtk_box_layout_get_baseline_child(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_box_layout_get_baseline_position")]
    #[doc(alias = "get_baseline_position")]
    #[doc(alias = "baseline-position")]
    pub fn baseline_position(&self) -> BaselinePosition {
        unsafe {
            from_glib(ffi::gtk_box_layout_get_baseline_position(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_box_layout_get_homogeneous")]
    #[doc(alias = "get_homogeneous")]
    #[doc(alias = "homogeneous")]
    pub fn is_homogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_box_layout_get_homogeneous(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_box_layout_get_spacing")]
    #[doc(alias = "get_spacing")]
    pub fn spacing(&self) -> u32 {
        unsafe { ffi::gtk_box_layout_get_spacing(self.to_glib_none().0) }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_box_layout_set_baseline_child")]
    #[doc(alias = "baseline-child")]
    pub fn set_baseline_child(&self, child: i32) {
        unsafe {
            ffi::gtk_box_layout_set_baseline_child(self.to_glib_none().0, child);
        }
    }

    #[doc(alias = "gtk_box_layout_set_baseline_position")]
    #[doc(alias = "baseline-position")]
    pub fn set_baseline_position(&self, position: BaselinePosition) {
        unsafe {
            ffi::gtk_box_layout_set_baseline_position(self.to_glib_none().0, position.into_glib());
        }
    }

    #[doc(alias = "gtk_box_layout_set_homogeneous")]
    #[doc(alias = "homogeneous")]
    pub fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_box_layout_set_homogeneous(self.to_glib_none().0, homogeneous.into_glib());
        }
    }

    #[doc(alias = "gtk_box_layout_set_spacing")]
    #[doc(alias = "spacing")]
    pub fn set_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_box_layout_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "baseline-child")]
    pub fn connect_baseline_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_baseline_child_trampoline<F: Fn(&BoxLayout) + 'static>(
            this: *mut ffi::GtkBoxLayout,
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
                c"notify::baseline-child".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_baseline_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "baseline-position")]
    pub fn connect_baseline_position_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_baseline_position_trampoline<F: Fn(&BoxLayout) + 'static>(
            this: *mut ffi::GtkBoxLayout,
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
                c"notify::baseline-position".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_baseline_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "homogeneous")]
    pub fn connect_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_homogeneous_trampoline<F: Fn(&BoxLayout) + 'static>(
            this: *mut ffi::GtkBoxLayout,
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
                c"notify::homogeneous".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_homogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "spacing")]
    pub fn connect_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<F: Fn(&BoxLayout) + 'static>(
            this: *mut ffi::GtkBoxLayout,
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
                c"notify::spacing".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for BoxLayout {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`BoxLayout`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BoxLayoutBuilder {
    builder: glib::object::ObjectBuilder<'static, BoxLayout>,
}

impl BoxLayoutBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    pub fn baseline_child(self, baseline_child: i32) -> Self {
        Self {
            builder: self.builder.property("baseline-child", baseline_child),
        }
    }

    pub fn baseline_position(self, baseline_position: BaselinePosition) -> Self {
        Self {
            builder: self
                .builder
                .property("baseline-position", baseline_position),
        }
    }

    pub fn homogeneous(self, homogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("homogeneous", homogeneous),
        }
    }

    pub fn spacing(self, spacing: i32) -> Self {
        Self {
            builder: self.builder.property("spacing", spacing),
        }
    }

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`BoxLayout`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> BoxLayout {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
