use crate::env::Env;
use napi_sys::napi_env;
use std::ffi::c_void;
use std::marker::PhantomData;

pub trait JsFinalize<D> {
    fn finalize<'a>(env: Env<'a>, data: &mut D);
}

pub(crate) extern "C" fn js_finalize_cb<D: Sized, F: JsFinalize<D>>(
    env: napi_env,
    data: *mut c_void,
    _hint: *mut c_void,
) {
    unsafe {
        let env = Env::from_raw(env);
        let mut data: Box<D> = Box::from_raw(data as *mut D);
        F::finalize(env, &mut data);
    }
}

pub struct DropDataFinalizer<D> {
    _m: PhantomData<D>,
}

impl<D> JsFinalize<D> for DropDataFinalizer<D> {
    fn finalize<'a>(_env: Env<'a>, _data: &mut D) {
        //
    }
}
