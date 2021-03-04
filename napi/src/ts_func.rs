use crate::types::{JsFunction, JsString};
use crate::value::{CastToJs, IntoRawJsValue};
use crate::JsResult;
use crate::{env::Env, finalize::js_drop_finalize_cb};
use napi_sys::{
    self, napi_env, napi_status, napi_threadsafe_function, napi_value, Status,
    ThreadsafeFunctionCallMode,
};
use std::ffi::c_void;
use std::marker::PhantomData;
use std::mem;

pub struct ThreadSafeFunction<D> {
    ts_func: napi_threadsafe_function,
    _m: PhantomData<D>,
}

unsafe impl<D> Send for ThreadSafeFunction<D> {}

impl<D> Drop for ThreadSafeFunction<D> {
    fn drop(&mut self) {
        unsafe {
            napi_sys::napi_release_threadsafe_function(
                self.ts_func,
                napi_sys::napi_threadsafe_function_release_mode_napi_tsfn_release,
            );
        }
    }
}

impl<D: Send + Sized> ThreadSafeFunction<D> {
    pub fn new<'a, C>(
        env: Env<'a>,
        func: JsFunction<'a>,
        js_caller: C,
    ) -> JsResult<ThreadSafeFunction<D>>
    where
        C: JsCaller<D>,
    {
        let async_resource_name: JsString<'a> = "NODE_NATIVE_TS_FUNC".cast(env)?;
        unsafe {
            let ctx = Box::into_raw(Box::new(js_caller));
            let mut result: napi_threadsafe_function = mem::zeroed();
            node_try!(
                napi_sys::napi_create_threadsafe_function,
                env,
                func.into_raw_js_value(),
                0 as napi_value,
                async_resource_name.into_raw_js_value(),
                0,
                1,
                ctx as *mut c_void,
                Some(js_drop_finalize_cb::<C>),
                ctx as *mut c_void,
                Some(ts_function_call_js::<D, C>),
                &mut result
            );
            Ok(ThreadSafeFunction {
                ts_func: result,
                _m: PhantomData,
            })
        }
    }

    pub fn call(&self, data: D) -> Result<(), TsError> {
        unsafe {
            let data = Box::into_raw(Box::new(data));
            let ret = napi_sys::napi_call_threadsafe_function(
                self.ts_func,
                data as *mut c_void,
                ThreadsafeFunctionCallMode::Blocking,
            );
            if ret == Status::Ok {
                Ok(())
            } else {
                Err(TsError { status: ret })
            }
        }
    }

    pub fn clone(&self) -> Result<Self, TsError> {
        unsafe {
            let ret = napi_sys::napi_acquire_threadsafe_function(self.ts_func);
            if ret == Status::Ok {
                Ok(ThreadSafeFunction {
                    ts_func: self.ts_func,
                    _m: PhantomData,
                })
            } else {
                Err(TsError { status: ret })
            }
        }
    }
}

#[derive(Debug)]
pub struct TsError {
    pub status: napi_status,
}

pub trait JsCaller<D: Send + Sized>: Sized {
    fn call<'a>(&self, env: Env<'a>, func: JsFunction<'a>, data: D);

    fn make_ts_func<'a>(
        self,
        env: Env<'a>,
        func: JsFunction<'a>,
    ) -> JsResult<ThreadSafeFunction<D>> {
        ThreadSafeFunction::new(env, func, self)
    }
}

unsafe extern "C" fn ts_function_call_js<'a, D, C>(
    env: napi_env,
    js_cb: napi_value,
    ctx: *mut c_void,
    data: *mut c_void,
) where
    D: Send + Sized,
    C: JsCaller<D>,
{
    let env: Env<'a> = Env::from_raw(env);
    let js_caller: Box<C> = Box::from_raw(ctx as *mut C);
    let func: JsFunction<'a> = JsFunction::from_raw_unchecked(js_cb);
    let data = Box::from_raw(data as *mut D);

    js_caller.call(env, func, *data);
    mem::forget(js_caller);
}
