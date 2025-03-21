// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager, Overflow,
    ShortcutType, SizeGroup, TextDirection, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkShortcutsShortcut")]
    pub struct ShortcutsShortcut(Object<ffi::GtkShortcutsShortcut, ffi::GtkShortcutsShortcutClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_shortcuts_shortcut_get_type(),
    }
}

impl ShortcutsShortcut {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ShortcutsShortcut`] objects.
    ///
    /// This method returns an instance of [`ShortcutsShortcutBuilder`](crate::builders::ShortcutsShortcutBuilder) which can be used to create [`ShortcutsShortcut`] objects.
    pub fn builder() -> ShortcutsShortcutBuilder {
        ShortcutsShortcutBuilder::new()
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "accel-size-group")]
    pub fn set_accel_size_group(&self, accel_size_group: Option<&SizeGroup>) {
        ObjectExt::set_property(self, "accel-size-group", accel_size_group)
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn accelerator(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "accelerator")
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn set_accelerator(&self, accelerator: Option<&str>) {
        ObjectExt::set_property(self, "accelerator", accelerator)
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "action-name")]
    pub fn action_name(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "action-name")
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "action-name")]
    pub fn set_action_name(&self, action_name: Option<&str>) {
        ObjectExt::set_property(self, "action-name", action_name)
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn icon(&self) -> Option<gio::Icon> {
        ObjectExt::property(self, "icon")
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn set_icon<P: IsA<gio::Icon>>(&self, icon: Option<&P>) {
        ObjectExt::set_property(self, "icon", icon)
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "icon-set")]
    pub fn is_icon_set(&self) -> bool {
        ObjectExt::property(self, "icon-set")
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "shortcut-type")]
    pub fn shortcut_type(&self) -> ShortcutType {
        ObjectExt::property(self, "shortcut-type")
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "shortcut-type")]
    pub fn set_shortcut_type(&self, shortcut_type: ShortcutType) {
        ObjectExt::set_property(self, "shortcut-type", shortcut_type)
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn subtitle(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "subtitle")
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn set_subtitle(&self, subtitle: Option<&str>) {
        ObjectExt::set_property(self, "subtitle", subtitle)
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "subtitle-set")]
    pub fn is_subtitle_set(&self) -> bool {
        ObjectExt::property(self, "subtitle-set")
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn title(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "title")
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn set_title(&self, title: Option<&str>) {
        ObjectExt::set_property(self, "title", title)
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "title-size-group")]
    pub fn set_title_size_group(&self, title_size_group: Option<&SizeGroup>) {
        ObjectExt::set_property(self, "title-size-group", title_size_group)
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "accel-size-group")]
    pub fn connect_accel_size_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accel_size_group_trampoline<
            F: Fn(&ShortcutsShortcut) + 'static,
        >(
            this: *mut ffi::GtkShortcutsShortcut,
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
                c"notify::accel-size-group".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_accel_size_group_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "accelerator")]
    pub fn connect_accelerator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accelerator_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
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
                c"notify::accelerator".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_accelerator_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "action-name")]
    pub fn connect_action_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_name_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
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
                c"notify::action-name".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_action_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "direction")]
    pub fn connect_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
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
                c"notify::direction".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_direction_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "icon")]
    pub fn connect_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
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
                c"notify::icon".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_icon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "icon-set")]
    pub fn connect_icon_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_set_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
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
                c"notify::icon-set".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_icon_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "shortcut-type")]
    pub fn connect_shortcut_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shortcut_type_trampoline<
            F: Fn(&ShortcutsShortcut) + 'static,
        >(
            this: *mut ffi::GtkShortcutsShortcut,
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
                c"notify::shortcut-type".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_shortcut_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "subtitle")]
    pub fn connect_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
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
                c"notify::subtitle".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_subtitle_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "subtitle-set")]
    pub fn connect_subtitle_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_set_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
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
                c"notify::subtitle-set".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_subtitle_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&ShortcutsShortcut) + 'static>(
            this: *mut ffi::GtkShortcutsShortcut,
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

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "title-size-group")]
    pub fn connect_title_size_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_size_group_trampoline<
            F: Fn(&ShortcutsShortcut) + 'static,
        >(
            this: *mut ffi::GtkShortcutsShortcut,
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
                c"notify::title-size-group".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_title_size_group_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ShortcutsShortcut`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ShortcutsShortcutBuilder {
    builder: glib::object::ObjectBuilder<'static, ShortcutsShortcut>,
}

impl ShortcutsShortcutBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn accel_size_group(self, accel_size_group: &SizeGroup) -> Self {
        Self {
            builder: self
                .builder
                .property("accel-size-group", accel_size_group.clone()),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn accelerator(self, accelerator: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("accelerator", accelerator.into()),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn action_name(self, action_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("action-name", action_name.into()),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn direction(self, direction: TextDirection) -> Self {
        Self {
            builder: self.builder.property("direction", direction),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn icon(self, icon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self.builder.property("icon", icon.clone().upcast()),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn icon_set(self, icon_set: bool) -> Self {
        Self {
            builder: self.builder.property("icon-set", icon_set),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn shortcut_type(self, shortcut_type: ShortcutType) -> Self {
        Self {
            builder: self.builder.property("shortcut-type", shortcut_type),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn subtitle(self, subtitle: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("subtitle", subtitle.into()),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn subtitle_set(self, subtitle_set: bool) -> Self {
        Self {
            builder: self.builder.property("subtitle-set", subtitle_set),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn title_size_group(self, title_size_group: &SizeGroup) -> Self {
        Self {
            builder: self
                .builder
                .property("title-size-group", title_size_group.clone()),
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

    #[cfg(feature = "v4_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_18")))]
    pub fn limit_events(self, limit_events: bool) -> Self {
        Self {
            builder: self.builder.property("limit-events", limit_events),
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
    /// Build the [`ShortcutsShortcut`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ShortcutsShortcut {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
