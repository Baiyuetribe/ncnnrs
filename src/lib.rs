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
    // let c_buf = unsafe { ffi::ncnn_version() };
    let c_buf = unsafe { ncnn_bind::ncnn_version() };
    let c_str = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    str_slice
}

#[test]
fn test_version() {
    println!("ncnn version: {}", version());
}
