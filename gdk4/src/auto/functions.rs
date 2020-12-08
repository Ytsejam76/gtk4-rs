// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Event;
use crate::Texture;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

#[doc(alias = "gdk_events_get_angle")]
pub fn events_get_angle<P: IsA<Event>, Q: IsA<Event>>(event1: &P, event2: &Q) -> Option<f64> {
    skip_assert_initialized!();
    unsafe {
        let mut angle = mem::MaybeUninit::uninit();
        let ret = from_glib(ffi::gdk_events_get_angle(
            event1.as_ref().to_glib_none().0,
            event2.as_ref().to_glib_none().0,
            angle.as_mut_ptr(),
        ));
        let angle = angle.assume_init();
        if ret {
            Some(angle)
        } else {
            None
        }
    }
}

#[doc(alias = "gdk_events_get_center")]
pub fn events_get_center<P: IsA<Event>, Q: IsA<Event>>(
    event1: &P,
    event2: &Q,
) -> Option<(f64, f64)> {
    skip_assert_initialized!();
    unsafe {
        let mut x = mem::MaybeUninit::uninit();
        let mut y = mem::MaybeUninit::uninit();
        let ret = from_glib(ffi::gdk_events_get_center(
            event1.as_ref().to_glib_none().0,
            event2.as_ref().to_glib_none().0,
            x.as_mut_ptr(),
            y.as_mut_ptr(),
        ));
        let x = x.assume_init();
        let y = y.assume_init();
        if ret {
            Some((x, y))
        } else {
            None
        }
    }
}

#[doc(alias = "gdk_events_get_distance")]
pub fn events_get_distance<P: IsA<Event>, Q: IsA<Event>>(event1: &P, event2: &Q) -> Option<f64> {
    skip_assert_initialized!();
    unsafe {
        let mut distance = mem::MaybeUninit::uninit();
        let ret = from_glib(ffi::gdk_events_get_distance(
            event1.as_ref().to_glib_none().0,
            event2.as_ref().to_glib_none().0,
            distance.as_mut_ptr(),
        ));
        let distance = distance.assume_init();
        if ret {
            Some(distance)
        } else {
            None
        }
    }
}

#[doc(alias = "gdk_intern_mime_type")]
pub fn intern_mime_type(string: &str) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gdk_intern_mime_type(string.to_glib_none().0)) }
}

#[doc(alias = "gdk_keyval_convert_case")]
pub fn keyval_convert_case(symbol: u32) -> (u32, u32) {
    assert_initialized_main_thread!();
    unsafe {
        let mut lower = mem::MaybeUninit::uninit();
        let mut upper = mem::MaybeUninit::uninit();
        ffi::gdk_keyval_convert_case(symbol, lower.as_mut_ptr(), upper.as_mut_ptr());
        let lower = lower.assume_init();
        let upper = upper.assume_init();
        (lower, upper)
    }
}

#[doc(alias = "gdk_keyval_from_name")]
pub fn keyval_from_name(keyval_name: &str) -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_keyval_from_name(keyval_name.to_glib_none().0) }
}

#[doc(alias = "gdk_keyval_is_lower")]
pub fn keyval_is_lower(keyval: u32) -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gdk_keyval_is_lower(keyval)) }
}

#[doc(alias = "gdk_keyval_is_upper")]
pub fn keyval_is_upper(keyval: u32) -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gdk_keyval_is_upper(keyval)) }
}

#[doc(alias = "gdk_keyval_name")]
pub fn keyval_name(keyval: u32) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gdk_keyval_name(keyval)) }
}

#[doc(alias = "gdk_keyval_to_lower")]
pub fn keyval_to_lower(keyval: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_keyval_to_lower(keyval) }
}

#[doc(alias = "gdk_keyval_to_unicode")]
pub fn keyval_to_unicode(keyval: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_keyval_to_unicode(keyval) }
}

#[doc(alias = "gdk_keyval_to_upper")]
pub fn keyval_to_upper(keyval: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_keyval_to_upper(keyval) }
}

#[doc(alias = "gdk_pixbuf_get_from_surface")]
pub fn pixbuf_get_from_surface(
    surface: &cairo::Surface,
    src_x: i32,
    src_y: i32,
    width: i32,
    height: i32,
) -> Option<gdk_pixbuf::Pixbuf> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gdk_pixbuf_get_from_surface(
            mut_override(surface.to_glib_none().0),
            src_x,
            src_y,
            width,
            height,
        ))
    }
}

#[doc(alias = "gdk_pixbuf_get_from_texture")]
pub fn pixbuf_get_from_texture<P: IsA<Texture>>(texture: &P) -> Option<gdk_pixbuf::Pixbuf> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gdk_pixbuf_get_from_texture(
            texture.as_ref().to_glib_none().0,
        ))
    }
}

#[doc(alias = "gdk_set_allowed_backends")]
pub fn set_allowed_backends(backends: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_set_allowed_backends(backends.to_glib_none().0);
    }
}

#[doc(alias = "gdk_toplevel_size_get_type")]
pub fn toplevel_size_get_type() -> glib::types::Type {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gdk_toplevel_size_get_type()) }
}

#[doc(alias = "gdk_unicode_to_keyval")]
pub fn unicode_to_keyval(wc: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_unicode_to_keyval(wc) }
}
