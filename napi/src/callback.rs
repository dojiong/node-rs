use crate::env::Env;
use crate::types::JsObject;
use crate::value::{JsValue, JsValueRaw};
use crate::JsResult;
use napi_sys::{napi_callback_info, napi_get_cb_info, napi_value};
use std::ffi::c_void;
use std::mem;
use std::ptr;

pub trait Callback<'a, T: JsValue<'a>> {
    fn call(&self, env: Env<'a>, info: CallbackInfo<'a>) -> JsResult<T>;
}

impl<'a, T: JsValue<'a>, F> Callback<'a, T> for F
where
    F: Fn(Env<'a>, CallbackInfo<'a>) -> JsResult<T>,
{
    fn call(&self, env: Env<'a>, info: CallbackInfo<'a>) -> JsResult<T> {
        (*self)(env, info)
    }
}

pub struct CallbackInfo<'a> {
    pub this: JsObject<'a>,
    pub(crate) argv: Vec<napi_value>,
    pub(crate) data: *mut c_void,
}

impl<'a> CallbackInfo<'a> {
    const DEFAULT_ARGC: usize = 6;

    pub(crate) unsafe fn from_raw(
        env: Env<'a>,
        info: napi_callback_info,
    ) -> JsResult<CallbackInfo<'a>> {
        let mut argc: usize = Self::DEFAULT_ARGC;
        loop {
            let mut this_arg: napi_value = mem::zeroed();
            let mut argv = Vec::with_capacity(argc);
            argv.set_len(argc);
            let mut data: *mut c_void = ptr::null_mut();
            node_try!(
                napi_get_cb_info,
                env,
                info,
                &mut argc,
                argv.as_mut_ptr(),
                &mut this_arg,
                &mut data
            );
            if argc <= argv.len() {
                argv.truncate(argc);
                return Ok(CallbackInfo {
                    this: JsObject::from_raw(env, this_arg)?,
                    argv,
                    data,
                });
            }
        }
    }

    pub fn argv_len(&self) -> usize {
        self.argv.len()
    }

    pub fn arg<T: JsValue<'a>>(&self, env: Env<'a>, index: usize) -> JsResult<T> {
        if index >= self.argv.len() {
            env.throw(None, "callback.arg: index out of bounds")?;
        }
        unsafe { T::from_raw(env, self.argv[index]) }
    }

    pub fn arg_raw(&self, index: usize) -> Option<JsValueRaw<'a>> {
        self.argv
            .get(index)
            .map(|v| JsValueRaw::from_raw_unchecked(*v))
    }
}
