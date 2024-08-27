use std::mem::MaybeUninit;

use super::{
    ffi,
    gpu::Gpu,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
    three_d_wait_for_vertical_refresh::ThreeDWaitForVerticalRefresh,
};

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_settings_services/>
#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLX3DSettingsServices")]
pub struct ThreeDSettingsServices(InterfaceImpl);

unsafe impl Interface for ThreeDSettingsServices {
    type Impl = ffi::IADLX3DSettingsServices;
    type Vtable = ffi::IADLX3DSettingsServicesVtbl;
    const IID: &'static str = "IADLX3DSettingsServices";
}

impl ThreeDSettingsServices {
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x3_d_settings_services__get_wait_for_vertical_refresh/>
    #[doc(alias = "GetWaitForVerticalRefresh")]
    pub fn get_wait_for_vertical_refresh(&self, gpu: &Gpu) -> Result<ThreeDWaitForVerticalRefresh> {
        let mut wait_for_vertical_refresh = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GetWaitForVerticalRefresh.unwrap())(
                self.as_raw(),
                gpu.as_raw(),
                wait_for_vertical_refresh.as_mut_ptr(),
            )
        };
        Error::from_result_with_assume_init_on_success(result, wait_for_vertical_refresh).map(
            |wait_for_vertical_refresh| unsafe {
                ThreeDWaitForVerticalRefresh::from_raw(wait_for_vertical_refresh)
            },
        )
    }
}
