use crate::env::Env;
use crate::types::{JsBool, JsNumber, JsObject, JsString};
use crate::value::{CastToRust, IntoRawJsValue, JsValue, JsValueRaw};
use crate::JsResult;
use napi_sys::{self, napi_value};
use std::marker::PhantomData;
use std::mem;

pub struct JsArray<'a> {
    value: napi_value,
    _m: PhantomData<&'a i8>,
}

impl<'a> JsValue<'a> for JsArray<'a> {
    unsafe fn as_raw(&self) -> napi_value {
        self.value
    }

    unsafe fn from_raw(env: Env<'a>, value: napi_value) -> JsResult<Self> {
        let mut is_array = false;
        node_try!(napi_sys::napi_is_array, env, value, &mut is_array);
        if !is_array {
            env.throw(None, "make JsArray from non-array")?;
        }
        Ok(JsArray {
            value,
            _m: PhantomData,
        })
    }
}

impl<'a> JsArray<'a> {
    pub fn new(env: Env<'a>) -> JsResult<Self> {
        unsafe {
            let mut value: napi_value = mem::zeroed();
            node_try!(napi_sys::napi_create_array, env, &mut value);
            Ok(JsArray {
                value,
                _m: PhantomData,
            })
        }
    }

    pub fn new_with_len(env: Env<'a>, len: usize) -> JsResult<Self> {
        unsafe {
            let mut value: napi_value = mem::zeroed();
            node_try!(
                napi_sys::napi_create_array_with_length,
                env,
                len,
                &mut value
            );
            Ok(JsArray {
                value,
                _m: PhantomData,
            })
        }
    }

    pub unsafe fn from_raw_values(env: Env<'a>, values: &[napi_value]) -> JsResult<Self> {
        let mut value: napi_value = mem::zeroed();
        node_try!(
            napi_sys::napi_create_array_with_length,
            env,
            values.len(),
            &mut value
        );
        for (i, item) in values.iter().enumerate() {
            node_try!(napi_sys::napi_set_element, env, value, i as u32, *item);
        }
        Ok(JsArray {
            value,
            _m: PhantomData,
        })
    }

    pub fn len(&self, env: Env<'a>) -> JsResult<usize> {
        unsafe {
            let mut result: u32 = 0;
            node_try!(
                napi_sys::napi_get_array_length,
                env,
                self.value,
                &mut result
            );
            Ok(result as usize)
        }
    }

    pub fn get<T: JsValue<'a>>(&self, env: Env<'a>, index: usize) -> JsResult<T> {
        unsafe {
            let mut value: napi_value = mem::zeroed();
            node_try!(
                napi_sys::napi_get_element,
                env,
                self.value,
                index as u32,
                &mut value
            );
            T::from_raw(env, value)
        }
    }

    pub fn get_raw(&self, env: Env<'a>, index: usize) -> JsResult<JsValueRaw<'a>> {
        unsafe {
            let mut value: napi_value = mem::zeroed();
            node_try!(
                napi_sys::napi_get_element,
                env,
                self.value,
                index as u32,
                &mut value
            );
            JsValueRaw::from_raw(env, value)
        }
    }

    pub fn get_str(&self, env: Env<'a>, index: usize) -> JsResult<String> {
        self.get::<JsString<'a>>(env, index)?.cast(env)
    }

    pub fn get_i32(&self, env: Env<'a>, index: usize) -> JsResult<i32> {
        self.get::<JsNumber<'a>>(env, index)?.cast(env)
    }

    pub fn get_i64(&self, env: Env<'a>, index: usize) -> JsResult<i64> {
        self.get::<JsNumber<'a>>(env, index)?.cast(env)
    }

    pub fn get_f64(&self, env: Env<'a>, index: usize) -> JsResult<f64> {
        self.get::<JsNumber<'a>>(env, index)?.cast(env)
    }

    pub fn get_bool(&self, env: Env<'a>, index: usize) -> JsResult<bool> {
        self.get::<JsBool<'a>>(env, index)?.cast(env)
    }

    pub fn get_obj(&self, env: Env<'a>, index: usize) -> JsResult<JsObject<'a>> {
        self.get(env, index)
    }

    pub fn set<T: JsValue<'a>>(&mut self, env: Env<'a>, index: usize, value: T) -> JsResult<()> {
        unsafe {
            node_try!(
                napi_sys::napi_set_element,
                env,
                self.value,
                index as u32,
                value.into_raw_js_value()
            );
        }
        Ok(())
    }

    pub fn has(&self, env: Env<'a>, index: usize) -> JsResult<bool> {
        unsafe {
            let mut result = false;
            node_try!(
                napi_sys::napi_has_element,
                env,
                self.value,
                index as u32,
                &mut result
            );
            Ok(result)
        }
    }

    pub fn delete(&mut self, env: Env<'a>, index: usize) -> JsResult<bool> {
        unsafe {
            let mut result = false;
            node_try!(
                napi_sys::napi_delete_element,
                env,
                self.value,
                index as u32,
                &mut result
            );
            Ok(result)
        }
    }
}

#[macro_export]
macro_rules! js_array {
    ($env:expr, $($item:expr),*) => {
        unsafe {
            ::node::types::JsArray::from_raw_values($env, &[$($item.as_raw()),*])
        }
    };
}
