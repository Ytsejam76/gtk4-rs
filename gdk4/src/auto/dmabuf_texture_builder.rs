// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v4_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
use crate::ColorState;
use crate::{ffi, Display, Texture};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GdkDmabufTextureBuilder")]
    pub struct DmabufTextureBuilder(Object<ffi::GdkDmabufTextureBuilder, ffi::GdkDmabufTextureBuilderClass>);

    match fn {
        type_ => || ffi::gdk_dmabuf_texture_builder_get_type(),
    }
}

impl DmabufTextureBuilder {
    #[doc(alias = "gdk_dmabuf_texture_builder_new")]
    pub fn new() -> DmabufTextureBuilder {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gdk_dmabuf_texture_builder_new()) }
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "gdk_dmabuf_texture_builder_get_color_state")]
    #[doc(alias = "get_color_state")]
    #[doc(alias = "color-state")]
    pub fn color_state(&self) -> Option<ColorState> {
        unsafe {
            from_glib_none(ffi::gdk_dmabuf_texture_builder_get_color_state(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_get_display")]
    #[doc(alias = "get_display")]
    pub fn display(&self) -> Display {
        unsafe {
            from_glib_none(ffi::gdk_dmabuf_texture_builder_get_display(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_get_fd")]
    #[doc(alias = "get_fd")]
    pub fn fd(&self, plane: u32) -> i32 {
        unsafe { ffi::gdk_dmabuf_texture_builder_get_fd(self.to_glib_none().0, plane) }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_get_fourcc")]
    #[doc(alias = "get_fourcc")]
    pub fn fourcc(&self) -> u32 {
        unsafe { ffi::gdk_dmabuf_texture_builder_get_fourcc(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_get_height")]
    #[doc(alias = "get_height")]
    pub fn height(&self) -> u32 {
        unsafe { ffi::gdk_dmabuf_texture_builder_get_height(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_get_modifier")]
    #[doc(alias = "get_modifier")]
    pub fn modifier(&self) -> u64 {
        unsafe { ffi::gdk_dmabuf_texture_builder_get_modifier(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_get_n_planes")]
    #[doc(alias = "get_n_planes")]
    #[doc(alias = "n-planes")]
    pub fn n_planes(&self) -> u32 {
        unsafe { ffi::gdk_dmabuf_texture_builder_get_n_planes(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_get_offset")]
    #[doc(alias = "get_offset")]
    pub fn offset(&self, plane: u32) -> u32 {
        unsafe { ffi::gdk_dmabuf_texture_builder_get_offset(self.to_glib_none().0, plane) }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_get_premultiplied")]
    #[doc(alias = "get_premultiplied")]
    #[doc(alias = "premultiplied")]
    pub fn is_premultiplied(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_dmabuf_texture_builder_get_premultiplied(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_get_stride")]
    #[doc(alias = "get_stride")]
    pub fn stride(&self, plane: u32) -> u32 {
        unsafe { ffi::gdk_dmabuf_texture_builder_get_stride(self.to_glib_none().0, plane) }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_get_update_region")]
    #[doc(alias = "get_update_region")]
    #[doc(alias = "update-region")]
    pub fn update_region(&self) -> Option<cairo::Region> {
        unsafe {
            from_glib_none(ffi::gdk_dmabuf_texture_builder_get_update_region(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_get_update_texture")]
    #[doc(alias = "get_update_texture")]
    #[doc(alias = "update-texture")]
    pub fn update_texture(&self) -> Option<Texture> {
        unsafe {
            from_glib_none(ffi::gdk_dmabuf_texture_builder_get_update_texture(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_get_width")]
    #[doc(alias = "get_width")]
    pub fn width(&self) -> u32 {
        unsafe { ffi::gdk_dmabuf_texture_builder_get_width(self.to_glib_none().0) }
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "gdk_dmabuf_texture_builder_set_color_state")]
    #[doc(alias = "color-state")]
    pub fn set_color_state(&self, color_state: Option<&ColorState>) {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_color_state(
                self.to_glib_none().0,
                color_state.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_display")]
    #[doc(alias = "display")]
    pub fn set_display(&self, display: &impl IsA<Display>) {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_display(
                self.to_glib_none().0,
                display.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_fd")]
    pub fn set_fd(&self, plane: u32, fd: i32) {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_fd(self.to_glib_none().0, plane, fd);
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_fourcc")]
    #[doc(alias = "fourcc")]
    pub fn set_fourcc(&self, fourcc: u32) {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_fourcc(self.to_glib_none().0, fourcc);
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_height")]
    #[doc(alias = "height")]
    pub fn set_height(&self, height: u32) {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_height(self.to_glib_none().0, height);
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_modifier")]
    #[doc(alias = "modifier")]
    pub fn set_modifier(&self, modifier: u64) {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_modifier(self.to_glib_none().0, modifier);
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_n_planes")]
    #[doc(alias = "n-planes")]
    pub fn set_n_planes(&self, n_planes: u32) {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_n_planes(self.to_glib_none().0, n_planes);
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_offset")]
    pub fn set_offset(&self, plane: u32, offset: u32) {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_offset(self.to_glib_none().0, plane, offset);
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_premultiplied")]
    #[doc(alias = "premultiplied")]
    pub fn set_premultiplied(&self, premultiplied: bool) {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_premultiplied(
                self.to_glib_none().0,
                premultiplied.into_glib(),
            );
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_stride")]
    pub fn set_stride(&self, plane: u32, stride: u32) {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_stride(self.to_glib_none().0, plane, stride);
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_update_region")]
    #[doc(alias = "update-region")]
    pub fn set_update_region(&self, region: Option<&cairo::Region>) {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_update_region(
                self.to_glib_none().0,
                mut_override(region.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_update_texture")]
    #[doc(alias = "update-texture")]
    pub fn set_update_texture(&self, texture: Option<&impl IsA<Texture>>) {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_update_texture(
                self.to_glib_none().0,
                texture.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_dmabuf_texture_builder_set_width")]
    #[doc(alias = "width")]
    pub fn set_width(&self, width: u32) {
        unsafe {
            ffi::gdk_dmabuf_texture_builder_set_width(self.to_glib_none().0, width);
        }
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "color-state")]
    pub fn connect_color_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_color_state_trampoline<
            F: Fn(&DmabufTextureBuilder) + 'static,
        >(
            this: *mut ffi::GdkDmabufTextureBuilder,
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
                c"notify::color-state".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_color_state_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "display")]
    pub fn connect_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_display_trampoline<F: Fn(&DmabufTextureBuilder) + 'static>(
            this: *mut ffi::GdkDmabufTextureBuilder,
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
                c"notify::display".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_display_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "fourcc")]
    pub fn connect_fourcc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fourcc_trampoline<F: Fn(&DmabufTextureBuilder) + 'static>(
            this: *mut ffi::GdkDmabufTextureBuilder,
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
                c"notify::fourcc".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_fourcc_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "height")]
    pub fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<F: Fn(&DmabufTextureBuilder) + 'static>(
            this: *mut ffi::GdkDmabufTextureBuilder,
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
                c"notify::height".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "modifier")]
    pub fn connect_modifier_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modifier_trampoline<F: Fn(&DmabufTextureBuilder) + 'static>(
            this: *mut ffi::GdkDmabufTextureBuilder,
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
                c"notify::modifier".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_modifier_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "n-planes")]
    pub fn connect_n_planes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_planes_trampoline<F: Fn(&DmabufTextureBuilder) + 'static>(
            this: *mut ffi::GdkDmabufTextureBuilder,
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
                c"notify::n-planes".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_n_planes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "premultiplied")]
    pub fn connect_premultiplied_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_premultiplied_trampoline<
            F: Fn(&DmabufTextureBuilder) + 'static,
        >(
            this: *mut ffi::GdkDmabufTextureBuilder,
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
                c"notify::premultiplied".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_premultiplied_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "update-region")]
    pub fn connect_update_region_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_update_region_trampoline<
            F: Fn(&DmabufTextureBuilder) + 'static,
        >(
            this: *mut ffi::GdkDmabufTextureBuilder,
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
                c"notify::update-region".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_update_region_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "update-texture")]
    pub fn connect_update_texture_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_update_texture_trampoline<
            F: Fn(&DmabufTextureBuilder) + 'static,
        >(
            this: *mut ffi::GdkDmabufTextureBuilder,
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
                c"notify::update-texture".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_update_texture_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "width")]
    pub fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<F: Fn(&DmabufTextureBuilder) + 'static>(
            this: *mut ffi::GdkDmabufTextureBuilder,
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
                c"notify::width".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
impl Default for DmabufTextureBuilder {
    fn default() -> Self {
        Self::new()
    }
}
