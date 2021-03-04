use crate::env::Env;
use napi_sys::napi_env;
use std::ffi::c_void;
use std::marker::PhantomData;

pub trait JsFinalize {
    type Item;

    fn finalize<'a>(env: Env<'a>, data: &mut Self::Item);

    unsafe extern "C" fn js_finalize_cb(env: napi_env, data: *mut c_void, _hint: *mut c_void) {
        let env = Env::from_raw(env);
        let mut data = Box::from_raw(data as *mut Self::Item);
        Self::finalize(env, &mut data);
    }
}

pub(crate) unsafe extern "C" fn js_drop_finalize_cb<D: Sized>(
    env: napi_env,
    data: *mut c_void,
    hint: *mut c_void,
) {
    <DropFinalizer<D> as JsFinalize>::js_finalize_cb(env, data, hint);
}

pub struct DropFinalizer<D> {
    _m: PhantomData<D>,
}

impl<D> JsFinalize for DropFinalizer<D> {
    type Item = D;

    fn finalize<'a>(_env: Env<'a>, _data: &mut D) {
        //
    }
}
