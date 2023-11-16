use std::{ffi::CString, ptr};

mod sys;

pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

unsafe extern "C" fn js_add(
    env: sys::napi_env,
    callback: sys::napi_callback_info,
) -> sys::napi_value {
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    unsafe {
        let mut args = [ptr::null_mut(); 2];
        sys::napi_get_cb_info(
            env,
            callback,
            &mut 2,
            args.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        sys::napi_get_value_double(env, args[0], &mut a);
        sys::napi_get_value_double(env, args[1], &mut b);
    };
    let result = add(a, b);

    let mut js_result = ptr::null_mut();
    unsafe {
        sys::napi_create_double(env, result, &mut js_result);
    };
    js_result
}

unsafe extern "C" fn napi_register_module_v1(
    env: sys::napi_env,
    exports: sys::napi_value,
) -> sys::napi_value {
    let desc = [sys::napi_property_descriptor {
        utf8name: CString::new("add").unwrap().as_ptr().cast(),
        name: ptr::null_mut(),
        getter: None,
        setter: None,
        method: Some(js_add),
        attributes: 0,
        value: ptr::null_mut(),
        data: ptr::null_mut(),
    }];
    sys::napi_define_properties(env, exports, desc.len(), desc.as_ptr());
    exports
}

#[ctor::ctor]
fn init() {
    let name = CString::new("entry").unwrap();
    let mut modules = sys::napi_module {
        nm_version: 1,
        nm_filename: ptr::null_mut(),
        nm_flags: 0,
        nm_modname: name.as_ptr().cast(),
        nm_priv: ptr::null_mut() as *mut _,
        nm_register_func: Some(napi_register_module_v1),
        reserved: [ptr::null_mut() as *mut _; 4],
    };
    unsafe {
        sys::napi_module_register(&mut modules);
    }
}
