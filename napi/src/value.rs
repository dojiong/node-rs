use crate::callback::Callback;
use crate::env::Env;
use crate::types;
use crate::types::JsArray;
use crate::JsResult;
use napi_sys::{napi_value, ValueType};
use std::mem::MaybeUninit;
use std::{ffi::CStr, marker::PhantomData};

pub trait IntoRawJsValue {
    unsafe fn into_raw_js_value(self) -> napi_value;
}

impl IntoRawJsValue for napi_value {
    unsafe fn into_raw_js_value(self) -> napi_value {
        self
    }
}

pub trait JsValue<'a>: Sized {
    unsafe fn as_raw(&self) -> napi_value;
    unsafe fn from_raw(env: Env<'a>, value: napi_value) -> JsResult<Self>;

    fn get_property_names(&self, env: Env<'a>) -> JsResult<JsArray<'a>> {
        unsafe {
            let mut result: MaybeUninit<napi_value> = MaybeUninit::uninit();
            node_try!(
                napi_sys::napi_get_property_names,
                env,
                self.as_raw(),
                result.as_mut_ptr()
            );
            JsArray::from_raw(env, result.assume_init())
        }
    }

    fn has_property<K: CastToJs<'a, types::JsString<'a>>>(
        &self,
        env: Env<'a>,
        key: K,
    ) -> JsResult<bool> {
        unsafe {
            let mut result = false;
            let key: types::JsString<'a> = key.cast(env)?;
            node_try!(
                napi_sys::napi_has_property,
                env,
                self.as_raw(),
                key.into_raw_js_value(),
                &mut result
            );
            Ok(result)
        }
    }

    fn has_named_property(&self, env: Env<'a>, key: &CStr) -> JsResult<bool> {
        unsafe {
            let mut result = false;
            node_try!(
                napi_sys::napi_has_named_property,
                env,
                self.as_raw(),
                key.as_ptr(),
                &mut result
            );
            Ok(result)
        }
    }

    fn get_property<K: CastToJs<'a, types::JsString<'a>>, V: JsValue<'a>>(
        &self,
        env: Env<'a>,
        key: K,
    ) -> JsResult<Option<V>> {
        unsafe {
            let mut value = MaybeUninit::uninit();
            node_try!(
                napi_sys::napi_get_property,
                env,
                self.as_raw(),
                key.cast(env)?.into_raw_js_value(),
                value.as_mut_ptr()
            );
            let value = value.assume_init();
            if env.is_type_of(value, ValueType::Undefined)? {
                Ok(None)
            } else {
                V::from_raw(env, value).map(Some)
            }
        }
    }

    fn get_named_property<V: JsValue<'a>>(&self, env: Env<'a>, key: &CStr) -> JsResult<Option<V>> {
        unsafe {
            let mut value = MaybeUninit::uninit();
            node_try!(
                napi_sys::napi_get_named_property,
                env,
                self.as_raw(),
                key.as_ptr(),
                value.as_mut_ptr()
            );
            let value = value.assume_init();
            if env.is_type_of(value, ValueType::Undefined)? {
                Ok(None)
            } else {
                V::from_raw(env, value).map(Some)
            }
        }
    }

    fn set_property<K: CastToJs<'a, types::JsString<'a>>, V: JsValue<'a>>(
        &mut self,
        env: Env<'a>,
        key: K,
        value: &V,
    ) -> JsResult<()> {
        unsafe {
            node_try!(
                napi_sys::napi_set_property,
                env,
                self.as_raw(),
                key.cast(env)?.into_raw_js_value(),
                value.as_raw()
            );
        }
        Ok(())
    }

    fn set_named_property<V: JsValue<'a>>(
        &mut self,
        env: Env<'a>,
        key: &CStr,
        value: &V,
    ) -> JsResult<()> {
        unsafe {
            node_try!(
                napi_sys::napi_set_named_property,
                env,
                self.as_raw(),
                key.as_ptr(),
                value.as_raw()
            );
        }
        Ok(())
    }

    fn set_function<T, C>(&mut self, env: Env<'a>, name: &str, callback: C) -> JsResult<()>
    where
        T: JsValue<'a>,
        C: Callback<'a, T> + Sized,
    {
        let js_func = types::JsFunction::new(env, name, callback)?;
        self.set_property(env, name, &js_func)
    }

    fn is_undefined(&self, env: Env<'a>) -> JsResult<bool> {
        env.is_type_of(unsafe { self.as_raw() }, ValueType::Undefined)
    }

    fn is_null(&self, env: Env<'a>) -> JsResult<bool> {
        env.is_type_of(unsafe { self.as_raw() }, ValueType::Null)
    }

    fn is_null_or_undefined(&self, env: Env<'a>) -> JsResult<bool> {
        match env.type_of(unsafe { self.as_raw() })? {
            ValueType::Null | ValueType::Undefined => Ok(true),
            _ => Ok(false),
        }
    }

    fn is_string(&self, env: Env<'a>) -> JsResult<bool> {
        env.is_type_of(unsafe { self.as_raw() }, ValueType::String)
    }

    fn is_array(&self, env: Env<'a>) -> JsResult<bool> {
        unsafe {
            let mut result = false;
            node_try!(napi_sys::napi_is_array, env, self.as_raw(), &mut result);
            Ok(result)
        }
    }

    fn is_object(&self, env: Env<'a>) -> JsResult<bool> {
        env.is_type_of(unsafe { self.as_raw() }, ValueType::Object)
    }

    fn is_number(&self, env: Env<'a>) -> JsResult<bool> {
        env.is_type_of(unsafe { self.as_raw() }, ValueType::Number)
    }

    fn is_buffer(&self, env: Env<'a>) -> JsResult<bool> {
        let mut result = false;
        unsafe {
            node_try!(napi_sys::napi_is_buffer, env, self.as_raw(), &mut result);
        }
        Ok(result)
    }

    fn to_string(self, env: Env<'a>) -> JsResult<types::JsString<'a>> {
        types::JsString::coerce_from(env, self)
    }

    fn to_object(self, env: Env<'a>) -> JsResult<types::JsObject<'a>> {
        types::JsObject::coerce_from(env, self)
    }

    fn to_bool(self, env: Env<'a>) -> JsResult<types::JsBool<'a>> {
        types::JsBool::coerce_from(env, self)
    }

    fn to_number(self, env: Env<'a>) -> JsResult<types::JsNumber<'a>> {
        types::JsNumber::coerce_from(env, self)
    }
}

impl<'a, T: JsValue<'a>> IntoRawJsValue for T {
    unsafe fn into_raw_js_value(self) -> napi_value {
        self.as_raw()
    }
}

pub trait CastToRust<'a, T>: JsValue<'a> {
    fn cast(&self, env: Env<'a>) -> JsResult<T>;
}

pub trait CastToJs<'a, T: JsValue<'a>>: Sized {
    fn cast(&self, env: Env<'a>) -> JsResult<T>;
}

pub struct JsValueRaw<'a> {
    value: napi_value,
    _m: PhantomData<&'a u8>,
}

impl<'a> JsValue<'a> for JsValueRaw<'a> {
    unsafe fn as_raw(&self) -> napi_value {
        self.value
    }

    unsafe fn from_raw(_env: Env<'a>, value: napi_value) -> JsResult<Self> {
        Ok(JsValueRaw {
            value,
            _m: PhantomData,
        })
    }
}

impl<'a> JsValueRaw<'a> {
    pub(crate) fn from_raw_unchecked(value: napi_value) -> JsValueRaw<'a> {
        JsValueRaw {
            value,
            _m: PhantomData,
        }
    }

    pub fn cast<T: JsValue<'a>>(self, env: Env<'a>) -> JsResult<T> {
        unsafe { T::from_raw(env, self.value) }
    }
}
