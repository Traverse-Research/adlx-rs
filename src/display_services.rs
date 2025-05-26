use std::mem::MaybeUninit;

use super::{
    ffi,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
    Display, DisplayFreeSync, DisplayList,
};

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_display_services/>
#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXDisplayServices")]
pub struct DisplayServices(InterfaceImpl);

unsafe impl Interface for DisplayServices {
    type Impl = ffi::IADLXDisplayServices;
    type Vtable = ffi::IADLXDisplayServicesVtbl;
    const IID: &'static str = "IADLXDisplayServices";
}

impl DisplayServices {
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_display_services__get_free_sync/>
    #[doc(alias = "GetFreeSync")]
    pub fn get_free_sync(&self, display: &Display) -> Result<DisplayFreeSync> {
        let mut free_sync = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GetFreeSync.unwrap())(
                self.as_raw(),
                display.as_raw(),
                free_sync.as_mut_ptr(),
            )
        };
        Error::from_result_with_assume_init_on_success(result, free_sync)
            .map(|free_sync| unsafe { DisplayFreeSync::from_raw(free_sync) })
    }

    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_display_services__get_displays/>
    #[doc(alias = "GetDisplays")]
    pub fn get_displays(&self) -> Result<DisplayList> {
        let mut displays = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().GetDisplays.unwrap())(self.as_raw(), displays.as_mut_ptr()) };
        Error::from_result_with_assume_init_on_success(result, displays)
            .map(|displays| unsafe { DisplayList::from_raw(displays) })
    }
}
