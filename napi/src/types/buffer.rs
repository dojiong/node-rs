use std::{ffi::c_void, marker::PhantomData, mem::MaybeUninit};

use napi_sys::{napi_create_buffer_copy, napi_get_buffer_info, napi_is_buffer, napi_value};

use crate::{
    env::Env,
    value::{CastToJs, CastToRust, JsValue},
    JsResult,
};

pub struct JsBuffer<'a> {
    value: napi_value,
    _m: PhantomData<&'a u8>,
}

impl<'a> JsValue<'a> for JsBuffer<'a> {
    unsafe fn as_raw(&self) -> napi_value {
        self.value
    }

    unsafe fn from_raw(env: Env<'a>, value: napi_value) -> JsResult<Self> {
        let mut is_buffer = false;
        node_try!(napi_is_buffer, env, value, &mut is_buffer);
        if !is_buffer {
            env.throw(None, "JsBuffer from non-buffer")?;
        }
        Ok(JsBuffer {
            value,
            _m: PhantomData,
        })
    }
}

impl<'a> JsBuffer<'a> {
    pub fn copy_bytes(env: Env<'a>, bytes: &[u8]) -> JsResult<Self> {
        unsafe {
            let mut _copied_data = MaybeUninit::uninit();
            let mut value = MaybeUninit::uninit();
            node_try!(
                napi_create_buffer_copy,
                env,
                bytes.len(),
                bytes.as_ptr() as *const c_void,
                _copied_data.as_mut_ptr(),
                value.as_mut_ptr()
            );
            Ok(Self {
                value: value.assume_init(),
                _m: PhantomData,
            })
        }
    }

    pub fn as_bytes(&self, env: Env<'a>) -> JsResult<&[u8]> {
        unsafe {
            let mut data = MaybeUninit::uninit();
            let mut size = MaybeUninit::uninit();
            node_try!(
                napi_get_buffer_info,
                env,
                self.value,
                data.as_mut_ptr(),
                size.as_mut_ptr()
            );
            Ok(std::slice::from_raw_parts(
                data.assume_init() as *const u8,
                size.assume_init(),
            ))
        }
    }
}

impl<'a> CastToRust<'a, Vec<u8>> for JsBuffer<'a> {
    fn cast(&self, env: crate::env::Env<'a>) -> JsResult<Vec<u8>> {
        Ok(Vec::from(self.as_bytes(env)?))
    }
}

impl<'a, 'b> CastToJs<'a, JsBuffer<'a>> for &'b [u8] {
    fn cast(&self, env: Env<'a>) -> JsResult<JsBuffer<'a>> {
        JsBuffer::copy_bytes(env, self)
    }
}

impl<'a> CastToJs<'a, JsBuffer<'a>> for Vec<u8> {
    fn cast(&self, env: Env<'a>) -> JsResult<JsBuffer<'a>> {
        JsBuffer::copy_bytes(env, self)
    }
}
