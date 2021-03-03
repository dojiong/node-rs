use std::{any::TypeId, ffi::c_void, marker::PhantomData, mem::MaybeUninit, ptr};

use napi_sys::{napi_env, napi_unwrap, napi_value, napi_wrap};

use crate::{env::Env, value::JsValue, JsResult};

use super::JsObject;

pub struct JsWrap<'a, T> {
    object: JsObject<'a>,
    _m: PhantomData<&'a T>,
}

impl<'a, T> JsValue<'a> for JsWrap<'a, T>
where
    T: 'static,
{
    unsafe fn as_raw(&self) -> napi_value {
        self.object.as_raw()
    }

    unsafe fn from_raw(env: Env<'a>, value: napi_value) -> crate::JsResult<Self> {
        let mut data: MaybeUninit<*mut TypeData<T>> = MaybeUninit::uninit();
        node_try!(
            napi_unwrap,
            env,
            value,
            data.as_mut_ptr() as *mut *mut c_void
        );
        (*data.assume_init()).check_type(env)?;

        Ok(JsWrap {
            object: JsObject::from_raw(env, value)?,
            _m: PhantomData,
        })
    }
}

impl<'a, T> JsWrap<'a, T>
where
    T: 'static,
{
    pub fn wrap(env: Env<'a>, object: &mut JsObject<'a>, data: T) -> JsResult<()> {
        let native = TypeData::into_boxed_raw(data);
        unsafe {
            node_try!(
                napi_wrap,
                env,
                object.as_raw(),
                native as *mut c_void,
                Some(<TypeData<T>>::finalize),
                ptr::null_mut(),
                ptr::null_mut()
            );
        }
        Ok(())
    }

    pub fn new(env: Env<'a>, mut object: JsObject<'a>, data: T) -> JsResult<Self> {
        Self::wrap(env, &mut object, data)?;
        Ok(Self {
            object,
            _m: PhantomData,
        })
    }

    pub fn make_ref<'o>(env: Env<'a>, object: &'o JsObject<'a>) -> JsResult<&'o T> {
        let mut data: MaybeUninit<*mut TypeData<T>> = MaybeUninit::uninit();
        unsafe {
            node_try!(
                napi_unwrap,
                env,
                object.as_raw(),
                data.as_mut_ptr() as *mut *mut c_void
            );
            let data = data.assume_init();
            (*data).check_type(env)?;
            Ok(&(*data).data)
        }
    }

    pub fn as_ref(&self, env: Env<'a>) -> JsResult<&T> {
        Self::make_ref(env, &self.object)
    }

    pub fn make_mut<'o>(env: Env<'a>, object: &'o mut JsObject<'a>) -> JsResult<&'o mut T> {
        let mut data: MaybeUninit<*mut TypeData<T>> = MaybeUninit::uninit();
        unsafe {
            node_try!(
                napi_unwrap,
                env,
                object.as_raw(),
                data.as_mut_ptr() as *mut *mut c_void
            );
            let data = data.assume_init();
            (*data).check_type(env)?;
            Ok(&mut (*data).data)
        }
    }

    pub fn as_mut(&mut self, env: Env<'a>) -> JsResult<&mut T> {
        Self::make_mut(env, &mut self.object)
    }
}

struct TypeData<T> {
    type_id: TypeId,
    data: T,
}

impl<T: 'static> TypeData<T> {
    fn into_boxed_raw(data: T) -> *mut Self {
        let result = Box::new(Self {
            type_id: TypeId::of::<T>(),
            data,
        });
        Box::into_raw(result)
    }

    unsafe extern "C" fn finalize<'a>(_env: napi_env, raw: *mut c_void, _hint: *mut c_void) {
        let _data = Box::from_raw(raw as *mut Self);
    }

    fn type_match(&self) -> bool {
        self.type_id == TypeId::of::<T>()
    }

    fn check_type<'a>(&self, env: Env<'a>) -> JsResult<()> {
        if !self.type_match() {
            env.throw(None, "object unwrap fail: type mismatch")?;
        }
        Ok(())
    }
}
