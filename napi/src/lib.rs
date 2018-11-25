pub extern crate napi_sys;

pub use napi_sys as sys;

#[macro_use]
pub mod error;

pub mod callback;
pub mod env;
pub mod finalize;
pub mod promise;
pub mod ts_func;
pub mod types;
pub mod value;

pub type JsResult<T> = Result<T, error::JsError>;

pub mod prelude {
    pub use crate::callback::{Callback, CallbackInfo};
    pub use crate::env::Env;
    pub use crate::error::JsError;
    pub use crate::finalize::JsFinalize;
    pub use crate::promise::JsPromise;
    pub use crate::ts_func::{JsCaller, ThreadSafeFunction, TsError};
    pub use crate::types::*;
    pub use crate::value::{CastToJs, CastToRust, IntoRawJsValue, JsValue, JsValueRaw};
    pub use crate::JsResult;
}
