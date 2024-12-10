// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager, Native,
    Overflow, PositionType, ShortcutManager, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkPopover")]
    pub struct Popover(Object<ffi::GtkPopover, ffi::GtkPopoverClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Native, ShortcutManager;

    match fn {
        type_ => || ffi::gtk_popover_get_type(),
    }
}

impl Popover {
    pub const NONE: Option<&'static Popover> = None;

    #[doc(alias = "gtk_popover_new")]
    pub fn new() -> Popover {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_popover_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Popover`] objects.
    ///
    /// This method returns an instance of [`PopoverBuilder`](crate::builders::PopoverBuilder) which can be used to create [`Popover`] objects.
    pub fn builder() -> PopoverBuilder {
        PopoverBuilder::new()
    }
}

impl Default for Popover {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Popover`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PopoverBuilder {
    builder: glib::object::ObjectBuilder<'static, Popover>,
}

impl PopoverBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn autohide(self, autohide: bool) -> Self {
        Self {
            builder: self.builder.property("autohide", autohide),
        }
    }

    pub fn cascade_popdown(self, cascade_popdown: bool) -> Self {
        Self {
            builder: self.builder.property("cascade-popdown", cascade_popdown),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn default_widget(self, default_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("default-widget", default_widget.clone().upcast()),
        }
    }

    pub fn has_arrow(self, has_arrow: bool) -> Self {
        Self {
            builder: self.builder.property("has-arrow", has_arrow),
        }
    }

    pub fn mnemonics_visible(self, mnemonics_visible: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("mnemonics-visible", mnemonics_visible),
        }
    }

    pub fn pointing_to(self, pointing_to: &gdk::Rectangle) -> Self {
        Self {
            builder: self.builder.property("pointing-to", pointing_to),
        }
    }

    pub fn position(self, position: PositionType) -> Self {
        Self {
            builder: self.builder.property("position", position),
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
    /// Build the [`Popover`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Popover {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}

pub trait PopoverExt: IsA<Popover> + 'static {
    #[doc(alias = "gtk_popover_get_autohide")]
    #[doc(alias = "get_autohide")]
    #[doc(alias = "autohide")]
    fn is_autohide(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_get_autohide(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_popover_get_cascade_popdown")]
    #[doc(alias = "get_cascade_popdown")]
    #[doc(alias = "cascade-popdown")]
    fn is_cascade_popdown(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_get_cascade_popdown(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_popover_get_child")]
    #[doc(alias = "get_child")]
    fn child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_popover_get_child(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_popover_get_has_arrow")]
    #[doc(alias = "get_has_arrow")]
    #[doc(alias = "has-arrow")]
    fn has_arrow(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_get_has_arrow(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_popover_get_mnemonics_visible")]
    #[doc(alias = "get_mnemonics_visible")]
    #[doc(alias = "mnemonics-visible")]
    fn is_mnemonics_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_get_mnemonics_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_popover_get_offset")]
    #[doc(alias = "get_offset")]
    fn offset(&self) -> (i32, i32) {
        unsafe {
            let mut x_offset = std::mem::MaybeUninit::uninit();
            let mut y_offset = std::mem::MaybeUninit::uninit();
            ffi::gtk_popover_get_offset(
                self.as_ref().to_glib_none().0,
                x_offset.as_mut_ptr(),
                y_offset.as_mut_ptr(),
            );
            (x_offset.assume_init(), y_offset.assume_init())
        }
    }

    #[doc(alias = "gtk_popover_get_pointing_to")]
    #[doc(alias = "get_pointing_to")]
    #[doc(alias = "pointing-to")]
    fn pointing_to(&self) -> (bool, gdk::Rectangle) {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_popover_get_pointing_to(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none_mut().0,
            ));
            (ret, rect)
        }
    }

    #[doc(alias = "gtk_popover_get_position")]
    #[doc(alias = "get_position")]
    fn position(&self) -> PositionType {
        unsafe {
            from_glib(ffi::gtk_popover_get_position(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_popover_popdown")]
    fn popdown(&self) {
        unsafe {
            ffi::gtk_popover_popdown(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_popover_popup")]
    fn popup(&self) {
        unsafe {
            ffi::gtk_popover_popup(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_popover_present")]
    fn present(&self) {
        unsafe {
            ffi::gtk_popover_present(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_popover_set_autohide")]
    #[doc(alias = "autohide")]
    fn set_autohide(&self, autohide: bool) {
        unsafe {
            ffi::gtk_popover_set_autohide(self.as_ref().to_glib_none().0, autohide.into_glib());
        }
    }

    #[doc(alias = "gtk_popover_set_cascade_popdown")]
    #[doc(alias = "cascade-popdown")]
    fn set_cascade_popdown(&self, cascade_popdown: bool) {
        unsafe {
            ffi::gtk_popover_set_cascade_popdown(
                self.as_ref().to_glib_none().0,
                cascade_popdown.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_popover_set_child")]
    #[doc(alias = "child")]
    fn set_child(&self, child: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_popover_set_child(
                self.as_ref().to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_popover_set_default_widget")]
    #[doc(alias = "default-widget")]
    fn set_default_widget(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_popover_set_default_widget(
                self.as_ref().to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_popover_set_has_arrow")]
    #[doc(alias = "has-arrow")]
    fn set_has_arrow(&self, has_arrow: bool) {
        unsafe {
            ffi::gtk_popover_set_has_arrow(self.as_ref().to_glib_none().0, has_arrow.into_glib());
        }
    }

    #[doc(alias = "gtk_popover_set_mnemonics_visible")]
    #[doc(alias = "mnemonics-visible")]
    fn set_mnemonics_visible(&self, mnemonics_visible: bool) {
        unsafe {
            ffi::gtk_popover_set_mnemonics_visible(
                self.as_ref().to_glib_none().0,
                mnemonics_visible.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_popover_set_offset")]
    fn set_offset(&self, x_offset: i32, y_offset: i32) {
        unsafe {
            ffi::gtk_popover_set_offset(self.as_ref().to_glib_none().0, x_offset, y_offset);
        }
    }

    #[doc(alias = "gtk_popover_set_pointing_to")]
    #[doc(alias = "pointing-to")]
    fn set_pointing_to(&self, rect: Option<&gdk::Rectangle>) {
        unsafe {
            ffi::gtk_popover_set_pointing_to(self.as_ref().to_glib_none().0, rect.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_popover_set_position")]
    #[doc(alias = "position")]
    fn set_position(&self, position: PositionType) {
        unsafe {
            ffi::gtk_popover_set_position(self.as_ref().to_glib_none().0, position.into_glib());
        }
    }

    #[doc(alias = "default-widget")]
    fn default_widget(&self) -> Option<Widget> {
        ObjectExt::property(self.as_ref(), "default-widget")
    }

    #[doc(alias = "activate-default")]
    fn connect_activate_default<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_default_trampoline<P: IsA<Popover>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPopover,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"activate-default".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    activate_default_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_activate_default(&self) {
        self.emit_by_name::<()>("activate-default", &[]);
    }

    #[doc(alias = "closed")]
    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<P: IsA<Popover>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPopover,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
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

    #[doc(alias = "autohide")]
    fn connect_autohide_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_autohide_trampoline<P: IsA<Popover>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPopover,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::autohide".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_autohide_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "cascade-popdown")]
    fn connect_cascade_popdown_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cascade_popdown_trampoline<
            P: IsA<Popover>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkPopover,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::cascade-popdown".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_cascade_popdown_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "child")]
    fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<P: IsA<Popover>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPopover,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::child".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "default-widget")]
    fn connect_default_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_widget_trampoline<
            P: IsA<Popover>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkPopover,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::default-widget".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_default_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "has-arrow")]
    fn connect_has_arrow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_arrow_trampoline<P: IsA<Popover>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPopover,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::has-arrow".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_has_arrow_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mnemonics-visible")]
    fn connect_mnemonics_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mnemonics_visible_trampoline<
            P: IsA<Popover>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkPopover,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::mnemonics-visible".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_mnemonics_visible_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pointing-to")]
    fn connect_pointing_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pointing_to_trampoline<P: IsA<Popover>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPopover,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::pointing-to".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pointing_to_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "position")]
    fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<P: IsA<Popover>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPopover,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Popover::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::position".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Popover>> PopoverExt for O {}
