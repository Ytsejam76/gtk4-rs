// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Accessible;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::Native;
use crate::Popover;
use crate::ShortcutManager;
use crate::Widget;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct EmojiChooser(Object<ffi::GtkEmojiChooser, ffi::GtkEmojiChooserClass>) @extends Popover, Widget, @implements Accessible, Buildable, ConstraintTarget, Native, ShortcutManager;

    match fn {
        get_type => || ffi::gtk_emoji_chooser_get_type(),
    }
}

impl EmojiChooser {
    #[doc(alias = "gtk_emoji_chooser_new")]
    pub fn new() -> EmojiChooser {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_emoji_chooser_new()).unsafe_cast() }
    }

    pub fn connect_emoji_picked<F: Fn(&EmojiChooser, &str) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn emoji_picked_trampoline<F: Fn(&EmojiChooser, &str) + 'static>(
            this: *mut ffi::GtkEmojiChooser,
            text: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(text),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"emoji-picked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    emoji_picked_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for EmojiChooser {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EmojiChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EmojiChooser")
    }
}
