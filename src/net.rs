use crate::datareader::DataReader;
use crate::ncnn_bind::*;
use crate::Extractor;
use std::ffi::CString;
use std::os::raw::c_int;

pub struct Net {
    ptr: ncnn_net_t,
}

unsafe impl Send for Net {}
unsafe impl Sync for Net {}

impl Net {
    pub fn new() -> Net {
        Net {
            ptr: unsafe { ncnn_net_create() },
        }
    }

    pub fn set_option(&mut self, opt: &crate::option::Option) {
        unsafe {
            ncnn_net_set_option(self.ptr, opt.ptr());
        }
    }

    pub fn set_vulkan_device(&mut self, device_index: u32) {
        #[cfg(not(feature = "cpu"))]
        unsafe {
            ncnn_net_set_vulkan_device(self.ptr, device_index as c_int);
        }
    }

    pub fn load_param(&mut self, path: &str) -> anyhow::Result<()> {
        let c_str = {
            #[cfg(target_os = "windows")]
            {
                let (gbk_bytes, _, _) = encoding_rs::GB18030.encode(path);
                CString::new(gbk_bytes)?
            }
            #[cfg(not(target_os = "windows"))]
            {
                CString::new(path)?
            }
        };
        if unsafe { ncnn_net_load_param(self.ptr, c_str.as_ptr()) } != 0 {
            #[cfg(target_os = "windows")] // 当Windows为utf-8编码时，再尝试一次
            {
                let c_str = CString::new(path)?;
                if unsafe { ncnn_net_load_param(self.ptr, c_str.as_ptr()) } != 0 {
                    return anyhow::bail!("Error loading params {}", path);
                } else {
                    return Ok(());
                }
            }
            anyhow::bail!("Error loading params {}", path);
        } else {
            Ok(())
        }
    }

    pub fn load_param_memory(&mut self, param_data: &[u8]) -> anyhow::Result<()> {
        let c_str =
            CString::new(param_data).map_err(|e| anyhow::anyhow!("Invalid param data: {}", e))?;
        let result = unsafe { ncnn_net_load_param_memory(self.ptr, c_str.as_ptr()) };
        if result != 0 {
            anyhow::bail!("Error loading params from memory");
        } else {
            Ok(())
        }
    }
    pub fn load_model(&mut self, path: &str) -> anyhow::Result<()> {
        let c_str = {
            #[cfg(target_os = "windows")]
            {
                let (gbk_bytes, _, _) = encoding_rs::GB18030.encode(path);
                CString::new(gbk_bytes)?
            }
            #[cfg(not(target_os = "windows"))]
            {
                CString::new(path)?
            }
        };
        if unsafe { ncnn_net_load_model(self.ptr, c_str.as_ptr()) } != 0 {
            #[cfg(target_os = "windows")] // 当Windows为utf-8编码时，再尝试一次
            {
                let c_str = CString::new(path)?;
                if unsafe { ncnn_net_load_model(self.ptr, c_str.as_ptr()) } != 0 {
                    return anyhow::bail!("Error loading model {}", path);
                } else {
                    return Ok(());
                }
            }
            anyhow::bail!("Error loading model {}", path);
        } else {
            Ok(())
        }
    }

    pub fn load_model_datareader(&mut self, dr: &DataReader) -> anyhow::Result<()> {
        if unsafe { ncnn_net_load_model_datareader(self.ptr, dr.ptr()) } != 0 {
            anyhow::bail!("Error loading model from datareader");
        } else {
            Ok(())
        }
    }

    pub fn create_extractor(&self) -> Extractor<'_> {
        let ptr;
        unsafe {
            ptr = ncnn_extractor_create(self.ptr);
        }
        Extractor::from_ptr(ptr)
    }
}

impl Drop for Net {
    fn drop(&mut self) {
        unsafe {
            ncnn_net_destroy(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_send<T: Send>() -> bool {
        true
    }
    fn is_sync<T: Sync>() -> bool {
        true
    }

    #[test]
    fn load_not_exist_model() {
        let mut net = Net::new();
        net.load_param("not_exist.param")
            .expect_err("Expected param to be not found");
    }

    #[test]
    fn check_sync_send() {
        assert!(is_send::<Net>());
        //assert!(is_sync::<Net>());
    }
}
