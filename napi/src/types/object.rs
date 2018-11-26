use crate::env::Env;
use crate::value::{IntoRawJsValue, JsValue};
use crate::JsResult;
use napi_sys::{napi_coerce_to_object, napi_value};
use std::marker::PhantomData;
use std::mem;

pub struct JsObject<'a> {
    value: napi_value,
    _m: PhantomData<&'a u8>,
}

impl<'a> JsValue<'a> for JsObject<'a> {
    unsafe fn as_raw(&self) -> napi_value {
        self.value
    }

    unsafe fn from_raw(_env: Env<'a>, value: napi_value) -> JsResult<Self> {
        Ok(JsObject {
            value,
            _m: PhantomData,
        })
    }

    fn to_object(self, _env: Env<'a>) -> JsResult<JsObject<'a>> {
        Ok(self)
    }
}

impl<'a> JsObject<'a> {
    pub fn new(env: Env<'a>) -> JsResult<Self> {
        unsafe {
            let mut value: napi_value = mem::zeroed();
            node_try!(napi_sys::napi_create_object, env, &mut value);
            Ok(JsObject {
                value,
                _m: PhantomData,
            })
        }
    }

    pub fn downcast<T: JsValue<'a>>(self, env: Env<'a>) -> JsResult<T> {
        unsafe { T::from_raw(env, self.value) }
    }

    pub fn coerce_from<T: IntoRawJsValue>(env: Env<'a>, value: T) -> JsResult<JsObject<'a>> {
        unsafe {
            let value = value.into_raw_js_value();
            let mut result: napi_value = mem::zeroed();
            node_try!(napi_coerce_to_object, env, value, &mut result);
            Ok(JsObject {
                value: result,
                _m: PhantomData,
            })
        }
    }
}
