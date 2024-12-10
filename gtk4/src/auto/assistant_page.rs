// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, AssistantPageType, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkAssistantPage")]
    pub struct AssistantPage(Object<ffi::GtkAssistantPage>);

    match fn {
        type_ => || ffi::gtk_assistant_page_get_type(),
    }
}

impl AssistantPage {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_assistant_page_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Widget {
        unsafe { from_glib_none(ffi::gtk_assistant_page_get_child(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn is_complete(&self) -> bool {
        ObjectExt::property(self, "complete")
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn set_complete(&self, complete: bool) {
        ObjectExt::set_property(self, "complete", complete)
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "page-type")]
    pub fn page_type(&self) -> AssistantPageType {
        ObjectExt::property(self, "page-type")
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "page-type")]
    pub fn set_page_type(&self, page_type: AssistantPageType) {
        ObjectExt::set_property(self, "page-type", page_type)
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn title(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "title")
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn set_title(&self, title: Option<&str>) {
        ObjectExt::set_property(self, "title", title)
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "complete")]
    pub fn connect_complete_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_complete_trampoline<F: Fn(&AssistantPage) + 'static>(
            this: *mut ffi::GtkAssistantPage,
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
                c"notify::complete".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_complete_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "page-type")]
    pub fn connect_page_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_type_trampoline<F: Fn(&AssistantPage) + 'static>(
            this: *mut ffi::GtkAssistantPage,
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
                c"notify::page-type".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_page_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&AssistantPage) + 'static>(
            this: *mut ffi::GtkAssistantPage,
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
                c"notify::title".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
