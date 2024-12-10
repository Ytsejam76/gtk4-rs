// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager, Overflow,
    Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkGLArea")]
    pub struct GLArea(Object<ffi::GtkGLArea, ffi::GtkGLAreaClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_gl_area_get_type(),
    }
}

impl GLArea {
    pub const NONE: Option<&'static GLArea> = None;

    #[doc(alias = "gtk_gl_area_new")]
    pub fn new() -> GLArea {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_gl_area_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`GLArea`] objects.
    ///
    /// This method returns an instance of [`GLAreaBuilder`](crate::builders::GLAreaBuilder) which can be used to create [`GLArea`] objects.
    pub fn builder() -> GLAreaBuilder {
        GLAreaBuilder::new()
    }
}

impl Default for GLArea {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`GLArea`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GLAreaBuilder {
    builder: glib::object::ObjectBuilder<'static, GLArea>,
}

impl GLAreaBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    pub fn allowed_apis(self, allowed_apis: gdk::GLAPI) -> Self {
        Self {
            builder: self.builder.property("allowed-apis", allowed_apis),
        }
    }

    pub fn auto_render(self, auto_render: bool) -> Self {
        Self {
            builder: self.builder.property("auto-render", auto_render),
        }
    }

    pub fn has_depth_buffer(self, has_depth_buffer: bool) -> Self {
        Self {
            builder: self.builder.property("has-depth-buffer", has_depth_buffer),
        }
    }

    pub fn has_stencil_buffer(self, has_stencil_buffer: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("has-stencil-buffer", has_stencil_buffer),
        }
    }

    #[cfg_attr(feature = "v4_12", deprecated = "Since 4.12")]
    pub fn use_es(self, use_es: bool) -> Self {
        Self {
            builder: self.builder.property("use-es", use_es),
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

    // rustdoc-stripper-ignore-next
    /// Build the [`GLArea`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> GLArea {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}

pub trait GLAreaExt: IsA<GLArea> + 'static {
    #[doc(alias = "gtk_gl_area_attach_buffers")]
    fn attach_buffers(&self) {
        unsafe {
            ffi::gtk_gl_area_attach_buffers(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_gl_area_get_allowed_apis")]
    #[doc(alias = "get_allowed_apis")]
    #[doc(alias = "allowed-apis")]
    fn allowed_apis(&self) -> gdk::GLAPI {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_allowed_apis(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_gl_area_get_api")]
    #[doc(alias = "get_api")]
    fn api(&self) -> gdk::GLAPI {
        unsafe { from_glib(ffi::gtk_gl_area_get_api(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_gl_area_get_auto_render")]
    #[doc(alias = "get_auto_render")]
    #[doc(alias = "auto-render")]
    fn is_auto_render(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_auto_render(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_gl_area_get_context")]
    #[doc(alias = "get_context")]
    fn context(&self) -> Option<gdk::GLContext> {
        unsafe { from_glib_none(ffi::gtk_gl_area_get_context(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_gl_area_get_error")]
    #[doc(alias = "get_error")]
    fn error(&self) -> Option<glib::Error> {
        unsafe { from_glib_none(ffi::gtk_gl_area_get_error(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_gl_area_get_has_depth_buffer")]
    #[doc(alias = "get_has_depth_buffer")]
    #[doc(alias = "has-depth-buffer")]
    fn has_depth_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_depth_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_gl_area_get_has_stencil_buffer")]
    #[doc(alias = "get_has_stencil_buffer")]
    #[doc(alias = "has-stencil-buffer")]
    fn has_stencil_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_stencil_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_gl_area_get_required_version")]
    #[doc(alias = "get_required_version")]
    fn required_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = std::mem::MaybeUninit::uninit();
            let mut minor = std::mem::MaybeUninit::uninit();
            ffi::gtk_gl_area_get_required_version(
                self.as_ref().to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            );
            (major.assume_init(), minor.assume_init())
        }
    }

    #[cfg_attr(feature = "v4_12", deprecated = "Since 4.12")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_gl_area_get_use_es")]
    #[doc(alias = "get_use_es")]
    #[doc(alias = "use-es")]
    fn uses_es(&self) -> bool {
        unsafe { from_glib(ffi::gtk_gl_area_get_use_es(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_gl_area_make_current")]
    fn make_current(&self) {
        unsafe {
            ffi::gtk_gl_area_make_current(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_gl_area_queue_render")]
    fn queue_render(&self) {
        unsafe {
            ffi::gtk_gl_area_queue_render(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_gl_area_set_allowed_apis")]
    #[doc(alias = "allowed-apis")]
    fn set_allowed_apis(&self, apis: gdk::GLAPI) {
        unsafe {
            ffi::gtk_gl_area_set_allowed_apis(self.as_ref().to_glib_none().0, apis.into_glib());
        }
    }

    #[doc(alias = "gtk_gl_area_set_auto_render")]
    #[doc(alias = "auto-render")]
    fn set_auto_render(&self, auto_render: bool) {
        unsafe {
            ffi::gtk_gl_area_set_auto_render(
                self.as_ref().to_glib_none().0,
                auto_render.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_gl_area_set_error")]
    fn set_error(&self, error: Option<&glib::Error>) {
        unsafe {
            ffi::gtk_gl_area_set_error(self.as_ref().to_glib_none().0, error.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_gl_area_set_has_depth_buffer")]
    #[doc(alias = "has-depth-buffer")]
    fn set_has_depth_buffer(&self, has_depth_buffer: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_depth_buffer(
                self.as_ref().to_glib_none().0,
                has_depth_buffer.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_gl_area_set_has_stencil_buffer")]
    #[doc(alias = "has-stencil-buffer")]
    fn set_has_stencil_buffer(&self, has_stencil_buffer: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_stencil_buffer(
                self.as_ref().to_glib_none().0,
                has_stencil_buffer.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_gl_area_set_required_version")]
    fn set_required_version(&self, major: i32, minor: i32) {
        unsafe {
            ffi::gtk_gl_area_set_required_version(self.as_ref().to_glib_none().0, major, minor);
        }
    }

    #[cfg_attr(feature = "v4_12", deprecated = "Since 4.12")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_gl_area_set_use_es")]
    #[doc(alias = "use-es")]
    fn set_use_es(&self, use_es: bool) {
        unsafe {
            ffi::gtk_gl_area_set_use_es(self.as_ref().to_glib_none().0, use_es.into_glib());
        }
    }

    #[doc(alias = "create-context")]
    fn connect_create_context<F: Fn(&Self) -> Option<gdk::GLContext> + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn create_context_trampoline<
            P: IsA<GLArea>,
            F: Fn(&P) -> Option<gdk::GLContext> + 'static,
        >(
            this: *mut ffi::GtkGLArea,
            f: glib::ffi::gpointer,
        ) -> *mut gdk::ffi::GdkGLContext {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref()).to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"create-context".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    create_context_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "render")]
    fn connect_render<F: Fn(&Self, &gdk::GLContext) -> glib::Propagation + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn render_trampoline<
            P: IsA<GLArea>,
            F: Fn(&P, &gdk::GLContext) -> glib::Propagation + 'static,
        >(
            this: *mut ffi::GtkGLArea,
            context: *mut gdk::ffi::GdkGLContext,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                GLArea::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(context),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"render".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    render_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resize")]
    fn connect_resize<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn resize_trampoline<P: IsA<GLArea>, F: Fn(&P, i32, i32) + 'static>(
            this: *mut ffi::GtkGLArea,
            width: std::ffi::c_int,
            height: std::ffi::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                GLArea::from_glib_borrow(this).unsafe_cast_ref(),
                width,
                height,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"resize".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    resize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "allowed-apis")]
    fn connect_allowed_apis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_allowed_apis_trampoline<P: IsA<GLArea>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::allowed-apis".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_allowed_apis_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "api")]
    fn connect_api_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_api_trampoline<P: IsA<GLArea>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::api".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_api_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "auto-render")]
    fn connect_auto_render_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_render_trampoline<P: IsA<GLArea>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::auto-render".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_auto_render_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "context")]
    fn connect_context_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_context_trampoline<P: IsA<GLArea>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::context".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_context_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "has-depth-buffer")]
    fn connect_has_depth_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_depth_buffer_trampoline<
            P: IsA<GLArea>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::has-depth-buffer".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_has_depth_buffer_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "has-stencil-buffer")]
    fn connect_has_stencil_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_stencil_buffer_trampoline<
            P: IsA<GLArea>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::has-stencil-buffer".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_has_stencil_buffer_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_12", deprecated = "Since 4.12")]
    #[doc(alias = "use-es")]
    fn connect_use_es_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_es_trampoline<P: IsA<GLArea>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::use-es".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_use_es_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<GLArea>> GLAreaExt for O {}
