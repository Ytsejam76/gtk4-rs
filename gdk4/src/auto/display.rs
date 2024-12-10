// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
use crate::DmabufFormats;
#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
use crate::GLContext;
use crate::{ffi, AppLaunchContext, Clipboard, Device, Event, Monitor, Seat, Surface};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GdkDisplay")]
    pub struct Display(Object<ffi::GdkDisplay>);

    match fn {
        type_ => || ffi::gdk_display_get_type(),
    }
}

impl Display {
    pub const NONE: Option<&'static Display> = None;

    #[doc(alias = "gdk_display_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Option<Display> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gdk_display_get_default()) }
    }

    #[doc(alias = "gdk_display_open")]
    pub fn open(display_name: Option<&str>) -> Option<Display> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gdk_display_open(display_name.to_glib_none().0)) }
    }
}

impl std::fmt::Display for Display {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&DisplayExt::name(self))
    }
}

pub trait DisplayExt: IsA<Display> + 'static {
    #[doc(alias = "gdk_display_beep")]
    fn beep(&self) {
        unsafe {
            ffi::gdk_display_beep(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_display_close")]
    fn close(&self) {
        unsafe {
            ffi::gdk_display_close(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gdk_display_create_gl_context")]
    fn create_gl_context(&self) -> Result<GLContext, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret =
                ffi::gdk_display_create_gl_context(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_display_device_is_grabbed")]
    fn device_is_grabbed(&self, device: &impl IsA<Device>) -> bool {
        unsafe {
            from_glib(ffi::gdk_display_device_is_grabbed(
                self.as_ref().to_glib_none().0,
                device.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_flush")]
    fn flush(&self) {
        unsafe {
            ffi::gdk_display_flush(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_display_get_app_launch_context")]
    #[doc(alias = "get_app_launch_context")]
    fn app_launch_context(&self) -> AppLaunchContext {
        unsafe {
            from_glib_full(ffi::gdk_display_get_app_launch_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_get_clipboard")]
    #[doc(alias = "get_clipboard")]
    fn clipboard(&self) -> Clipboard {
        unsafe {
            from_glib_none(ffi::gdk_display_get_clipboard(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_get_default_seat")]
    #[doc(alias = "get_default_seat")]
    fn default_seat(&self) -> Option<Seat> {
        unsafe {
            from_glib_none(ffi::gdk_display_get_default_seat(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gdk_display_get_dmabuf_formats")]
    #[doc(alias = "get_dmabuf_formats")]
    #[doc(alias = "dmabuf-formats")]
    fn dmabuf_formats(&self) -> DmabufFormats {
        unsafe {
            from_glib_none(ffi::gdk_display_get_dmabuf_formats(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_get_monitor_at_surface")]
    #[doc(alias = "get_monitor_at_surface")]
    fn monitor_at_surface(&self, surface: &impl IsA<Surface>) -> Option<Monitor> {
        unsafe {
            from_glib_none(ffi::gdk_display_get_monitor_at_surface(
                self.as_ref().to_glib_none().0,
                surface.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_get_monitors")]
    #[doc(alias = "get_monitors")]
    fn monitors(&self) -> gio::ListModel {
        unsafe {
            from_glib_none(ffi::gdk_display_get_monitors(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gdk_display_get_name(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_display_get_primary_clipboard")]
    #[doc(alias = "get_primary_clipboard")]
    fn primary_clipboard(&self) -> Clipboard {
        unsafe {
            from_glib_none(ffi::gdk_display_get_primary_clipboard(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gdk_display_get_startup_notification_id")]
    #[doc(alias = "get_startup_notification_id")]
    fn startup_notification_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gdk_display_get_startup_notification_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_is_closed")]
    fn is_closed(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_is_closed(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_display_is_composited")]
    #[doc(alias = "composited")]
    fn is_composited(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_display_is_composited(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_is_rgba")]
    #[doc(alias = "rgba")]
    fn is_rgba(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_is_rgba(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_display_list_seats")]
    fn list_seats(&self) -> Vec<Seat> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_display_list_seats(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gdk_display_notify_startup_complete")]
    fn notify_startup_complete(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_display_notify_startup_complete(
                self.as_ref().to_glib_none().0,
                startup_id.to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gdk_display_prepare_gl")]
    fn prepare_gl(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gdk_display_prepare_gl(self.as_ref().to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gdk_display_put_event")]
    fn put_event(&self, event: impl AsRef<Event>) {
        unsafe {
            ffi::gdk_display_put_event(
                self.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_display_supports_input_shapes")]
    #[doc(alias = "input-shapes")]
    fn supports_input_shapes(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_display_supports_input_shapes(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gdk_display_supports_shadow_width")]
    #[doc(alias = "shadow-width")]
    fn supports_shadow_width(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_display_supports_shadow_width(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_sync")]
    fn sync(&self) {
        unsafe {
            ffi::gdk_display_sync(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "closed")]
    fn connect_closed<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<P: IsA<Display>, F: Fn(&P, bool) + 'static>(
            this: *mut ffi::GdkDisplay,
            is_error: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Display::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(is_error),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"closed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    closed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "opened")]
    fn connect_opened<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn opened_trampoline<P: IsA<Display>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkDisplay,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Display::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"opened".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    opened_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "seat-added")]
    fn connect_seat_added<F: Fn(&Self, &Seat) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn seat_added_trampoline<P: IsA<Display>, F: Fn(&P, &Seat) + 'static>(
            this: *mut ffi::GdkDisplay,
            seat: *mut ffi::GdkSeat,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Display::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(seat),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"seat-added".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    seat_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "seat-removed")]
    fn connect_seat_removed<F: Fn(&Self, &Seat) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn seat_removed_trampoline<
            P: IsA<Display>,
            F: Fn(&P, &Seat) + 'static,
        >(
            this: *mut ffi::GdkDisplay,
            seat: *mut ffi::GdkSeat,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Display::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(seat),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"seat-removed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    seat_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "setting-changed")]
    fn connect_setting_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn setting_changed_trampoline<
            P: IsA<Display>,
            F: Fn(&P, &str) + 'static,
        >(
            this: *mut ffi::GdkDisplay,
            setting: *mut std::ffi::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Display::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(setting),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"setting-changed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    setting_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "composited")]
    fn connect_composited_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_composited_trampoline<P: IsA<Display>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkDisplay,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Display::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::composited".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_composited_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "dmabuf-formats")]
    fn connect_dmabuf_formats_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dmabuf_formats_trampoline<
            P: IsA<Display>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkDisplay,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Display::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::dmabuf-formats".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_dmabuf_formats_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "input-shapes")]
    fn connect_input_shapes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_shapes_trampoline<
            P: IsA<Display>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkDisplay,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Display::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::input-shapes".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_input_shapes_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "rgba")]
    fn connect_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rgba_trampoline<P: IsA<Display>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkDisplay,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Display::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::rgba".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_rgba_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "shadow-width")]
    fn connect_shadow_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shadow_width_trampoline<
            P: IsA<Display>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkDisplay,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Display::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::shadow-width".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_shadow_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Display>> DisplayExt for O {}
