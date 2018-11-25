use crate::callback::{Callback, CallbackInfo};
use crate::env::Env;
use crate::value::{IntoRawJsValue, JsValue, JsValueRaw};
use crate::JsResult;
use napi_sys::{
    napi_call_function, napi_callback_info, napi_create_function, napi_env, napi_value, ValueType,
};
use std::ffi::c_void;
use std::marker::PhantomData;
use std::mem;
use std::os::raw::c_char;

pub struct JsArgv {
    values: Vec<napi_value>,
}

impl JsArgv {
    pub fn new() -> JsArgv {
        JsArgv { values: Vec::new() }
    }

    pub fn push<V: IntoRawJsValue>(&mut self, value: V) {
        self.values.push(unsafe { value.into_raw_js_value() })
    }
}

#[macro_export]
macro_rules! js_argv {
    ($($item:expr),*) => ({
        let mut argv = JsArgv::new();
        $(argv.push($item);)*
        argv
    })
}

pub struct JsFunction<'a> {
    value: napi_value,
    _m: PhantomData<&'a i8>,
}

impl<'a> JsValue<'a> for JsFunction<'a> {
    unsafe fn as_raw(&self) -> napi_value {
        self.value
    }

    unsafe fn from_raw(env: Env<'a>, value: napi_value) -> JsResult<Self> {
        if !env.is_type_of(value, ValueType::Function)? {
            env.throw(None, "make JsFunction from non-function")?;
        }
        Ok(JsFunction {
            value,
            _m: PhantomData,
        })
    }
}

unsafe extern "C" fn _callback_fn<'a, T: JsValue<'a>, C: Callback<'a, T>>(
    env: napi_env,
    info: napi_callback_info,
) -> napi_value {
    let env: Env<'a> = Env::from_raw(env);
    let cb_info = match CallbackInfo::from_raw(env, info) {
        Ok(x) => x,
        Err(e) => {
            e.throw(env);
            return 0 as napi_value;
        }
    };
    let cb: Box<C> = Box::from_raw(cb_info.data as *mut C);
    let result = cb.call(env, cb_info);
    let _ = Box::into_raw(cb);
    match result {
        Ok(result) => result.into_raw_js_value(),
        Err(e) => {
            e.throw(env);
            0 as napi_value
        }
    }
}

impl<'a> JsFunction<'a> {
    pub(crate) fn from_raw_unchecked(value: napi_value) -> JsFunction<'a> {
        JsFunction {
            value,
            _m: PhantomData,
        }
    }

    pub fn new<T: JsValue<'a>, C: Callback<'a, T> + Sized>(
        env: Env<'a>,
        name: &str,
        callback: C,
    ) -> JsResult<Self> {
        unsafe {
            let boxed_cb = Box::into_raw(Box::new(callback));
            let mut result: napi_value = mem::zeroed();
            node_try!(
                napi_create_function,
                env,
                name.as_ptr() as *const c_char,
                name.len(),
                Some(_callback_fn::<T, C>),
                boxed_cb as *mut c_void,
                &mut result
            );
            Ok(JsFunction {
                value: result,
                _m: PhantomData,
            })
        }
    }

    pub fn call<TS, R>(&self, env: Env<'a>, this: &TS, argv: JsArgv) -> JsResult<R>
    where
        TS: JsValue<'a>,
        R: JsValue<'a>,
    {
        unsafe {
            let result = self.call_raw_argv(env, this, &argv.values)?;
            R::from_raw(env, result)
        }
    }

    pub fn call_r<T: JsValue<'a>>(
        &self,
        env: Env<'a>,
        this: &T,
        argv: JsArgv,
    ) -> JsResult<JsValueRaw<'a>> {
        self.call(env, this, argv)
    }

    pub fn call1<TS, T, R>(&self, env: Env<'a>, this: &TS, arg: T) -> JsResult<R>
    where
        TS: JsValue<'a>,
        T: IntoRawJsValue,
        R: JsValue<'a>,
    {
        self.call(env, this, js_argv![arg])
    }

    pub fn call1_r<TS, T>(&self, env: Env<'a>, this: &TS, arg: T) -> JsResult<JsValueRaw<'a>>
    where
        TS: JsValue<'a>,
        T: IntoRawJsValue,
    {
        self.call1(env, this, arg)
    }

    pub fn call2<TS, T1, T2, R>(&self, env: Env<'a>, this: &TS, arg1: T1, arg2: T2) -> JsResult<R>
    where
        TS: JsValue<'a>,
        T1: IntoRawJsValue,
        T2: IntoRawJsValue,
        R: JsValue<'a>,
    {
        self.call(env, this, js_argv![arg1, arg2])
    }

    pub fn call2_r<TS, T1, T2>(
        &self,
        env: Env<'a>,
        this: &TS,
        arg1: T1,
        arg2: T2,
    ) -> JsResult<JsValueRaw<'a>>
    where
        TS: JsValue<'a>,
        T1: IntoRawJsValue,
        T2: IntoRawJsValue,
    {
        self.call(env, this, js_argv![arg1, arg2])
    }

    pub(crate) unsafe fn call_raw_argv<T: JsValue<'a>>(
        &self,
        env: Env<'a>,
        this: &T,
        argv: &[napi_value],
    ) -> JsResult<napi_value> {
        let mut result: napi_value = mem::zeroed();
        node_try!(
            napi_call_function,
            env,
            this.as_raw(),
            self.value,
            argv.len(),
            argv.as_ptr(),
            &mut result
        );
        Ok(result)
    }
}
