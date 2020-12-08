// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct ContentDeserializer(Object<ffi::GdkContentDeserializer>);

    match fn {
        get_type => || ffi::gdk_content_deserializer_get_type(),
    }
}

impl ContentDeserializer {
    #[doc(alias = "gdk_content_deserializer_get_cancellable")]
    pub fn get_cancellable(&self) -> Option<gio::Cancellable> {
        unsafe {
            from_glib_none(ffi::gdk_content_deserializer_get_cancellable(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_deserializer_get_gtype")]
    pub fn get_gtype(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gdk_content_deserializer_get_gtype(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_deserializer_get_input_stream")]
    pub fn get_input_stream(&self) -> gio::InputStream {
        unsafe {
            from_glib_none(ffi::gdk_content_deserializer_get_input_stream(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_deserializer_get_mime_type")]
    pub fn get_mime_type(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gdk_content_deserializer_get_mime_type(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_deserializer_get_priority")]
    pub fn get_priority(&self) -> i32 {
        unsafe { ffi::gdk_content_deserializer_get_priority(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_content_deserializer_get_value")]
    pub fn get_value(&self) -> glib::Value {
        unsafe {
            from_glib_none(ffi::gdk_content_deserializer_get_value(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_deserializer_return_error")]
    pub fn return_error(&self, error: &mut glib::Error) {
        unsafe {
            ffi::gdk_content_deserializer_return_error(
                self.to_glib_none().0,
                error.to_glib_none_mut().0,
            );
        }
    }

    #[doc(alias = "gdk_content_deserializer_return_success")]
    pub fn return_success(&self) {
        unsafe {
            ffi::gdk_content_deserializer_return_success(self.to_glib_none().0);
        }
    }
}

impl fmt::Display for ContentDeserializer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ContentDeserializer")
    }
}
