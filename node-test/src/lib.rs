#[macro_use]
extern crate node;

use node::nodeinit;
use node::prelude::*;
use std::thread;
use std::time::Duration;

fn hello<'a>(env: Env<'a>, info: CallbackInfo<'a>) -> JsResult<JsString<'a>> {
    let name: String = info.arg::<JsString<'a>>(env, 0)?.cast(env)?;
    let s: JsString<'a> = format!("hello {}", name).cast(env)?;
    Ok(s)
}

struct AddCaller;

impl JsCaller<i32> for AddCaller {
    fn call<'a>(&self, env: Env<'a>, func: JsFunction<'a>, data: i32) {
        let num: JsNumber<'a> = data.cast(env).unwrap();
        func.call_r(env, &func, js_argv![num]).unwrap();
    }
}

fn add_slow<'a>(env: Env<'a>, info: CallbackInfo<'a>) -> JsResult<JsUndefined<'a>> {
    let a = info.arg_i32(env, 0)?;
    let b = info.arg_i32(env, 1)?;
    let cb = info.arg::<JsFunction<'a>>(env, 2)?;
    let ts_func = AddCaller.make_ts_func(env, cb)?;
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        ts_func.call(a + b).unwrap();
    });
    env.undefined()
}

struct WrapData {
    n: i32,
}

fn make_wrap<'a>(env: Env<'a>, mut info: CallbackInfo<'a>) -> JsResult<JsUndefined<'a>> {
    let n = info.arg_i32(env, 0)?;
    JsWrap::wrap(env, &mut info.this, WrapData { n })?;
    env.undefined()
}

fn get_wrap<'a>(env: Env<'a>, info: CallbackInfo<'a>) -> JsResult<JsNumber<'a>> {
    let obj: JsWrap<'a, WrapData> = info.arg(env, 0)?;
    let data: &WrapData = obj.as_ref(env)?;
    data.n.cast(env)
}

#[nodeinit]
fn addon<'a>(env: Env<'a>, mut exports: JsObject<'a>) -> JsResult<JsObject<'a>> {
    exports.set_function(env, "hello", hello)?;
    exports.set_function(env, "add_slow", add_slow)?;
    exports.set_function(env, "make_wrap", make_wrap)?;
    exports.set_function(env, "get_wrap", get_wrap)?;
    Ok(exports)
}
