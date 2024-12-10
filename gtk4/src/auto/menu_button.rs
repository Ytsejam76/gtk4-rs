// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Align, ArrowType, Buildable, ConstraintTarget, LayoutManager,
    Overflow, Popover, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkMenuButton")]
    pub struct MenuButton(Object<ffi::GtkMenuButton>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_menu_button_get_type(),
    }
}

impl MenuButton {
    #[doc(alias = "gtk_menu_button_new")]
    pub fn new() -> MenuButton {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_menu_button_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`MenuButton`] objects.
    ///
    /// This method returns an instance of [`MenuButtonBuilder`](crate::builders::MenuButtonBuilder) which can be used to create [`MenuButton`] objects.
    pub fn builder() -> MenuButtonBuilder {
        MenuButtonBuilder::new()
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_menu_button_get_active")]
    #[doc(alias = "get_active")]
    #[doc(alias = "active")]
    pub fn is_active(&self) -> bool {
        unsafe { from_glib(ffi::gtk_menu_button_get_active(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gtk_menu_button_get_always_show_arrow")]
    #[doc(alias = "get_always_show_arrow")]
    #[doc(alias = "always-show-arrow")]
    pub fn must_always_show_arrow(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_button_get_always_show_arrow(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_menu_button_get_can_shrink")]
    #[doc(alias = "get_can_shrink")]
    #[doc(alias = "can-shrink")]
    pub fn can_shrink(&self) -> bool {
        unsafe { from_glib(ffi::gtk_menu_button_get_can_shrink(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gtk_menu_button_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_menu_button_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_menu_button_get_direction")]
    #[doc(alias = "get_direction")]
    pub fn direction(&self) -> ArrowType {
        unsafe { from_glib(ffi::gtk_menu_button_get_direction(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_menu_button_get_has_frame")]
    #[doc(alias = "get_has_frame")]
    #[doc(alias = "has-frame")]
    pub fn has_frame(&self) -> bool {
        unsafe { from_glib(ffi::gtk_menu_button_get_has_frame(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_menu_button_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    #[doc(alias = "icon-name")]
    pub fn icon_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_menu_button_get_icon_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_menu_button_get_label")]
    #[doc(alias = "get_label")]
    pub fn label(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_menu_button_get_label(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_menu_button_get_menu_model")]
    #[doc(alias = "get_menu_model")]
    #[doc(alias = "menu-model")]
    pub fn menu_model(&self) -> Option<gio::MenuModel> {
        unsafe { from_glib_none(ffi::gtk_menu_button_get_menu_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_menu_button_get_popover")]
    #[doc(alias = "get_popover")]
    pub fn popover(&self) -> Option<Popover> {
        unsafe { from_glib_none(ffi::gtk_menu_button_get_popover(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gtk_menu_button_get_primary")]
    #[doc(alias = "get_primary")]
    #[doc(alias = "primary")]
    pub fn is_primary(&self) -> bool {
        unsafe { from_glib(ffi::gtk_menu_button_get_primary(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_menu_button_get_use_underline")]
    #[doc(alias = "get_use_underline")]
    #[doc(alias = "use-underline")]
    pub fn uses_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_button_get_use_underline(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_menu_button_popdown")]
    pub fn popdown(&self) {
        unsafe {
            ffi::gtk_menu_button_popdown(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_menu_button_popup")]
    pub fn popup(&self) {
        unsafe {
            ffi::gtk_menu_button_popup(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_menu_button_set_active")]
    #[doc(alias = "active")]
    pub fn set_active(&self, active: bool) {
        unsafe {
            ffi::gtk_menu_button_set_active(self.to_glib_none().0, active.into_glib());
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gtk_menu_button_set_always_show_arrow")]
    #[doc(alias = "always-show-arrow")]
    pub fn set_always_show_arrow(&self, always_show_arrow: bool) {
        unsafe {
            ffi::gtk_menu_button_set_always_show_arrow(
                self.to_glib_none().0,
                always_show_arrow.into_glib(),
            );
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_menu_button_set_can_shrink")]
    #[doc(alias = "can-shrink")]
    pub fn set_can_shrink(&self, can_shrink: bool) {
        unsafe {
            ffi::gtk_menu_button_set_can_shrink(self.to_glib_none().0, can_shrink.into_glib());
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gtk_menu_button_set_child")]
    #[doc(alias = "child")]
    pub fn set_child(&self, child: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_menu_button_set_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_menu_button_set_create_popup_func")]
    pub fn set_create_popup_func<P: Fn(&MenuButton) + 'static>(&self, func: P) {
        let func_data: Box_<P> = Box_::new(func);
        unsafe extern "C" fn func_func<P: Fn(&MenuButton) + 'static>(
            menu_button: *mut ffi::GtkMenuButton,
            user_data: glib::ffi::gpointer,
        ) {
            let menu_button = from_glib_borrow(menu_button);
            let callback = &*(user_data as *mut P);
            (*callback)(&menu_button)
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn destroy_notify_func<P: Fn(&MenuButton) + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback = Box_::from_raw(data as *mut P);
        }
        let destroy_call3 = Some(destroy_notify_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            ffi::gtk_menu_button_set_create_popup_func(
                self.to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gtk_menu_button_set_direction")]
    #[doc(alias = "direction")]
    pub fn set_direction(&self, direction: ArrowType) {
        unsafe {
            ffi::gtk_menu_button_set_direction(self.to_glib_none().0, direction.into_glib());
        }
    }

    #[doc(alias = "gtk_menu_button_set_has_frame")]
    #[doc(alias = "has-frame")]
    pub fn set_has_frame(&self, has_frame: bool) {
        unsafe {
            ffi::gtk_menu_button_set_has_frame(self.to_glib_none().0, has_frame.into_glib());
        }
    }

    #[doc(alias = "gtk_menu_button_set_icon_name")]
    #[doc(alias = "icon-name")]
    pub fn set_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::gtk_menu_button_set_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_menu_button_set_label")]
    #[doc(alias = "label")]
    pub fn set_label(&self, label: &str) {
        unsafe {
            ffi::gtk_menu_button_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_menu_button_set_menu_model")]
    #[doc(alias = "menu-model")]
    pub fn set_menu_model(&self, menu_model: Option<&impl IsA<gio::MenuModel>>) {
        unsafe {
            ffi::gtk_menu_button_set_menu_model(
                self.to_glib_none().0,
                menu_model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_menu_button_set_popover")]
    #[doc(alias = "popover")]
    pub fn set_popover(&self, popover: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_menu_button_set_popover(
                self.to_glib_none().0,
                popover.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gtk_menu_button_set_primary")]
    #[doc(alias = "primary")]
    pub fn set_primary(&self, primary: bool) {
        unsafe {
            ffi::gtk_menu_button_set_primary(self.to_glib_none().0, primary.into_glib());
        }
    }

    #[doc(alias = "gtk_menu_button_set_use_underline")]
    #[doc(alias = "use-underline")]
    pub fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_menu_button_set_use_underline(
                self.to_glib_none().0,
                use_underline.into_glib(),
            );
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "activate")]
    pub fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<F: Fn(&MenuButton) + 'static>(
            this: *mut ffi::GtkMenuButton,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"activate".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    activate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    pub fn emit_activate(&self) {
        self.emit_by_name::<()>("activate", &[]);
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "active")]
    pub fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<F: Fn(&MenuButton) + 'static>(
            this: *mut ffi::GtkMenuButton,
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
                c"notify::active".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_active_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "always-show-arrow")]
    pub fn connect_always_show_arrow_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_always_show_arrow_trampoline<F: Fn(&MenuButton) + 'static>(
            this: *mut ffi::GtkMenuButton,
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
                c"notify::always-show-arrow".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_always_show_arrow_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "can-shrink")]
    pub fn connect_can_shrink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_shrink_trampoline<F: Fn(&MenuButton) + 'static>(
            this: *mut ffi::GtkMenuButton,
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
                c"notify::can-shrink".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_can_shrink_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "child")]
    pub fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<F: Fn(&MenuButton) + 'static>(
            this: *mut ffi::GtkMenuButton,
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
                c"notify::child".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "direction")]
    pub fn connect_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<F: Fn(&MenuButton) + 'static>(
            this: *mut ffi::GtkMenuButton,
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

    #[doc(alias = "has-frame")]
    pub fn connect_has_frame_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_frame_trampoline<F: Fn(&MenuButton) + 'static>(
            this: *mut ffi::GtkMenuButton,
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
                c"notify::has-frame".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_has_frame_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-name")]
    pub fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&MenuButton) + 'static>(
            this: *mut ffi::GtkMenuButton,
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
                c"notify::icon-name".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "label")]
    pub fn connect_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<F: Fn(&MenuButton) + 'static>(
            this: *mut ffi::GtkMenuButton,
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
                c"notify::label".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "menu-model")]
    pub fn connect_menu_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menu_model_trampoline<F: Fn(&MenuButton) + 'static>(
            this: *mut ffi::GtkMenuButton,
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
                c"notify::menu-model".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_menu_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "popover")]
    pub fn connect_popover_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_popover_trampoline<F: Fn(&MenuButton) + 'static>(
            this: *mut ffi::GtkMenuButton,
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
                c"notify::popover".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_popover_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "primary")]
    pub fn connect_primary_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_primary_trampoline<F: Fn(&MenuButton) + 'static>(
            this: *mut ffi::GtkMenuButton,
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
                c"notify::primary".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_primary_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-underline")]
    pub fn connect_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_underline_trampoline<F: Fn(&MenuButton) + 'static>(
            this: *mut ffi::GtkMenuButton,
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
                c"notify::use-underline".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_use_underline_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for MenuButton {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`MenuButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct MenuButtonBuilder {
    builder: glib::object::ObjectBuilder<'static, MenuButton>,
}

impl MenuButtonBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn active(self, active: bool) -> Self {
        Self {
            builder: self.builder.property("active", active),
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    pub fn always_show_arrow(self, always_show_arrow: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("always-show-arrow", always_show_arrow),
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    pub fn can_shrink(self, can_shrink: bool) -> Self {
        Self {
            builder: self.builder.property("can-shrink", can_shrink),
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn direction(self, direction: ArrowType) -> Self {
        Self {
            builder: self.builder.property("direction", direction),
        }
    }

    pub fn has_frame(self, has_frame: bool) -> Self {
        Self {
            builder: self.builder.property("has-frame", has_frame),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn label(self, label: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("label", label.into()),
        }
    }

    pub fn menu_model(self, menu_model: &impl IsA<gio::MenuModel>) -> Self {
        Self {
            builder: self
                .builder
                .property("menu-model", menu_model.clone().upcast()),
        }
    }

    pub fn popover(self, popover: &impl IsA<Popover>) -> Self {
        Self {
            builder: self.builder.property("popover", popover.clone().upcast()),
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    pub fn primary(self, primary: bool) -> Self {
        Self {
            builder: self.builder.property("primary", primary),
        }
    }

    pub fn use_underline(self, use_underline: bool) -> Self {
        Self {
            builder: self.builder.property("use-underline", use_underline),
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
    /// Build the [`MenuButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> MenuButton {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
