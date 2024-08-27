use std::mem::MaybeUninit;

use super::{
    ffi,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
};

#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLX3DSettingsServices")]
pub struct ThreeDWaitForVerticalRefresh(InterfaceImpl);

unsafe impl Interface for ThreeDWaitForVerticalRefresh {
    type Impl = ffi::IADLX3DWaitForVerticalRefresh;
    type Vtable = ffi::IADLX3DWaitForVerticalRefreshVtbl;
    const IID: &'static str = "IADLX3DWaitForVerticalRefresh";
}

impl ThreeDWaitForVerticalRefresh {
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

    pub fn get_mode(&self) -> Result<ffi::ADLX_WAIT_FOR_VERTICAL_REFRESH_MODE> {
        let mut mode = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().GetMode.unwrap())(self.as_raw(), mode.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, mode)
    }

    pub fn set_mode(&mut self, mode: ffi::ADLX_WAIT_FOR_VERTICAL_REFRESH_MODE) -> Result<()> {
        let result = unsafe { (self.vtable().SetMode.unwrap())(self.as_raw(), mode) };

        Error::from_result(result)
    }
}
