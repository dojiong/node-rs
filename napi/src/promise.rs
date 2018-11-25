use crate::env::Env;
use crate::value::{IntoRawJsValue, JsValue};
use crate::JsResult;
use napi_sys::{
    napi_create_promise, napi_deferred, napi_is_promise, napi_reject_deferred,
    napi_resolve_deferred, napi_value,
};
use std::marker::PhantomData;
use std::mem;

pub struct JsPromise<'a> {
    value: napi_value,
    _m: PhantomData<&'a i8>,
}

impl<'a> JsValue<'a> for JsPromise<'a> {
    unsafe fn as_raw(&self) -> napi_value {
        self.value
    }

    unsafe fn from_raw(env: Env<'a>, value: napi_value) -> JsResult<Self> {
        let mut is_promise = false;
        node_try!(napi_is_promise, env, value, &mut is_promise);
        if !is_promise {
            env.throw(None, "Promise from non-promise")?;
        }
        Ok(JsPromise {
            value,
            _m: PhantomData,
        })
    }
}

impl<'a> JsPromise<'a> {
    pub fn new(env: Env<'a>) -> JsResult<(JsPromise<'a>, JsDeferred)> {
        unsafe {
            let mut deferred: napi_deferred = mem::zeroed();
            let mut value: napi_value = mem::zeroed();
            node_try!(napi_create_promise, env, &mut deferred, &mut value);
            Ok((
                JsPromise {
                    value,
                    _m: PhantomData,
                },
                JsDeferred { deferred },
            ))
        }
    }
}

pub struct JsDeferred {
    deferred: napi_deferred,
}

unsafe impl Send for JsDeferred {}

impl JsDeferred {
    pub fn resolve<'a, V: IntoRawJsValue>(self, env: Env<'a>, value: V) -> JsResult<()> {
        unsafe {
            node_try!(
                napi_resolve_deferred,
                env,
                self.deferred,
                value.into_raw_js_value()
            );
            Ok(())
        }
    }

    pub fn reject<'a, V: IntoRawJsValue>(self, env: Env<'a>, value: V) -> JsResult<()> {
        unsafe {
            node_try!(
                napi_reject_deferred,
                env,
                self.deferred,
                value.into_raw_js_value()
            );
            Ok(())
        }
    }
}
