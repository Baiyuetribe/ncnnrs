// 包含生成的绑定
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ncnn_bind {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod allocator;
mod datareader;
mod extractor;
mod mat;
mod net;
mod option;

pub use allocator::*;
pub use datareader::*;
pub use extractor::*;
pub use mat::*;
pub use net::*;
pub use option::*;

use std::ffi::CStr;

pub fn version() -> &'static str {
    let c_buf = unsafe { ncnn_bind::ncnn_version() };
    let c_str = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap_or("unknown");
    str_slice
}

pub fn get_gpu_count() -> i32 {
    let res = unsafe { ncnn_bind::ncnn_get_gpu_count() };
    res
}

pub fn destroy_gpu_instance() {
    unsafe { ncnn_bind::ncnn_destroy_gpu_instance() };
}

pub fn get_gpu_heap_budget(index: i32) -> u32 {
    let device = unsafe { ncnn_bind::ncnn_get_gpu_device(index) };
    let res = unsafe { ncnn_bind::ncnn_VulkanDevice_get_heap_budget(device) };
    res
}

pub fn get_device_name(index: i32) -> &'static str {
    let info = unsafe { ncnn_bind::ncnn_get_gpu_info(index) };
    let res: *const i8 = unsafe { ncnn_bind::ncnn_GpuInfo_device_name(info) };
    let c_str = unsafe { CStr::from_ptr(res) };
    let str_slice: &str = c_str.to_str().unwrap_or("unknown");
    str_slice
}

#[test]
fn test_version() {
    println!("ncnn version: {}", version());
}

#[test]
fn test_some() {
    let res = unsafe { ncnn_bind::ncnn_get_gpu_count() };
    println!("cpu count: {}", res);
}
