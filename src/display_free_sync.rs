use std::mem::MaybeUninit;

use super::{
    ffi,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
};

#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXDisplayFreeSync")]
pub struct DisplayFreeSync(InterfaceImpl);

unsafe impl Interface for DisplayFreeSync {
    type Impl = ffi::IADLXDisplayFreeSync;
    type Vtable = ffi::IADLXFreeSyncVtbl;
    const IID: &'static str = "IADLXDisplayFreeSync";
}

impl DisplayFreeSync {
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
}
