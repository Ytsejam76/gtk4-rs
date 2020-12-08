// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct EntryBuffer(Object<ffi::GtkEntryBuffer, ffi::GtkEntryBufferClass>);

    match fn {
        get_type => || ffi::gtk_entry_buffer_get_type(),
    }
}

#[derive(Clone, Default)]
pub struct EntryBufferBuilder {
    max_length: Option<i32>,
    text: Option<String>,
}

impl EntryBufferBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> EntryBuffer {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref max_length) = self.max_length {
            properties.push(("max-length", max_length));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
        }
        let ret = glib::Object::new(EntryBuffer::static_type(), &properties)
            .expect("object new")
            .downcast::<EntryBuffer>()
            .expect("downcast");
        ret
    }

    pub fn max_length(mut self, max_length: i32) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }
}

pub const NONE_ENTRY_BUFFER: Option<&EntryBuffer> = None;

pub trait EntryBufferExt: 'static {
    #[doc(alias = "gtk_entry_buffer_emit_deleted_text")]
    fn emit_deleted_text(&self, position: u32, n_chars: u32);

    #[doc(alias = "gtk_entry_buffer_emit_inserted_text")]
    fn emit_inserted_text(&self, position: u32, chars: &str, n_chars: u32);

    fn get_property_length(&self) -> u32;

    fn get_property_max_length(&self) -> i32;

    fn set_property_max_length(&self, max_length: i32);

    fn get_property_text(&self) -> Option<glib::GString>;

    fn set_property_text(&self, text: Option<&str>);

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<EntryBuffer>> EntryBufferExt for O {
    fn emit_deleted_text(&self, position: u32, n_chars: u32) {
        unsafe {
            ffi::gtk_entry_buffer_emit_deleted_text(
                self.as_ref().to_glib_none().0,
                position,
                n_chars,
            );
        }
    }

    fn emit_inserted_text(&self, position: u32, chars: &str, n_chars: u32) {
        unsafe {
            ffi::gtk_entry_buffer_emit_inserted_text(
                self.as_ref().to_glib_none().0,
                position,
                chars.to_glib_none().0,
                n_chars,
            );
        }
    }

    fn get_property_length(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"length\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `length` getter")
                .unwrap()
        }
    }

    fn get_property_max_length(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"max-length\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-length` getter")
                .unwrap()
        }
    }

    fn set_property_max_length(&self, max_length: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"max-length\0".as_ptr() as *const _,
                glib::Value::from(&max_length).to_glib_none().0,
            );
        }
    }

    fn get_property_text(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"text\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `text` getter")
        }
    }

    fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"text\0".as_ptr() as *const _,
                glib::Value::from(text).to_glib_none().0,
            );
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEntryBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<EntryBuffer>,
        {
            let f: &F = &*(f as *const F);
            f(&EntryBuffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_max_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_length_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEntryBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<EntryBuffer>,
        {
            let f: &F = &*(f as *const F);
            f(&EntryBuffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEntryBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<EntryBuffer>,
        {
            let f: &F = &*(f as *const F);
            f(&EntryBuffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for EntryBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EntryBuffer")
    }
}
