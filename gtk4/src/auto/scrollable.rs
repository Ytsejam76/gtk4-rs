// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Adjustment, Border, ScrollablePolicy};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkScrollable")]
    pub struct Scrollable(Interface<ffi::GtkScrollable, ffi::GtkScrollableInterface>);

    match fn {
        type_ => || ffi::gtk_scrollable_get_type(),
    }
}

impl Scrollable {
    pub const NONE: Option<&'static Scrollable> = None;
}

pub trait ScrollableExt: IsA<Scrollable> + 'static {
    #[doc(alias = "gtk_scrollable_get_border")]
    #[doc(alias = "get_border")]
    fn border(&self) -> Option<Border> {
        unsafe {
            let mut border = Border::uninitialized();
            let ret = from_glib(ffi::gtk_scrollable_get_border(
                self.as_ref().to_glib_none().0,
                border.to_glib_none_mut().0,
            ));
            if ret {
                Some(border)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_scrollable_get_hadjustment")]
    #[doc(alias = "get_hadjustment")]
    fn hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrollable_get_hadjustment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scrollable_get_hscroll_policy")]
    #[doc(alias = "get_hscroll_policy")]
    #[doc(alias = "hscroll-policy")]
    fn hscroll_policy(&self) -> ScrollablePolicy {
        unsafe {
            from_glib(ffi::gtk_scrollable_get_hscroll_policy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scrollable_get_vadjustment")]
    #[doc(alias = "get_vadjustment")]
    fn vadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrollable_get_vadjustment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scrollable_get_vscroll_policy")]
    #[doc(alias = "get_vscroll_policy")]
    #[doc(alias = "vscroll-policy")]
    fn vscroll_policy(&self) -> ScrollablePolicy {
        unsafe {
            from_glib(ffi::gtk_scrollable_get_vscroll_policy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scrollable_set_hadjustment")]
    #[doc(alias = "hadjustment")]
    fn set_hadjustment(&self, hadjustment: Option<&impl IsA<Adjustment>>) {
        unsafe {
            ffi::gtk_scrollable_set_hadjustment(
                self.as_ref().to_glib_none().0,
                hadjustment.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_scrollable_set_hscroll_policy")]
    #[doc(alias = "hscroll-policy")]
    fn set_hscroll_policy(&self, policy: ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_hscroll_policy(
                self.as_ref().to_glib_none().0,
                policy.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_scrollable_set_vadjustment")]
    #[doc(alias = "vadjustment")]
    fn set_vadjustment(&self, vadjustment: Option<&impl IsA<Adjustment>>) {
        unsafe {
            ffi::gtk_scrollable_set_vadjustment(
                self.as_ref().to_glib_none().0,
                vadjustment.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_scrollable_set_vscroll_policy")]
    #[doc(alias = "vscroll-policy")]
    fn set_vscroll_policy(&self, policy: ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_vscroll_policy(
                self.as_ref().to_glib_none().0,
                policy.into_glib(),
            );
        }
    }

    #[doc(alias = "hadjustment")]
    fn connect_hadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hadjustment_trampoline<
            P: IsA<Scrollable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkScrollable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Scrollable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::hadjustment\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_hadjustment_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "hscroll-policy")]
    fn connect_hscroll_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hscroll_policy_trampoline<
            P: IsA<Scrollable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkScrollable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Scrollable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::hscroll-policy\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_hscroll_policy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "vadjustment")]
    fn connect_vadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vadjustment_trampoline<
            P: IsA<Scrollable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkScrollable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Scrollable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::vadjustment\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_vadjustment_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "vscroll-policy")]
    fn connect_vscroll_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vscroll_policy_trampoline<
            P: IsA<Scrollable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkScrollable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Scrollable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::vscroll-policy\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_vscroll_policy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Scrollable>> ScrollableExt for O {}
