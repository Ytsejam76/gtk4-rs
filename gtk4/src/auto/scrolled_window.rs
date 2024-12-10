// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Adjustment, Align, Buildable, ConstraintTarget, CornerType,
    DirectionType, LayoutManager, Overflow, PolicyType, PositionType, ScrollType, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkScrolledWindow")]
    pub struct ScrolledWindow(Object<ffi::GtkScrolledWindow>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_scrolled_window_get_type(),
    }
}

impl ScrolledWindow {
    #[doc(alias = "gtk_scrolled_window_new")]
    pub fn new() -> ScrolledWindow {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_scrolled_window_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ScrolledWindow`] objects.
    ///
    /// This method returns an instance of [`ScrolledWindowBuilder`](crate::builders::ScrolledWindowBuilder) which can be used to create [`ScrolledWindow`] objects.
    pub fn builder() -> ScrolledWindowBuilder {
        ScrolledWindowBuilder::new()
    }

    #[doc(alias = "gtk_scrolled_window_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_scrolled_window_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_scrolled_window_get_hadjustment")]
    #[doc(alias = "get_hadjustment")]
    pub fn hadjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(ffi::gtk_scrolled_window_get_hadjustment(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scrolled_window_get_has_frame")]
    #[doc(alias = "get_has_frame")]
    #[doc(alias = "has-frame")]
    pub fn has_frame(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_has_frame(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scrolled_window_get_hscrollbar")]
    #[doc(alias = "get_hscrollbar")]
    pub fn hscrollbar(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_scrolled_window_get_hscrollbar(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scrolled_window_get_kinetic_scrolling")]
    #[doc(alias = "get_kinetic_scrolling")]
    #[doc(alias = "kinetic-scrolling")]
    pub fn is_kinetic_scrolling(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_kinetic_scrolling(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scrolled_window_get_max_content_height")]
    #[doc(alias = "get_max_content_height")]
    #[doc(alias = "max-content-height")]
    pub fn max_content_height(&self) -> i32 {
        unsafe { ffi::gtk_scrolled_window_get_max_content_height(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_scrolled_window_get_max_content_width")]
    #[doc(alias = "get_max_content_width")]
    #[doc(alias = "max-content-width")]
    pub fn max_content_width(&self) -> i32 {
        unsafe { ffi::gtk_scrolled_window_get_max_content_width(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_scrolled_window_get_min_content_height")]
    #[doc(alias = "get_min_content_height")]
    #[doc(alias = "min-content-height")]
    pub fn min_content_height(&self) -> i32 {
        unsafe { ffi::gtk_scrolled_window_get_min_content_height(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_scrolled_window_get_min_content_width")]
    #[doc(alias = "get_min_content_width")]
    #[doc(alias = "min-content-width")]
    pub fn min_content_width(&self) -> i32 {
        unsafe { ffi::gtk_scrolled_window_get_min_content_width(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_scrolled_window_get_overlay_scrolling")]
    #[doc(alias = "get_overlay_scrolling")]
    #[doc(alias = "overlay-scrolling")]
    pub fn is_overlay_scrolling(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_overlay_scrolling(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scrolled_window_get_placement")]
    #[doc(alias = "get_placement")]
    #[doc(alias = "window-placement")]
    pub fn placement(&self) -> CornerType {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_placement(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scrolled_window_get_policy")]
    #[doc(alias = "get_policy")]
    pub fn policy(&self) -> (PolicyType, PolicyType) {
        unsafe {
            let mut hscrollbar_policy = std::mem::MaybeUninit::uninit();
            let mut vscrollbar_policy = std::mem::MaybeUninit::uninit();
            ffi::gtk_scrolled_window_get_policy(
                self.to_glib_none().0,
                hscrollbar_policy.as_mut_ptr(),
                vscrollbar_policy.as_mut_ptr(),
            );
            (
                from_glib(hscrollbar_policy.assume_init()),
                from_glib(vscrollbar_policy.assume_init()),
            )
        }
    }

    #[doc(alias = "gtk_scrolled_window_get_propagate_natural_height")]
    #[doc(alias = "get_propagate_natural_height")]
    #[doc(alias = "propagate-natural-height")]
    pub fn propagates_natural_height(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_propagate_natural_height(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scrolled_window_get_propagate_natural_width")]
    #[doc(alias = "get_propagate_natural_width")]
    #[doc(alias = "propagate-natural-width")]
    pub fn propagates_natural_width(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_propagate_natural_width(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scrolled_window_get_vadjustment")]
    #[doc(alias = "get_vadjustment")]
    pub fn vadjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(ffi::gtk_scrolled_window_get_vadjustment(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scrolled_window_get_vscrollbar")]
    #[doc(alias = "get_vscrollbar")]
    pub fn vscrollbar(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_scrolled_window_get_vscrollbar(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_scrolled_window_set_child")]
    #[doc(alias = "child")]
    pub fn set_child(&self, child: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_scrolled_window_set_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_scrolled_window_set_hadjustment")]
    #[doc(alias = "hadjustment")]
    pub fn set_hadjustment(&self, hadjustment: Option<&impl IsA<Adjustment>>) {
        unsafe {
            ffi::gtk_scrolled_window_set_hadjustment(
                self.to_glib_none().0,
                hadjustment.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_scrolled_window_set_has_frame")]
    #[doc(alias = "has-frame")]
    pub fn set_has_frame(&self, has_frame: bool) {
        unsafe {
            ffi::gtk_scrolled_window_set_has_frame(self.to_glib_none().0, has_frame.into_glib());
        }
    }

    #[doc(alias = "gtk_scrolled_window_set_kinetic_scrolling")]
    #[doc(alias = "kinetic-scrolling")]
    pub fn set_kinetic_scrolling(&self, kinetic_scrolling: bool) {
        unsafe {
            ffi::gtk_scrolled_window_set_kinetic_scrolling(
                self.to_glib_none().0,
                kinetic_scrolling.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_scrolled_window_set_max_content_height")]
    #[doc(alias = "max-content-height")]
    pub fn set_max_content_height(&self, height: i32) {
        unsafe {
            ffi::gtk_scrolled_window_set_max_content_height(self.to_glib_none().0, height);
        }
    }

    #[doc(alias = "gtk_scrolled_window_set_max_content_width")]
    #[doc(alias = "max-content-width")]
    pub fn set_max_content_width(&self, width: i32) {
        unsafe {
            ffi::gtk_scrolled_window_set_max_content_width(self.to_glib_none().0, width);
        }
    }

    #[doc(alias = "gtk_scrolled_window_set_min_content_height")]
    #[doc(alias = "min-content-height")]
    pub fn set_min_content_height(&self, height: i32) {
        unsafe {
            ffi::gtk_scrolled_window_set_min_content_height(self.to_glib_none().0, height);
        }
    }

    #[doc(alias = "gtk_scrolled_window_set_min_content_width")]
    #[doc(alias = "min-content-width")]
    pub fn set_min_content_width(&self, width: i32) {
        unsafe {
            ffi::gtk_scrolled_window_set_min_content_width(self.to_glib_none().0, width);
        }
    }

    #[doc(alias = "gtk_scrolled_window_set_overlay_scrolling")]
    #[doc(alias = "overlay-scrolling")]
    pub fn set_overlay_scrolling(&self, overlay_scrolling: bool) {
        unsafe {
            ffi::gtk_scrolled_window_set_overlay_scrolling(
                self.to_glib_none().0,
                overlay_scrolling.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_scrolled_window_set_placement")]
    #[doc(alias = "window-placement")]
    pub fn set_placement(&self, window_placement: CornerType) {
        unsafe {
            ffi::gtk_scrolled_window_set_placement(
                self.to_glib_none().0,
                window_placement.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_scrolled_window_set_policy")]
    pub fn set_policy(&self, hscrollbar_policy: PolicyType, vscrollbar_policy: PolicyType) {
        unsafe {
            ffi::gtk_scrolled_window_set_policy(
                self.to_glib_none().0,
                hscrollbar_policy.into_glib(),
                vscrollbar_policy.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_scrolled_window_set_propagate_natural_height")]
    #[doc(alias = "propagate-natural-height")]
    pub fn set_propagate_natural_height(&self, propagate: bool) {
        unsafe {
            ffi::gtk_scrolled_window_set_propagate_natural_height(
                self.to_glib_none().0,
                propagate.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_scrolled_window_set_propagate_natural_width")]
    #[doc(alias = "propagate-natural-width")]
    pub fn set_propagate_natural_width(&self, propagate: bool) {
        unsafe {
            ffi::gtk_scrolled_window_set_propagate_natural_width(
                self.to_glib_none().0,
                propagate.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_scrolled_window_set_vadjustment")]
    #[doc(alias = "vadjustment")]
    pub fn set_vadjustment(&self, vadjustment: Option<&impl IsA<Adjustment>>) {
        unsafe {
            ffi::gtk_scrolled_window_set_vadjustment(
                self.to_glib_none().0,
                vadjustment.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_scrolled_window_unset_placement")]
    pub fn unset_placement(&self) {
        unsafe {
            ffi::gtk_scrolled_window_unset_placement(self.to_glib_none().0);
        }
    }

    #[doc(alias = "hscrollbar-policy")]
    pub fn hscrollbar_policy(&self) -> PolicyType {
        ObjectExt::property(self, "hscrollbar-policy")
    }

    #[doc(alias = "hscrollbar-policy")]
    pub fn set_hscrollbar_policy(&self, hscrollbar_policy: PolicyType) {
        ObjectExt::set_property(self, "hscrollbar-policy", hscrollbar_policy)
    }

    #[doc(alias = "vscrollbar-policy")]
    pub fn vscrollbar_policy(&self) -> PolicyType {
        ObjectExt::property(self, "vscrollbar-policy")
    }

    #[doc(alias = "vscrollbar-policy")]
    pub fn set_vscrollbar_policy(&self, vscrollbar_policy: PolicyType) {
        ObjectExt::set_property(self, "vscrollbar-policy", vscrollbar_policy)
    }

    #[doc(alias = "edge-overshot")]
    pub fn connect_edge_overshot<F: Fn(&Self, PositionType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn edge_overshot_trampoline<
            F: Fn(&ScrolledWindow, PositionType) + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
            pos: ffi::GtkPositionType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(pos))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"edge-overshot".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    edge_overshot_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "edge-reached")]
    pub fn connect_edge_reached<F: Fn(&Self, PositionType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn edge_reached_trampoline<
            F: Fn(&ScrolledWindow, PositionType) + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
            pos: ffi::GtkPositionType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(pos))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"edge-reached".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    edge_reached_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "move-focus-out")]
    pub fn connect_move_focus_out<F: Fn(&Self, DirectionType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_focus_out_trampoline<
            F: Fn(&ScrolledWindow, DirectionType) + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
            direction_type: ffi::GtkDirectionType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(direction_type))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"move-focus-out".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    move_focus_out_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_move_focus_out(&self, direction_type: DirectionType) {
        self.emit_by_name::<()>("move-focus-out", &[&direction_type]);
    }

    #[doc(alias = "scroll-child")]
    pub fn connect_scroll_child<F: Fn(&Self, ScrollType, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn scroll_child_trampoline<
            F: Fn(&ScrolledWindow, ScrollType, bool) -> bool + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
            scroll: ffi::GtkScrollType,
            horizontal: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                from_glib(scroll),
                from_glib(horizontal),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"scroll-child".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    scroll_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_scroll_child(&self, scroll: ScrollType, horizontal: bool) -> bool {
        self.emit_by_name("scroll-child", &[&scroll, &horizontal])
    }

    #[doc(alias = "child")]
    pub fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<F: Fn(&ScrolledWindow) + 'static>(
            this: *mut ffi::GtkScrolledWindow,
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

    #[doc(alias = "hadjustment")]
    pub fn connect_hadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hadjustment_trampoline<F: Fn(&ScrolledWindow) + 'static>(
            this: *mut ffi::GtkScrolledWindow,
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
                c"notify::hadjustment".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_hadjustment_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "has-frame")]
    pub fn connect_has_frame_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_frame_trampoline<F: Fn(&ScrolledWindow) + 'static>(
            this: *mut ffi::GtkScrolledWindow,
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

    #[doc(alias = "hscrollbar-policy")]
    pub fn connect_hscrollbar_policy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_hscrollbar_policy_trampoline<
            F: Fn(&ScrolledWindow) + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
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
                c"notify::hscrollbar-policy".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_hscrollbar_policy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "kinetic-scrolling")]
    pub fn connect_kinetic_scrolling_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_kinetic_scrolling_trampoline<
            F: Fn(&ScrolledWindow) + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
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
                c"notify::kinetic-scrolling".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_kinetic_scrolling_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-content-height")]
    pub fn connect_max_content_height_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_content_height_trampoline<
            F: Fn(&ScrolledWindow) + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
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
                c"notify::max-content-height".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_max_content_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-content-width")]
    pub fn connect_max_content_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_content_width_trampoline<
            F: Fn(&ScrolledWindow) + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
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
                c"notify::max-content-width".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_max_content_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-content-height")]
    pub fn connect_min_content_height_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_content_height_trampoline<
            F: Fn(&ScrolledWindow) + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
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
                c"notify::min-content-height".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_min_content_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-content-width")]
    pub fn connect_min_content_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_content_width_trampoline<
            F: Fn(&ScrolledWindow) + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
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
                c"notify::min-content-width".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_min_content_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "overlay-scrolling")]
    pub fn connect_overlay_scrolling_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_overlay_scrolling_trampoline<
            F: Fn(&ScrolledWindow) + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
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
                c"notify::overlay-scrolling".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_overlay_scrolling_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "propagate-natural-height")]
    pub fn connect_propagate_natural_height_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_propagate_natural_height_trampoline<
            F: Fn(&ScrolledWindow) + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
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
                c"notify::propagate-natural-height".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_propagate_natural_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "propagate-natural-width")]
    pub fn connect_propagate_natural_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_propagate_natural_width_trampoline<
            F: Fn(&ScrolledWindow) + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
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
                c"notify::propagate-natural-width".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_propagate_natural_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "vadjustment")]
    pub fn connect_vadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vadjustment_trampoline<F: Fn(&ScrolledWindow) + 'static>(
            this: *mut ffi::GtkScrolledWindow,
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
                c"notify::vadjustment".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_vadjustment_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "vscrollbar-policy")]
    pub fn connect_vscrollbar_policy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_vscrollbar_policy_trampoline<
            F: Fn(&ScrolledWindow) + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
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
                c"notify::vscrollbar-policy".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_vscrollbar_policy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "window-placement")]
    pub fn connect_window_placement_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_window_placement_trampoline<
            F: Fn(&ScrolledWindow) + 'static,
        >(
            this: *mut ffi::GtkScrolledWindow,
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
                c"notify::window-placement".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_window_placement_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ScrolledWindow {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ScrolledWindow`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ScrolledWindowBuilder {
    builder: glib::object::ObjectBuilder<'static, ScrolledWindow>,
}

impl ScrolledWindowBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn hadjustment(self, hadjustment: &impl IsA<Adjustment>) -> Self {
        Self {
            builder: self
                .builder
                .property("hadjustment", hadjustment.clone().upcast()),
        }
    }

    pub fn has_frame(self, has_frame: bool) -> Self {
        Self {
            builder: self.builder.property("has-frame", has_frame),
        }
    }

    pub fn hscrollbar_policy(self, hscrollbar_policy: PolicyType) -> Self {
        Self {
            builder: self
                .builder
                .property("hscrollbar-policy", hscrollbar_policy),
        }
    }

    pub fn kinetic_scrolling(self, kinetic_scrolling: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("kinetic-scrolling", kinetic_scrolling),
        }
    }

    pub fn max_content_height(self, max_content_height: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("max-content-height", max_content_height),
        }
    }

    pub fn max_content_width(self, max_content_width: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("max-content-width", max_content_width),
        }
    }

    pub fn min_content_height(self, min_content_height: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("min-content-height", min_content_height),
        }
    }

    pub fn min_content_width(self, min_content_width: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("min-content-width", min_content_width),
        }
    }

    pub fn overlay_scrolling(self, overlay_scrolling: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("overlay-scrolling", overlay_scrolling),
        }
    }

    pub fn propagate_natural_height(self, propagate_natural_height: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("propagate-natural-height", propagate_natural_height),
        }
    }

    pub fn propagate_natural_width(self, propagate_natural_width: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("propagate-natural-width", propagate_natural_width),
        }
    }

    pub fn vadjustment(self, vadjustment: &impl IsA<Adjustment>) -> Self {
        Self {
            builder: self
                .builder
                .property("vadjustment", vadjustment.clone().upcast()),
        }
    }

    pub fn vscrollbar_policy(self, vscrollbar_policy: PolicyType) -> Self {
        Self {
            builder: self
                .builder
                .property("vscrollbar-policy", vscrollbar_policy),
        }
    }

    pub fn window_placement(self, window_placement: CornerType) -> Self {
        Self {
            builder: self.builder.property("window-placement", window_placement),
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
    /// Build the [`ScrolledWindow`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ScrolledWindow {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
