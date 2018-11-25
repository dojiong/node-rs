use crate::env::Env;
use crate::value::JsValue;
use crate::JsResult;
use napi_sys::{napi_get_null, napi_get_undefined, napi_value, ValueType};
use std::marker::PhantomData;
use std::mem;

pub struct JsUndefined<'a> {
    value: napi_value,
    _m: PhantomData<&'a i8>,
}

impl<'a> JsValue<'a> for JsUndefined<'a> {
    unsafe fn as_raw(&self) -> napi_value {
        self.value
    }

    unsafe fn from_raw(env: Env<'a>, value: napi_value) -> JsResult<Self> {
        if !env.is_type_of(value, ValueType::Undefined)? {
            env.throw(None, "JsUndefined from non-undefined")?;
        }
        Ok(JsUndefined {
            value,
            _m: PhantomData,
        })
    }
}

impl<'a> JsUndefined<'a> {
    pub fn get(env: Env<'a>) -> JsResult<Self> {
        unsafe {
            let mut value: napi_value = mem::zeroed();
            node_try!(napi_get_undefined, env, &mut value);
            Ok(Self::from_raw_unchecked(value))
        }
    }

    pub(crate) fn from_raw_unchecked(value: napi_value) -> JsUndefined<'a> {
        JsUndefined {
            value,
            _m: PhantomData,
        }
    }
}

pub struct JsNull<'a> {
    value: napi_value,
    _m: PhantomData<&'a i8>,
}

impl<'a> JsValue<'a> for JsNull<'a> {
    unsafe fn as_raw(&self) -> napi_value {
        self.value
    }

    unsafe fn from_raw(env: Env<'a>, value: napi_value) -> JsResult<Self> {
        if !env.is_type_of(value, ValueType::Null)? {
            env.throw(None, "JsNull from non-null")?;
        }
        Ok(JsNull {
            value,
            _m: PhantomData,
        })
    }
}

impl<'a> JsNull<'a> {
    pub fn get(env: Env<'a>) -> JsResult<Self> {
        unsafe {
            let mut value: napi_value = mem::zeroed();
            node_try!(napi_get_null, env, &mut value);
            Ok(Self::from_raw_unchecked(value))
        }
    }

    pub(crate) fn from_raw_unchecked(value: napi_value) -> JsNull<'a> {
        JsNull {
            value,
            _m: PhantomData,
        }
    }
}
