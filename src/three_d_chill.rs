use std::mem::MaybeUninit;

use super::{
    ffi,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
};

#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLX3DChill")]
pub struct ThreeDChill(InterfaceImpl);

unsafe impl Interface for ThreeDChill {
    type Impl = ffi::IADLX3DChill;
    type Vtable = ffi::IADLX3DChillVtbl;
    const IID: &'static str = "IADLX3DChill";
}

impl ThreeDChill {
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_chill__is_supported/>
    #[doc(alias = "IsSupported")]
    pub fn is_supported(&self) -> Result<bool> {
        let mut supported = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().IsSupported.unwrap())(self.as_raw(), supported.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, supported).map(|s| s != 0)
    }

    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_chill__is_enabled/>
    #[doc(alias = "IsEnabled")]
    pub fn is_enabled(&self) -> Result<bool> {
        let mut enabled = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().IsEnabled.unwrap())(self.as_raw(), enabled.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, enabled).map(|e| e != 0)
    }

    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_chill__get_f_p_s_range/>
    #[doc(alias = "GetFPSRange")]
    pub fn get_fps_range(&self) -> Result<ffi::ADLX_IntRange> {
        let mut fps_range = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().GetFPSRange.unwrap())(self.as_raw(), fps_range.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, fps_range)
    }

    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_chill__get_min_f_p_s/>
    #[doc(alias = "GetMinFPS")]
    pub fn get_min_fps(&self) -> Result<i32> {
        let mut min_fps = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().GetMinFPS.unwrap())(self.as_raw(), min_fps.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, min_fps)
    }

    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_chill__get_max_f_p_s/>
    #[doc(alias = "GetMaxFPS")]
    pub fn get_max_fps(&self) -> Result<i32> {
        let mut max_fps = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().GetMaxFPS.unwrap())(self.as_raw(), max_fps.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, max_fps)
    }
}
