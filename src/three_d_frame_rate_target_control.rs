use std::mem::MaybeUninit;

use super::{
    ffi,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
};

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_frame_rate_target_control/>
#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLX3DFrameRateTargetControl")]
pub struct ThreeDFrameRateTargetControl(InterfaceImpl);

unsafe impl Interface for ThreeDFrameRateTargetControl {
    type Impl = ffi::IADLX3DFrameRateTargetControl;
    type Vtable = ffi::IADLX3DFrameRateTargetControlVtbl;
    const IID: &'static str = "IADLX3DFrameRateTargetControl";
}

impl ThreeDFrameRateTargetControl {
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_frame_rate_target_control__is_supported/>
    #[doc(alias = "IsSupported")]
    pub fn is_supported(&self) -> Result<bool> {
        let mut supported = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().IsSupported.unwrap())(self.as_raw(), supported.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, supported).map(|s| s != 0)
    }

    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_frame_rate_target_control__is_enabled/>
    #[doc(alias = "IsEnabled")]
    pub fn is_enabled(&self) -> Result<bool> {
        let mut enabled = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().IsEnabled.unwrap())(self.as_raw(), enabled.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, enabled).map(|e| e != 0)
    }

    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_frame_rate_target_control__get_f_p_s_range/>
    #[doc(alias = "GetFPSRange")]
    pub fn get_fps_range(&self) -> Result<ffi::ADLX_IntRange> {
        let mut fps_range = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().GetFPSRange.unwrap())(self.as_raw(), fps_range.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, fps_range)
    }

    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_frame_rate_target_control__get_f_p_s/>
    #[doc(alias = "GetFPS")]
    pub fn get_fps(&self) -> Result<i32> {
        let mut fps = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().GetFPS.unwrap())(self.as_raw(), fps.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, fps)
    }
}
