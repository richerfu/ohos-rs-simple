#![allow(non_camel_case_types)]
use std::os::raw::{c_char, c_int, c_uint, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_value__ {
    _unused: [u8; 0],
}

pub type napi_value = *mut napi_value__;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_env__ {
    _unused: [u8; 0],
}

pub type napi_env = *mut napi_env__;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_callback_info__ {
    _unused: [u8; 0],
}
pub type napi_callback_info = *mut napi_callback_info__;

pub type napi_callback =
    Option<unsafe extern "C" fn(env: napi_env, info: napi_callback_info) -> napi_value>;

pub type napi_property_attributes = i32;

pub type napi_addon_register_func =
    Option<unsafe extern "C" fn(env: napi_env, exports: napi_value) -> napi_value>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct napi_property_descriptor {
    pub utf8name: *const c_char,
    pub name: napi_value,
    pub method: napi_callback,
    pub getter: napi_callback,
    pub setter: napi_callback,
    pub value: napi_value,
    pub attributes: napi_property_attributes,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_module {
    pub nm_version: c_int,
    pub nm_flags: c_uint,
    pub nm_filename: *const c_char,
    pub nm_register_func: napi_addon_register_func,
    pub nm_modname: *const c_char,
    pub nm_priv: *mut c_void,
    pub reserved: [*mut c_void; 4usize],
}
