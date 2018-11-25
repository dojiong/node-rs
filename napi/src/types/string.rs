use crate::env::Env;
use crate::value::{CastToJs, CastToRust, IntoRawJsValue, JsValue};
use crate::JsResult;
use napi_sys::{
    napi_coerce_to_string, napi_create_string_utf8, napi_get_value_string_utf8, napi_value,
    ValueType,
};
use std::marker::PhantomData;
use std::mem;
use std::os::raw::c_char;
use std::ptr;

pub struct JsString<'a> {
    value: napi_value,
    _m: PhantomData<&'a i8>,
}

impl<'a> JsValue<'a> for JsString<'a> {
    unsafe fn as_raw(&self) -> napi_value {
        self.value
    }

    unsafe fn from_raw(env: Env<'a>, mut value: napi_value) -> JsResult<Self> {
        match env.type_of(value)? {
            ValueType::String => {}
            _ => {
                let mut str_value: napi_value = mem::zeroed();
                node_try!(napi_coerce_to_string, env, value, &mut str_value);
                value = str_value;
            }
        }
        Ok(JsString {
            value,
            _m: PhantomData,
        })
    }

    fn to_string(self, _env: Env<'a>) -> JsResult<JsString<'a>> {
        Ok(self)
    }
}

impl<'a> JsString<'a> {
    pub fn new(env: Env<'a>, s: &str) -> JsResult<Self> {
        Self::from_utf8(env, s.as_bytes())
    }

    pub fn from_utf8(env: Env<'a>, bytes: &[u8]) -> JsResult<Self> {
        unsafe {
            let mut value: napi_value = mem::zeroed();
            node_try!(
                napi_create_string_utf8,
                env,
                bytes.as_ptr() as *const c_char,
                bytes.len(),
                &mut value
            );
            Ok(JsString {
                value,
                _m: PhantomData,
            })
        }
    }

    pub fn coerce_from<V: IntoRawJsValue>(env: Env<'a>, value: V) -> JsResult<Self> {
        unsafe {
            let value = value.into_raw_js_value();
            let mut result: napi_value = mem::zeroed();
            node_try!(napi_coerce_to_string, env, value, &mut result);
            Ok(JsString {
                value: result,
                _m: PhantomData,
            })
        }
    }

    pub fn get_str(&self, env: Env<'a>) -> JsResult<String> {
        unsafe {
            let mut size = 0;
            node_try!(
                napi_get_value_string_utf8,
                env,
                self.value,
                ptr::null_mut(),
                0,
                &mut size
            );
            let mut data: Vec<u8> = Vec::with_capacity(size + 1);
            data.set_len(size);
            node_try!(
                napi_get_value_string_utf8,
                env,
                self.value,
                data.as_mut_ptr() as *mut c_char,
                size + 1,
                &mut size
            );
            Ok(String::from_utf8_unchecked(data))
        }
    }
}

impl<'a> CastToRust<'a, String> for JsString<'a> {
    fn cast(&self, env: Env<'a>) -> JsResult<String> {
        self.get_str(env)
    }
}

impl<'a, S: AsRef<str>> CastToJs<'a, JsString<'a>> for S {
    fn cast(&self, env: Env<'a>) -> JsResult<JsString<'a>> {
        JsString::new(env, self.as_ref())
    }
}
