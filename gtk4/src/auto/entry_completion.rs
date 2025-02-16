// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, Buildable, CellArea, CellLayout, TreeIter, TreeModel};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkEntryCompletion")]
    pub struct EntryCompletion(Object<ffi::GtkEntryCompletion>) @implements Buildable, CellLayout;

    match fn {
        type_ => || ffi::gtk_entry_completion_get_type(),
    }
}

impl EntryCompletion {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_new")]
    pub fn new() -> EntryCompletion {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_entry_completion_new()) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_new_with_area")]
    #[doc(alias = "new_with_area")]
    pub fn with_area(area: &impl IsA<CellArea>) -> EntryCompletion {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_entry_completion_new_with_area(
                area.as_ref().to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`EntryCompletion`] objects.
    ///
    /// This method returns an instance of [`EntryCompletionBuilder`](crate::builders::EntryCompletionBuilder) which can be used to create [`EntryCompletion`] objects.
    pub fn builder() -> EntryCompletionBuilder {
        EntryCompletionBuilder::new()
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_complete")]
    pub fn complete(&self) {
        unsafe {
            ffi::gtk_entry_completion_complete(self.to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_compute_prefix")]
    pub fn compute_prefix(&self, key: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_entry_completion_compute_prefix(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_get_completion_prefix")]
    #[doc(alias = "get_completion_prefix")]
    pub fn completion_prefix(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_entry_completion_get_completion_prefix(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_get_inline_completion")]
    #[doc(alias = "get_inline_completion")]
    #[doc(alias = "inline-completion")]
    pub fn is_inline_completion(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_inline_completion(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_get_inline_selection")]
    #[doc(alias = "get_inline_selection")]
    #[doc(alias = "inline-selection")]
    pub fn is_inline_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_inline_selection(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_get_minimum_key_length")]
    #[doc(alias = "get_minimum_key_length")]
    #[doc(alias = "minimum-key-length")]
    pub fn minimum_key_length(&self) -> i32 {
        unsafe { ffi::gtk_entry_completion_get_minimum_key_length(self.to_glib_none().0) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> Option<TreeModel> {
        unsafe { from_glib_none(ffi::gtk_entry_completion_get_model(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_get_popup_completion")]
    #[doc(alias = "get_popup_completion")]
    #[doc(alias = "popup-completion")]
    pub fn is_popup_completion(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_completion(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_get_popup_set_width")]
    #[doc(alias = "get_popup_set_width")]
    #[doc(alias = "popup-set-width")]
    pub fn is_popup_set_width(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_set_width(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_get_popup_single_match")]
    #[doc(alias = "get_popup_single_match")]
    #[doc(alias = "popup-single-match")]
    pub fn is_popup_single_match(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_single_match(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_get_text_column")]
    #[doc(alias = "get_text_column")]
    #[doc(alias = "text-column")]
    pub fn text_column(&self) -> i32 {
        unsafe { ffi::gtk_entry_completion_get_text_column(self.to_glib_none().0) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_insert_prefix")]
    pub fn insert_prefix(&self) {
        unsafe {
            ffi::gtk_entry_completion_insert_prefix(self.to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_set_inline_completion")]
    #[doc(alias = "inline-completion")]
    pub fn set_inline_completion(&self, inline_completion: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_inline_completion(
                self.to_glib_none().0,
                inline_completion.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_set_inline_selection")]
    #[doc(alias = "inline-selection")]
    pub fn set_inline_selection(&self, inline_selection: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_inline_selection(
                self.to_glib_none().0,
                inline_selection.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_set_match_func")]
    pub fn set_match_func<P: Fn(&EntryCompletion, &str, &TreeIter) -> bool + 'static>(
        &self,
        func: P,
    ) {
        let func_data: Box_<P> = Box_::new(func);
        unsafe extern "C" fn func_func<
            P: Fn(&EntryCompletion, &str, &TreeIter) -> bool + 'static,
        >(
            completion: *mut ffi::GtkEntryCompletion,
            key: *const std::ffi::c_char,
            iter: *mut ffi::GtkTreeIter,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let completion = from_glib_borrow(completion);
            let key: Borrowed<glib::GString> = from_glib_borrow(key);
            let iter = from_glib_borrow(iter);
            let callback = &*(user_data as *mut P);
            (*callback)(&completion, key.as_str(), &iter).into_glib()
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn func_notify_func<
            P: Fn(&EntryCompletion, &str, &TreeIter) -> bool + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback = Box_::from_raw(data as *mut P);
        }
        let destroy_call3 = Some(func_notify_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            ffi::gtk_entry_completion_set_match_func(
                self.to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_set_minimum_key_length")]
    #[doc(alias = "minimum-key-length")]
    pub fn set_minimum_key_length(&self, length: i32) {
        unsafe {
            ffi::gtk_entry_completion_set_minimum_key_length(self.to_glib_none().0, length);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_set_model")]
    #[doc(alias = "model")]
    pub fn set_model(&self, model: Option<&impl IsA<TreeModel>>) {
        unsafe {
            ffi::gtk_entry_completion_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_set_popup_completion")]
    #[doc(alias = "popup-completion")]
    pub fn set_popup_completion(&self, popup_completion: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_completion(
                self.to_glib_none().0,
                popup_completion.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_set_popup_set_width")]
    #[doc(alias = "popup-set-width")]
    pub fn set_popup_set_width(&self, popup_set_width: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_set_width(
                self.to_glib_none().0,
                popup_set_width.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_set_popup_single_match")]
    #[doc(alias = "popup-single-match")]
    pub fn set_popup_single_match(&self, popup_single_match: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_single_match(
                self.to_glib_none().0,
                popup_single_match.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_entry_completion_set_text_column")]
    #[doc(alias = "text-column")]
    pub fn set_text_column(&self, column: i32) {
        unsafe {
            ffi::gtk_entry_completion_set_text_column(self.to_glib_none().0, column);
        }
    }

    #[doc(alias = "cell-area")]
    pub fn cell_area(&self) -> Option<CellArea> {
        ObjectExt::property(self, "cell-area")
    }

    #[doc(alias = "cursor-on-match")]
    pub fn connect_cursor_on_match<
        F: Fn(&Self, &TreeModel, &TreeIter) -> glib::Propagation + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cursor_on_match_trampoline<
            F: Fn(&EntryCompletion, &TreeModel, &TreeIter) -> glib::Propagation + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
            model: *mut ffi::GtkTreeModel,
            iter: *mut ffi::GtkTreeIter,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(model),
                &from_glib_borrow(iter),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"cursor-on-match".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    cursor_on_match_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "insert-prefix")]
    pub fn connect_insert_prefix<F: Fn(&Self, &str) -> glib::Propagation + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn insert_prefix_trampoline<
            F: Fn(&EntryCompletion, &str) -> glib::Propagation + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
            prefix: *mut std::ffi::c_char,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(prefix),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"insert-prefix".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    insert_prefix_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "match-selected")]
    pub fn connect_match_selected<
        F: Fn(&Self, &TreeModel, &TreeIter) -> glib::Propagation + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn match_selected_trampoline<
            F: Fn(&EntryCompletion, &TreeModel, &TreeIter) -> glib::Propagation + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
            model: *mut ffi::GtkTreeModel,
            iter: *mut ffi::GtkTreeIter,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(model),
                &from_glib_borrow(iter),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"match-selected".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    match_selected_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "no-matches")]
    pub fn connect_no_matches<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn no_matches_trampoline<F: Fn(&EntryCompletion) + 'static>(
            this: *mut ffi::GtkEntryCompletion,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"no-matches".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    no_matches_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "inline-completion")]
    pub fn connect_inline_completion_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_inline_completion_trampoline<
            F: Fn(&EntryCompletion) + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
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
                c"notify::inline-completion".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_inline_completion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "inline-selection")]
    pub fn connect_inline_selection_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inline_selection_trampoline<
            F: Fn(&EntryCompletion) + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
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
                c"notify::inline-selection".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_inline_selection_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "minimum-key-length")]
    pub fn connect_minimum_key_length_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_minimum_key_length_trampoline<
            F: Fn(&EntryCompletion) + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
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
                c"notify::minimum-key-length".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_minimum_key_length_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "model")]
    pub fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&EntryCompletion) + 'static>(
            this: *mut ffi::GtkEntryCompletion,
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
                c"notify::model".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "popup-completion")]
    pub fn connect_popup_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_popup_completion_trampoline<
            F: Fn(&EntryCompletion) + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
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
                c"notify::popup-completion".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_popup_completion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "popup-set-width")]
    pub fn connect_popup_set_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_popup_set_width_trampoline<
            F: Fn(&EntryCompletion) + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
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
                c"notify::popup-set-width".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_popup_set_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "popup-single-match")]
    pub fn connect_popup_single_match_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_popup_single_match_trampoline<
            F: Fn(&EntryCompletion) + 'static,
        >(
            this: *mut ffi::GtkEntryCompletion,
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
                c"notify::popup-single-match".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_popup_single_match_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "text-column")]
    pub fn connect_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_column_trampoline<F: Fn(&EntryCompletion) + 'static>(
            this: *mut ffi::GtkEntryCompletion,
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
                c"notify::text-column".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_text_column_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for EntryCompletion {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`EntryCompletion`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct EntryCompletionBuilder {
    builder: glib::object::ObjectBuilder<'static, EntryCompletion>,
}

impl EntryCompletionBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn cell_area(self, cell_area: &impl IsA<CellArea>) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-area", cell_area.clone().upcast()),
        }
    }

    pub fn inline_completion(self, inline_completion: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("inline-completion", inline_completion),
        }
    }

    pub fn inline_selection(self, inline_selection: bool) -> Self {
        Self {
            builder: self.builder.property("inline-selection", inline_selection),
        }
    }

    pub fn minimum_key_length(self, minimum_key_length: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("minimum-key-length", minimum_key_length),
        }
    }

    pub fn model(self, model: &impl IsA<TreeModel>) -> Self {
        Self {
            builder: self.builder.property("model", model.clone().upcast()),
        }
    }

    pub fn popup_completion(self, popup_completion: bool) -> Self {
        Self {
            builder: self.builder.property("popup-completion", popup_completion),
        }
    }

    pub fn popup_set_width(self, popup_set_width: bool) -> Self {
        Self {
            builder: self.builder.property("popup-set-width", popup_set_width),
        }
    }

    pub fn popup_single_match(self, popup_single_match: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("popup-single-match", popup_single_match),
        }
    }

    pub fn text_column(self, text_column: i32) -> Self {
        Self {
            builder: self.builder.property("text-column", text_column),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`EntryCompletion`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> EntryCompletion {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
