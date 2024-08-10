use crate::ncnn_bind::*;
use std::os::raw::c_int;

pub struct Option {
    ptr: ncnn_option_t,
}

impl Option {
    pub fn new() -> Option {
        let ptr;
        unsafe {
            ptr = ncnn_option_create();
        }
        Option { ptr }
    }

    pub fn set_num_threads(&mut self, num_threads: u32) {
        unsafe {
            ncnn_option_set_num_threads(self.ptr, num_threads as c_int);
        }
    }

    pub fn get_num_threads(&self) -> u32 {
        unsafe { ncnn_option_get_num_threads(self.ptr) as u32 }
    }

    pub fn use_vulkan_compute(&mut self, enabled: bool) {
        unsafe {
            ncnn_option_set_use_vulkan_compute(self.ptr, enabled as c_int);
        }
    }

    pub fn get_vulkan_compute(&self) -> bool {
        unsafe { ncnn_option_get_use_vulkan_compute(self.ptr) != 0 }
    }

    pub(crate) fn ptr(&self) -> ncnn_option_t {
        self.ptr
    }
}

impl Drop for Option {
    fn drop(&mut self) {
        unsafe {
            ncnn_option_destroy(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_cpu_info() {
        use crate::option::*;
        let mut opt = Option::new();
        opt.set_num_threads(4);
        assert_eq!(4, opt.get_num_threads());
    }
}
