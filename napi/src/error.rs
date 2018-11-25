use crate::env::Env;
use napi_sys::{self, napi_status};
use std::ffi::CString;
use std::mem;
use std::ptr;

#[derive(Debug)]
pub enum JsError {
    Error { code: napi_status, message: CString },
    PendingException,
}

impl JsError {
    pub fn is_pending_exception(&self) -> bool {
        match self {
            JsError::PendingException => true,
            _ => false,
        }
    }

    pub fn from_env<'a>(env: Env<'a>) -> JsError {
        unsafe {
            let mut info: *const napi_sys::napi_extended_error_info = mem::zeroed();
            let _ = napi_sys::napi_get_last_error_info(env.env, &mut info);
            let code = (*info).error_code;
            let message = (*info).error_message;

            let mut is_exc_pending = false;
            let _ = napi_sys::napi_is_exception_pending(env.env, &mut is_exc_pending);
            if is_exc_pending {
                return JsError::PendingException;
            }
            JsError::Error {
                code,
                message: CString::from_raw(message as *mut i8),
            }
        }
    }

    pub fn throw<'a>(&self, env: Env<'a>) {
        match self {
            JsError::Error {
                code: _,
                ref message,
            } => unsafe {
                let _ = napi_sys::napi_throw_error(env.env, ptr::null(), message.as_ptr());
            },
            JsError::PendingException => {}
        }
    }
}

#[macro_export]
macro_rules! node_try {
    ($func:path, $env:expr, $($x:expr),*) => {
        let code = $func($env.env, $($x),*);
        if code != ::napi_sys::Status::Ok {
            return Err(crate::error::JsError::from_env($env))
        }
    }
}
