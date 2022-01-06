// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ParamSpecExpression;

use glib::gobject_ffi;
use glib::translate::*;
use glib::{ParamSpec, StaticType, Value};

impl std::fmt::Debug for ParamSpecExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("ParamSpecExpression")
    }
}

impl std::ops::Deref for ParamSpecExpression {
    type Target = ParamSpec;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const ParamSpecExpression as *const ParamSpec) }
    }
}

unsafe impl glib::ParamSpecType for ParamSpecExpression {}

#[doc(hidden)]
impl FromGlibPtrFull<*mut gobject_ffi::GParamSpec> for ParamSpecExpression {
    unsafe fn from_glib_full(ptr: *mut gobject_ffi::GParamSpec) -> Self {
        from_glib_full(ptr as *mut ffi::GtkParamSpecExpression)
    }
}

impl ParamSpecExpression {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "gtk_param_spec_expression")]
    pub fn new(name: &str, nick: &str, blurb: &str, flags: glib::ParamFlags) -> ParamSpec {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_param_spec_expression(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    pub fn upcast(self) -> ParamSpec {
        unsafe { from_glib_full(self.to_glib_full() as *mut gobject_ffi::GParamSpec) }
    }

    pub fn upcast_ref(&self) -> &ParamSpec {
        &*self
    }
}

#[doc(hidden)]
impl glib::value::ValueType for ParamSpecExpression {
    type Type = ParamSpecExpression;
}

#[doc(hidden)]
impl glib::value::ValueTypeOptional for ParamSpecExpression {}

#[doc(hidden)]
unsafe impl<'a> glib::value::FromValue<'a> for ParamSpecExpression {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a Value) -> Self {
        let ptr = gobject_ffi::g_value_dup_param(value.to_glib_none().0);
        assert!(!ptr.is_null());
        from_glib_full(ptr as *mut gobject_ffi::GParamSpec)
    }
}

#[doc(hidden)]
impl glib::value::ToValue for ParamSpecExpression {
    fn to_value(&self) -> Value {
        unsafe {
            let mut value = Value::from_type(Self::static_type());
            gobject_ffi::g_value_take_param(
                value.to_glib_none_mut().0,
                self.to_glib_full() as *mut _,
            );
            value
        }
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[doc(hidden)]
impl glib::value::ToValueOptional for ParamSpecExpression {
    fn to_value_optional(s: Option<&Self>) -> Value {
        assert_initialized_main_thread!();
        let mut value = Value::for_value_type::<Self>();
        unsafe {
            gobject_ffi::g_value_take_param(value.to_glib_none_mut().0, s.to_glib_full() as *mut _);
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_synced;

    #[test]
    fn test_paramspec_expression() {
        test_synced(move || {
            let pspec = ParamSpecExpression::new(
                "expression",
                "Expression",
                "Some Expression",
                glib::ParamFlags::CONSTRUCT_ONLY | glib::ParamFlags::READABLE,
            );

            let expr_pspec = pspec.downcast::<ParamSpecExpression>();
            assert!(expr_pspec.is_ok());
        });
    }
}
