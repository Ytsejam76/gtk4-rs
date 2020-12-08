// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::FileChooser;
use crate::FileChooserAction;
use crate::FileFilter;
use crate::LayoutManager;
use crate::Overflow;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct FileChooserWidget(Object<ffi::GtkFileChooserWidget>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, FileChooser;

    match fn {
        get_type => || ffi::gtk_file_chooser_widget_get_type(),
    }
}

impl FileChooserWidget {
    #[doc(alias = "gtk_file_chooser_widget_new")]
    pub fn new(action: FileChooserAction) -> FileChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_file_chooser_widget_new(action.to_glib())).unsafe_cast()
        }
    }

    pub fn get_property_search_mode(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"search-mode\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `search-mode` getter")
                .unwrap()
        }
    }

    pub fn set_property_search_mode(&self, search_mode: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"search-mode\0".as_ptr() as *const _,
                glib::Value::from(&search_mode).to_glib_none().0,
            );
        }
    }

    pub fn get_property_subtitle(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"subtitle\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `subtitle` getter")
        }
    }

    pub fn connect_desktop_folder<F: Fn(&FileChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn desktop_folder_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"desktop-folder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    desktop_folder_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_desktop_folder(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("desktop-folder", &[])
                .unwrap()
        };
    }

    pub fn connect_down_folder<F: Fn(&FileChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn down_folder_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"down-folder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    down_folder_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_down_folder(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("down-folder", &[])
                .unwrap()
        };
    }

    pub fn connect_home_folder<F: Fn(&FileChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn home_folder_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"home-folder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    home_folder_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_home_folder(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("home-folder", &[])
                .unwrap()
        };
    }

    pub fn connect_location_popup<F: Fn(&FileChooserWidget, &str) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn location_popup_trampoline<
            F: Fn(&FileChooserWidget, &str) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            path: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(path),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"location-popup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    location_popup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_location_popup(&self, path: &str) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("location-popup", &[&path])
                .unwrap()
        };
    }

    pub fn connect_location_popup_on_paste<F: Fn(&FileChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn location_popup_on_paste_trampoline<
            F: Fn(&FileChooserWidget) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"location-popup-on-paste\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    location_popup_on_paste_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_location_popup_on_paste(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("location-popup-on-paste", &[])
                .unwrap()
        };
    }

    pub fn connect_location_toggle_popup<F: Fn(&FileChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn location_toggle_popup_trampoline<
            F: Fn(&FileChooserWidget) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"location-toggle-popup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    location_toggle_popup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_location_toggle_popup(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("location-toggle-popup", &[])
                .unwrap()
        };
    }

    pub fn connect_places_shortcut<F: Fn(&FileChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn places_shortcut_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"places-shortcut\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    places_shortcut_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_places_shortcut(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("places-shortcut", &[])
                .unwrap()
        };
    }

    pub fn connect_quick_bookmark<F: Fn(&FileChooserWidget, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn quick_bookmark_trampoline<F: Fn(&FileChooserWidget, i32) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            bookmark_index: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), bookmark_index)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"quick-bookmark\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    quick_bookmark_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_quick_bookmark(&self, bookmark_index: i32) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("quick-bookmark", &[&bookmark_index])
                .unwrap()
        };
    }

    pub fn connect_recent_shortcut<F: Fn(&FileChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn recent_shortcut_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"recent-shortcut\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    recent_shortcut_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_recent_shortcut(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("recent-shortcut", &[])
                .unwrap()
        };
    }

    pub fn connect_search_shortcut<F: Fn(&FileChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn search_shortcut_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"search-shortcut\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    search_shortcut_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_search_shortcut(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("search-shortcut", &[])
                .unwrap()
        };
    }

    pub fn connect_show_hidden<F: Fn(&FileChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn show_hidden_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show-hidden\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    show_hidden_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_show_hidden(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("show-hidden", &[])
                .unwrap()
        };
    }

    pub fn connect_up_folder<F: Fn(&FileChooserWidget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn up_folder_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"up-folder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    up_folder_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_up_folder(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("up-folder", &[])
                .unwrap()
        };
    }

    pub fn connect_property_search_mode_notify<F: Fn(&FileChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_mode_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
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
                b"notify::search-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_search_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_subtitle_notify<F: Fn(&FileChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
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
                b"notify::subtitle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subtitle_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct FileChooserWidgetBuilder {
    search_mode: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<AccessibleRole>,
    action: Option<FileChooserAction>,
    create_folders: Option<bool>,
    filter: Option<FileFilter>,
    select_multiple: Option<bool>,
}

impl FileChooserWidgetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> FileChooserWidget {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref search_mode) = self.search_mode {
            properties.push(("search-mode", search_mode));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        if let Some(ref action) = self.action {
            properties.push(("action", action));
        }
        if let Some(ref create_folders) = self.create_folders {
            properties.push(("create-folders", create_folders));
        }
        if let Some(ref filter) = self.filter {
            properties.push(("filter", filter));
        }
        if let Some(ref select_multiple) = self.select_multiple {
            properties.push(("select-multiple", select_multiple));
        }
        let ret = glib::Object::new(FileChooserWidget::static_type(), &properties)
            .expect("object new")
            .downcast::<FileChooserWidget>()
            .expect("downcast");
        ret
    }

    pub fn search_mode(mut self, search_mode: bool) -> Self {
        self.search_mode = Some(search_mode);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }

    pub fn action(mut self, action: FileChooserAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn create_folders(mut self, create_folders: bool) -> Self {
        self.create_folders = Some(create_folders);
        self
    }

    pub fn filter(mut self, filter: &FileFilter) -> Self {
        self.filter = Some(filter.clone());
        self
    }

    pub fn select_multiple(mut self, select_multiple: bool) -> Self {
        self.select_multiple = Some(select_multiple);
        self
    }
}

impl fmt::Display for FileChooserWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileChooserWidget")
    }
}
