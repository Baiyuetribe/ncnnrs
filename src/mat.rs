use crate::allocator::Allocator;
use crate::ncnn_bind::*;
use core::fmt;

use std::os::raw::c_void;

const PIXEL_CONVERT_SHIFT: u32 = 16;

pub enum MatPixelType {
    BGR,
    BGRA,
    GRAY,
    RGB,
    RGBA,

    RGBtoBGR,
    RGBtoGRAY,
    RGBtoRGBA,
    RGBtoBGRA,

    BGRtoRGB,
    BGRtoGRAY,
    BGRtoRGBA,
    BGRtoBGRA,

    GRAYtoRGB,
    GRAYtoBGR,
    GRAYtoRGBA,
    GRAYtoBGRA,

    RGBAtoRGB,
    RGBAtoBGR,
    RGBAtoGRAY,
    RGBAtoBGRA,

    BGRAtoRGB,
    BGRAtoBGR,
    BGRAtoGRAY,
    BGRAtoRGBA,
}

impl MatPixelType {
    fn to_int(&self) -> i32 {
        match self {
            MatPixelType::BGR => NCNN_MAT_PIXEL_BGR as _,
            MatPixelType::BGRA => NCNN_MAT_PIXEL_BGRA as _,
            MatPixelType::GRAY => NCNN_MAT_PIXEL_GRAY as _,
            MatPixelType::RGB => NCNN_MAT_PIXEL_RGB as _,
            MatPixelType::RGBA => NCNN_MAT_PIXEL_RGBA as _,
            MatPixelType::RGBtoBGR => {
                (NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::RGBtoGRAY => {
                (NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::RGBtoRGBA => {
                (NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::RGBtoBGRA => {
                (NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BGRtoRGB => {
                (NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BGRtoGRAY => {
                (NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BGRtoRGBA => {
                (NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BGRtoBGRA => {
                (NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::GRAYtoRGB => {
                (NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::GRAYtoBGR => {
                (NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::GRAYtoRGBA => {
                (NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::GRAYtoBGRA => {
                (NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::RGBAtoRGB => {
                (NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::RGBAtoBGR => {
                (NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::RGBAtoGRAY => {
                (NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::RGBAtoBGRA => {
                (NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BGRAtoRGB => {
                (NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BGRAtoBGR => {
                (NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BGRAtoGRAY => {
                (NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BGRAtoRGBA => {
                (NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)) as _
            }
        }
    }

    fn stride(&self) -> i32 {
        match self {
            MatPixelType::BGR
            | MatPixelType::BGRtoBGRA
            | MatPixelType::BGRtoGRAY
            | MatPixelType::BGRtoRGB
            | MatPixelType::BGRtoRGBA => 3,
            MatPixelType::BGRA
            | MatPixelType::BGRAtoBGR
            | MatPixelType::BGRAtoGRAY
            | MatPixelType::BGRAtoRGB
            | MatPixelType::BGRAtoRGBA => 4,
            MatPixelType::GRAY
            | MatPixelType::GRAYtoBGR
            | MatPixelType::GRAYtoBGRA
            | MatPixelType::GRAYtoRGB
            | MatPixelType::GRAYtoRGBA => 1,
            MatPixelType::RGB
            | MatPixelType::RGBtoBGR
            | MatPixelType::RGBtoBGRA
            | MatPixelType::RGBtoGRAY
            | MatPixelType::RGBtoRGBA => 3,
            MatPixelType::RGBA
            | MatPixelType::RGBAtoBGR
            | MatPixelType::RGBAtoBGRA
            | MatPixelType::RGBAtoGRAY
            | MatPixelType::RGBAtoRGB => 4,
        }
    }
}

pub struct Mat {
    ptr: ncnn_mat_t,
}

// Mat is basically a glorified atomically refcounted matrix.
unsafe impl Send for Mat {}

impl Mat {
    /// Constructs an empty matrix.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs an empty 1D matrix.
    pub fn new_1d(w: i32, alloc: Option<&Allocator>) -> Self {
        Self {
            ptr: unsafe {
                ncnn_mat_create_1d(
                    w,
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        }
    }

    /// Constructs an empty 2D matrix.
    pub fn new_2d(w: i32, h: i32, alloc: Option<&Allocator>) -> Self {
        Self {
            ptr: unsafe {
                ncnn_mat_create_2d(
                    w,
                    h,
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        }
    }

    /// Constructs an empty 3D matrix.
    pub fn new_3d(w: i32, h: i32, c: i32, alloc: Option<&Allocator>) -> Self {
        Self {
            ptr: unsafe {
                ncnn_mat_create_3d(
                    w,
                    h,
                    c,
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        }
    }

    /// Constructs an empty 4D matrix.
    pub fn new_4d(w: i32, h: i32, d: i32, c: i32, alloc: Option<&Allocator>) -> Self {
        Self {
            ptr: unsafe {
                ncnn_mat_create_4d(
                    w,
                    h,
                    d,
                    c,
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        }
    }

    /// Constructs 1D matrix with a given raw data.
    ///
    /// # Safety
    ///
    /// Data pointer must not be aliased, it must be valid for the entire lifetime of Mat and it must be of correct size.
    pub unsafe fn new_external_1d(w: i32, data: *mut c_void, alloc: Option<&Allocator>) -> Self {
        Self {
            ptr: ncnn_mat_create_external_1d(
                w,
                data,
                alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
            ),
        }
    }

    /// Constructs 2D matrix with a given raw data.
    ///
    /// # Safety
    ///
    /// Data pointer must not be aliased, it must be valid for the entire lifetime of Mat and it must be of correct size.
    pub unsafe fn new_external_2d(
        w: i32,
        h: i32,
        data: *mut c_void,
        alloc: Option<&Allocator>,
    ) -> Self {
        Self {
            ptr: ncnn_mat_create_external_2d(
                w,
                h,
                data,
                alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
            ),
        }
    }

    /// Constructs 3D matrix with a given raw data.
    ///
    /// # Safety
    ///
    /// Data pointer must not be aliased, it must be valid for the entire lifetime of Mat and it must be of correct size.
    pub unsafe fn new_external_3d(
        w: i32,
        h: i32,
        c: i32,
        data: *mut c_void,
        alloc: Option<&Allocator>,
    ) -> Self {
        Self {
            ptr: ncnn_mat_create_external_3d(
                w,
                h,
                c,
                data,
                alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
            ),
        }
    }

    /// Constructs 4D matrix with a given raw data.
    ///
    /// # Safety
    ///
    /// Data pointer must not be aliased, it must be valid for the entire lifetime of Mat and it must be of correct size.
    pub unsafe fn new_external_4d(
        w: i32,
        h: i32,
        d: i32,
        c: i32,
        data: *mut c_void,
        alloc: Option<&Allocator>,
    ) -> Self {
        Self {
            ptr: ncnn_mat_create_external_4d(
                w,
                h,
                d,
                c,
                data,
                alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
            ),
        }
    }

    /// Constructs matrix from a pixel byte array
    pub fn from_pixels(
        data: &[u8],
        pixel_type: MatPixelType,
        width: i32,
        height: i32,
        alloc: Option<&Allocator>,
    ) -> anyhow::Result<Mat> {
        let len = width * height * pixel_type.stride();
        if data.len() != len as _ {
            anyhow::bail!("Expected data length {}, provided {}", len, data.len());
        }

        Ok(Self {
            ptr: unsafe {
                ncnn_mat_from_pixels(
                    data.as_ptr(),
                    pixel_type.to_int(),
                    width,
                    height,
                    width * pixel_type.stride(),
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        })
    }

    /// Constructs matrix from resizing a pixel byte array.
    pub fn from_pixels_resize(
        data: &[u8],
        pixel_type: MatPixelType,
        width: i32,
        height: i32,
        target_width: i32,
        target_height: i32,
        alloc: Option<&Allocator>,
    ) -> anyhow::Result<Mat> {
        let len = width * height * pixel_type.stride();
        if data.len() != len as _ {
            anyhow::bail!("Expected data length {}, provided {}", len, data.len());
        }

        Ok(Self {
            ptr: unsafe {
                ncnn_mat_from_pixels_resize(
                    data.as_ptr(),
                    pixel_type.to_int(),
                    width,
                    height,
                    width * pixel_type.stride(),
                    target_width,
                    target_height,
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        })
    }

    pub fn substract_mean_normalize(&mut self, mean_vals: &[f32], norm_vals: &[f32]) {
        let channels = self.c() as usize;
        assert_eq!(mean_vals.len(), channels);
        assert_eq!(norm_vals.len(), channels);
        unsafe {
            ncnn_mat_substract_mean_normalize(self.ptr, mean_vals.as_ptr(), norm_vals.as_ptr())
        }
    }

    /// Fills matrix with a given value.
    pub fn fill(&mut self, value: f32) {
        unsafe { ncnn_mat_fill_float(self.ptr, value) };
    }

    /// Returns number of matrix dimensions.
    pub fn dims(&self) -> i32 {
        unsafe { ncnn_mat_get_dims(self.ptr) }
    }

    /// Returns matrix width.
    pub fn w(&self) -> i32 {
        unsafe { ncnn_mat_get_w(self.ptr) }
    }

    /// Returns matrix height.
    pub fn h(&self) -> i32 {
        unsafe { ncnn_mat_get_h(self.ptr) }
    }

    /// Returns matrix depth.
    pub fn d(&self) -> i32 {
        unsafe { ncnn_mat_get_d(self.ptr) }
    }

    /// Returns matrix channels.
    pub fn c(&self) -> i32 {
        unsafe { ncnn_mat_get_c(self.ptr) }
    }

    pub fn elemsize(&self) -> u64 {
        (unsafe { ncnn_mat_get_elemsize(self.ptr) }) as u64
    }

    pub fn elempack(&self) -> i32 {
        unsafe { ncnn_mat_get_elempack(self.ptr) }
    }

    pub fn cstep(&self) -> usize {
        #[cfg(target_pointer_width = "32")]
        {
            unsafe { ncnn_mat_get_cstep(self.ptr) as u32 as usize }
        }

        #[cfg(target_pointer_width = "64")]
        {
            unsafe { ncnn_mat_get_cstep(self.ptr) as u64 as usize }
        }
    }

    /// Pointer to raw matrix data
    pub fn data(&self) -> *mut ::std::os::raw::c_void {
        unsafe { ncnn_mat_get_data(self.ptr) }
    }

    pub(crate) fn ptr(&self) -> ncnn_mat_t {
        self.ptr
    }

    pub(crate) fn mut_ptr(&mut self) -> *mut ncnn_mat_t {
        &mut self.ptr
    }
}

impl Default for Mat {
    fn default() -> Self {
        Self {
            ptr: unsafe { ncnn_mat_create() },
        }
    }
}

impl fmt::Debug for Mat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mat")
            .field("dims", &self.dims())
            .field("c", &self.c())
            .field("h", &self.h())
            .field("w", &self.w())
            .field("elemsize", &self.elemsize())
            .field("elempack", &self.elempack())
            .field("cstep", &self.cstep())
            .finish()
    }
}

impl Drop for Mat {
    fn drop(&mut self) {
        unsafe {
            ncnn_mat_destroy(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Mat;

    #[test]
    fn basic_getter_and_setter() {
        let m: Mat = Mat::new_3d(224, 224, 3, None);
        assert_eq!(224, m.h());
        assert_eq!(224, m.w());
        assert_eq!(3, m.c());
    }
}
