use crate::error::JsError;
use crate::types;
use crate::JsResult;
use napi_sys::{napi_env, napi_throw_error, napi_typeof, napi_value, napi_valuetype};
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem;
use std::os::raw::c_char;
use std::ptr;

#[derive(Clone, Copy)]
pub struct Env<'a> {
    pub(crate) env: napi_env,
    _r: PhantomData<&'a i8>,
}

impl<'a> Env<'a> {
    pub unsafe fn from_raw(env: napi_env) -> Env<'a> {
        Env {
            env,
            _r: PhantomData,
        }
    }

    pub fn throw(self, code: Option<&str>, message: &str) -> JsResult<()> {
        unsafe {
            let code = code.map(|s| CString::new(s).expect("create cstring from str fail"));
            let message = CString::new(message).expect("create cstring from str fail");
            let mut c_code: *const c_char = ptr::null();
            if let Some(ref code) = code {
                c_code = code.as_c_str().as_ptr();
            }
            node_try!(napi_throw_error, self, c_code, message.as_c_str().as_ptr());
            Err(JsError::PendingException)
        }
    }

    pub fn type_of(self, value: napi_value) -> JsResult<napi_valuetype> {
        unsafe {
            let mut result: napi_valuetype = mem::zeroed();
            node_try!(napi_typeof, self, value, &mut result);
            Ok(result)
        }
    }

    pub fn is_type_of(self, value: napi_value, typ: napi_valuetype) -> JsResult<bool> {
        Ok(self.type_of(value)? == typ)
    }

    pub fn null(self) -> JsResult<types::JsNull<'a>> {
        types::JsNull::get(self)
    }

    pub fn undefined(self) -> JsResult<types::JsUndefined<'a>> {
        types::JsUndefined::get(self)
    }

    pub fn string(self, s: &str) -> JsResult<types::JsString<'a>> {
        types::JsString::new(self, s)
    }
}
