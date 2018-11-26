#![recursion_limit = "256"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn nodeinit(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast: syn::ItemFn = syn::parse(input).expect("#[nodeinit] must be used on a function");
    let fname = ast.ident.clone();
    let modname: syn::Ident;
    if attr.is_empty() {
        modname = ast.ident.clone();
    } else {
        modname = syn::parse(attr).expect("could not parse module name");
    }
    quote!(
        #ast

        #[allow(improper_ctypes)]
        #[cfg_attr(target_os = "linux", link_section = ".ctors")]
        #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
        #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
        pub static __LOAD_NODE_MODULE: unsafe extern "C" fn() = {
            use std::mem;
            use std::ffi::c_void;
            use std::os::raw::c_char;
            use node::sys::{napi_env, napi_value, napi_module};
            use node::env::Env;
            use node::value::{IntoRawJsValue};
            use node::types::JsObject;

            unsafe extern "C" fn __node_module_init(env: napi_env, exports: napi_value) -> napi_value {
                let env = Env::from_raw(env);
                let exports = match JsObject::from_raw(env, exports) {
                    Ok(x) => x,
                    Err(e) => {
                        e.throw(env);
                        return 0 as napi_value;
                    }
                };
                let result = #fname(env, exports);
                match result {
                    Ok(exports) => exports.into_raw_js_value(),
                    Err(e) => {
                        e.throw(env);
                        0 as napi_value
                    }
                }
            }

            unsafe extern "C" fn __load_node_module() {
                static mut __NODE_MODULE: node::sys::napi_module = node::sys::napi_module {
                    nm_version: node::sys::NAPI_MODULE_VERSION as i32,
                    nm_flags: 0,
                    nm_filename: b"node_module.rs\0" as *const u8 as *const c_char,
                    nm_register_func: Some(__node_module_init),
                    nm_modname: concat!(stringify!(#modname), "\0").as_ptr() as *const c_char,
                    nm_priv: 0 as *mut c_void,
                    reserved: [0 as *mut c_void; 4],
                };
                node::sys::napi_module_register(&mut __NODE_MODULE);
            }
            __load_node_module
        };
    )
    .into()
}
