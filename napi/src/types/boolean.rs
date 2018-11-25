use crate::env::Env;
use crate::value::{CastToJs, CastToRust, IntoRawJsValue, JsValue};
use crate::JsResult;
use napi_sys::{napi_coerce_to_bool, napi_get_boolean, napi_get_value_bool, napi_value, ValueType};
use std::marker::PhantomData;
use std::mem;

pub struct JsBool<'a> {
    value: napi_value,
    _m: PhantomData<&'a i8>,
}

impl<'a> JsValue<'a> for JsBool<'a> {
    unsafe fn as_raw(&self) -> napi_value {
        self.value
    }

    unsafe fn from_raw(env: Env<'a>, mut value: napi_value) -> JsResult<Self> {
        if !env.is_type_of(value, ValueType::Boolean)? {
            let mut bool_value: napi_value = mem::zeroed();
            node_try!(napi_coerce_to_bool, env, value, &mut bool_value);
            value = bool_value
        }
        Ok(JsBool {
            value,
            _m: PhantomData,
        })
    }

    fn to_bool(self, _env: Env<'a>) -> JsResult<JsBool<'a>> {
        Ok(self)
    }
}

impl<'a> JsBool<'a> {
    pub fn new(env: Env<'a>, b: bool) -> JsResult<Self> {
        unsafe {
            let mut value: napi_value = mem::zeroed();
            node_try!(napi_get_boolean, env, b, &mut value);
            Ok(JsBool {
                value,
                _m: PhantomData,
            })
        }
    }

    pub fn get_bool(&self, env: Env<'a>) -> JsResult<bool> {
        unsafe {
            let mut result = false;
            node_try!(napi_get_value_bool, env, self.value, &mut result);
            Ok(result)
        }
    }

    pub fn coerce_from<V: IntoRawJsValue>(env: Env<'a>, value: V) -> JsResult<Self> {
        unsafe {
            let value = value.into_raw_js_value();
            let mut result: napi_value = mem::zeroed();
            node_try!(napi_coerce_to_bool, env, value, &mut result);
            Ok(JsBool {
                value: result,
                _m: PhantomData,
            })
        }
    }
}

impl<'a> CastToRust<'a, bool> for JsBool<'a> {
    fn cast(&self, env: Env<'a>) -> JsResult<bool> {
        self.get_bool(env)
    }
}

impl<'a> CastToJs<'a, JsBool<'a>> for bool {
    fn cast(&self, env: Env<'a>) -> JsResult<JsBool<'a>> {
        JsBool::new(env, *self)
    }
}
