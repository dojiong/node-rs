use crate::env::Env;
use crate::value::{CastToJs, CastToRust, IntoRawJsValue, JsValue};
use crate::JsResult;
use napi_sys::{
    napi_coerce_to_number, napi_create_double, napi_create_int32, napi_create_int64,
    napi_create_uint32, napi_get_value_double, napi_get_value_int32, napi_get_value_int64,
    napi_value, ValueType,
};
use std::marker::PhantomData;
use std::mem;

pub struct JsNumber<'a> {
    value: napi_value,
    _m: PhantomData<&'a i8>,
}

impl<'a> JsValue<'a> for JsNumber<'a> {
    unsafe fn as_raw(&self) -> napi_value {
        self.value
    }

    unsafe fn from_raw(env: Env<'a>, mut value: napi_value) -> JsResult<Self> {
        match env.type_of(value)? {
            ValueType::Number => {}
            _ => {
                let mut num_value: napi_value = mem::zeroed();
                node_try!(napi_coerce_to_number, env, value, &mut num_value);
                value = num_value;
            }
        }
        Ok(JsNumber {
            value,
            _m: PhantomData,
        })
    }

    fn to_number(self, _env: Env<'a>) -> JsResult<JsNumber<'a>> {
        Ok(self)
    }
}

impl<'a> JsNumber<'a> {
    pub fn coerce_from<T: IntoRawJsValue>(env: Env<'a>, value: T) -> JsResult<Self> {
        unsafe {
            let value = value.into_raw_js_value();
            let mut result: napi_value = mem::zeroed();
            node_try!(napi_coerce_to_number, env, value, &mut result);
            Ok(JsNumber {
                value: result,
                _m: PhantomData,
            })
        }
    }
}

impl<'a> CastToJs<'a, JsNumber<'a>> for i32 {
    fn cast(&self, env: Env<'a>) -> JsResult<JsNumber<'a>> {
        unsafe {
            let mut value: napi_value = mem::zeroed();
            node_try!(napi_create_int32, env, *self, &mut value);
            Ok(JsNumber {
                value,
                _m: PhantomData,
            })
        }
    }
}

impl<'a> CastToJs<'a, JsNumber<'a>> for u32 {
    fn cast(&self, env: Env<'a>) -> JsResult<JsNumber<'a>> {
        unsafe {
            let mut value: napi_value = mem::zeroed();
            node_try!(napi_create_uint32, env, *self, &mut value);
            Ok(JsNumber {
                value,
                _m: PhantomData,
            })
        }
    }
}

impl<'a> CastToJs<'a, JsNumber<'a>> for i64 {
    fn cast(&self, env: Env<'a>) -> JsResult<JsNumber<'a>> {
        unsafe {
            let mut value: napi_value = mem::zeroed();
            node_try!(napi_create_int64, env, *self, &mut value);
            Ok(JsNumber {
                value,
                _m: PhantomData,
            })
        }
    }
}

impl<'a> CastToJs<'a, JsNumber<'a>> for f32 {
    fn cast(&self, env: Env<'a>) -> JsResult<JsNumber<'a>> {
        unsafe {
            let mut value: napi_value = mem::zeroed();
            node_try!(napi_create_double, env, *self as f64, &mut value);
            Ok(JsNumber {
                value,
                _m: PhantomData,
            })
        }
    }
}

impl<'a> CastToJs<'a, JsNumber<'a>> for f64 {
    fn cast(&self, env: Env<'a>) -> JsResult<JsNumber<'a>> {
        unsafe {
            let mut value: napi_value = mem::zeroed();
            node_try!(napi_create_double, env, *self, &mut value);
            Ok(JsNumber {
                value,
                _m: PhantomData,
            })
        }
    }
}

impl<'a> CastToRust<'a, i32> for JsNumber<'a> {
    fn cast(&self, env: Env<'a>) -> JsResult<i32> {
        unsafe {
            let mut result: i32 = 0;
            node_try!(napi_get_value_int32, env, self.value, &mut result);
            Ok(result)
        }
    }
}

impl<'a> CastToRust<'a, i64> for JsNumber<'a> {
    fn cast(&self, env: Env<'a>) -> JsResult<i64> {
        unsafe {
            let mut result: i64 = 0;
            node_try!(napi_get_value_int64, env, self.value, &mut result);
            Ok(result)
        }
    }
}

impl<'a> CastToRust<'a, f64> for JsNumber<'a> {
    fn cast(&self, env: Env<'a>) -> JsResult<f64> {
        unsafe {
            let mut result: f64 = 0f64;
            node_try!(napi_get_value_double, env, self.value, &mut result);
            Ok(result)
        }
    }
}
