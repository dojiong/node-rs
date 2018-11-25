use crate::callback::Callback;
use crate::env::Env;
use crate::types;
use crate::JsResult;
use napi_sys::{napi_get_property, napi_has_property, napi_set_property, napi_value, ValueType};
use std::marker::PhantomData;
use std::mem;

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

    fn set<K, V>(&mut self, env: Env<'a>, key: &K, value: &V) -> JsResult<()>
    where
        K: JsValue<'a>,
        V: JsValue<'a>,
    {
        unsafe {
            node_try!(
                napi_set_property,
                env,
                self.as_raw(),
                key.as_raw(),
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
        let key: types::JsString<'a> = name.cast(env)?;
        self.set(env, &key, &js_func)
    }

    fn get<K, V>(&mut self, env: Env<'a>, key: &K) -> JsResult<V>
    where
        K: JsValue<'a>,
        V: JsValue<'a>,
    {
        unsafe {
            let mut value: napi_value = mem::uninitialized();
            node_try!(
                napi_get_property,
                env,
                self.as_raw(),
                key.as_raw(),
                &mut value
            );
            V::from_raw(env, value)
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
                napi_has_property,
                env,
                self.as_raw(),
                key.into_raw_js_value(),
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
            let mut value: napi_value = mem::uninitialized();
            node_try!(
                napi_get_property,
                env,
                self.as_raw(),
                key.cast(env)?.into_raw_js_value(),
                &mut value
            );
            if env.is_type_of(value, ValueType::Undefined)? {
                Ok(None)
            } else {
                V::from_raw(env, value).map(Some)
            }
        }
    }

    fn is_undefined(&self, env: Env<'a>) -> JsResult<bool> {
        env.is_type_of(unsafe { self.as_raw() }, ValueType::Undefined)
    }

    fn is_null(&self, env: Env<'a>) -> JsResult<bool> {
        env.is_type_of(unsafe { self.as_raw() }, ValueType::Null)
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
