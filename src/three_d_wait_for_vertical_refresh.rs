use std::mem::MaybeUninit;

use super::{
    ffi,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
};

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_wait_for_vertical_refresh/>
#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLX3DWaitForVerticalRefresh")]
pub struct ThreeDWaitForVerticalRefresh(InterfaceImpl);

unsafe impl Interface for ThreeDWaitForVerticalRefresh {
    type Impl = ffi::IADLX3DWaitForVerticalRefresh;
    type Vtable = ffi::IADLX3DWaitForVerticalRefreshVtbl;
    const IID: &'static str = "IADLX3DWaitForVerticalRefresh";
}

impl ThreeDWaitForVerticalRefresh {
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_wait_for_vertical_refresh__is_supported/>
    #[doc(alias = "IsSupported")]
    pub fn is_supported(&self) -> Result<bool> {
        let mut supported = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().IsSupported.unwrap())(self.as_raw(), supported.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, supported).map(|s| s != 0)
    }

    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_wait_for_vertical_refresh__is_enabled/>
    #[doc(alias = "IsEnabled")]
    pub fn is_enabled(&self) -> Result<bool> {
        let mut enabled = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().IsEnabled.unwrap())(self.as_raw(), enabled.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, enabled).map(|e| e != 0)
    }

    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_wait_for_vertical_refresh__get_mode/>
    #[doc(alias = "GetMode")]
    pub fn get_mode(&self) -> Result<ffi::ADLX_WAIT_FOR_VERTICAL_REFRESH_MODE> {
        let mut mode = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().GetMode.unwrap())(self.as_raw(), mode.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, mode)
    }

    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_wait_for_vertical_refresh__set_mode/>
    #[doc(alias = "SetMode")]
    pub fn set_mode(&mut self, mode: ffi::ADLX_WAIT_FOR_VERTICAL_REFRESH_MODE) -> Result<()> {
        let result = unsafe { (self.vtable().SetMode.unwrap())(self.as_raw(), mode) };

        Error::from_result(result)
    }
}
