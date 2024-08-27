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
    if c_buf.is_null() {
        return "unknown";
    }
    let c_str = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap_or("unknown");
    str_slice
}

pub fn get_gpu_count() -> i32 {
    #[cfg(feature = "cpu")]
    return 0;
    #[cfg(not(feature = "cpu"))]
    {
        let res = unsafe { ncnn_bind::ncnn_get_gpu_count() };
        res
    }
}

pub fn destroy_gpu_instance() {
    #[cfg(not(feature = "cpu"))]
    unsafe {
        ncnn_bind::ncnn_destroy_gpu_instance()
    };
}

pub fn get_gpu_heap_budget(index: i32) -> u32 {
    #[cfg(feature = "cpu")]
    return 0;
    #[cfg(not(feature = "cpu"))]
    {
        let device = unsafe { ncnn_bind::ncnn_get_gpu_device(index) };
        if device.is_null() {
            return 0;
        }
        let res = unsafe { ncnn_bind::ncnn_VulkanDevice_get_heap_budget(device) };
        res
    }
}

pub fn get_device_name(index: i32) -> &'static str {
    #[cfg(feature = "cpu")]
    {
        return "cpu-only";
    }
    #[cfg(not(feature = "cpu"))]
    {
        let info = unsafe { ncnn_bind::ncnn_get_gpu_info(index) };
        if info.is_null() {
            return "unknown";
        }
        let res = unsafe { ncnn_bind::ncnn_GpuInfo_device_name(info) };
        if res.is_null() {
            return "unknown";
        }
        let c_str = unsafe { CStr::from_ptr(res) };
        let str_slice: &str = c_str.to_str().unwrap_or("unknown");
        str_slice
    }
}

// 填充
pub fn copy_make_border(
    src: &crate::mat::Mat,
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
    type_: BorderType,
    v: f32,
) -> anyhow::Result<crate::mat::Mat> {
    if top < 0 || bottom < 0 || left < 0 || right < 0 {
        return Err(anyhow::anyhow!("Border sizes must be non-negative"));
    }
    let mut dst = crate::mat::Mat::new();
    unsafe {
        ncnn_bind::ncnn_copy_make_border1(
            src.ptr(),
            dst.ptr(),
            top,
            bottom,
            left,
            right,
            type_.to_int(),
            v,
            std::ptr::null_mut(), // crate::option::Option::new().ptr(), // null ？
        )
    }
    Ok(dst)
}

#[test]
fn test_version() {
    println!("ncnn version: {}", version());
}
