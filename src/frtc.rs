use std::mem::MaybeUninit;

use super::{
    ffi,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
};

#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLX3DFrameRateTargetControl")]
pub struct FrameRateTargetControl(InterfaceImpl);

unsafe impl Interface for FrameRateTargetControl {
    type Impl = ffi::IADLX3DFrameRateTargetControl;
    type Vtable = ffi::IADLX3DFrameRateTargetControlVtbl;
    const IID: &'static str = "IADLX3DFrameRateTargetControl";
}

impl FrameRateTargetControl {
    pub fn is_supported(&self) -> Result<bool> {
        let mut supported = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().IsSupported.unwrap())(self.as_raw(), supported.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, supported).map(|s| s != 0)
    }

    pub fn is_enabled(&self) -> Result<bool> {
        let mut enabled = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().IsEnabled.unwrap())(self.as_raw(), enabled.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, enabled).map(|e| e != 0)
    }

    pub fn get_fps_range(&self) -> Result<ffi::ADLX_IntRange> {
        let mut fps_range = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().GetFPSRange.unwrap())(self.as_raw(), fps_range.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, fps_range).map(|fps_range| fps_range)
    }

    pub fn get_fps(&self) -> Result<ffi::adlx_int> {
        let mut fps = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().GetFPS.unwrap())(self.as_raw(), fps.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, fps).map(|fps| fps)
    }
}
