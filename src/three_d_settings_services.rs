use std::mem::MaybeUninit;

use crate::{chill::Chill, frtc::FrameRateTargetControl};

use super::{
    ffi,
    gpu::Gpu,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
    three_d_wait_for_vertical_refresh::ThreeDWaitForVerticalRefresh,
};

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

    pub fn get_chill(&self, gpu: &Gpu) -> Result<Chill> {
        let mut chill = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GetChill.unwrap())(self.as_raw(), gpu.as_raw(), chill.as_mut_ptr())
        };
        Error::from_result_with_assume_init_on_success(result, chill)
            .map(|chill| unsafe { Chill::from_raw(chill) })
    }

    pub fn get_frame_rate_target_control(&self, gpu: &Gpu) -> Result<FrameRateTargetControl> {
        let mut frtc = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GetFrameRateTargetControl.unwrap())(
                self.as_raw(),
                gpu.as_raw(),
                frtc.as_mut_ptr(),
            )
        };
        Error::from_result_with_assume_init_on_success(result, frtc)
            .map(|frtc| unsafe { FrameRateTargetControl::from_raw(frtc) })
    }
}
