use std::mem::MaybeUninit;

use super::{
    ffi,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
    Display, DisplayFreeSync, DisplayList,
};

#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXDisplayServices")]
pub struct DisplaysServices(InterfaceImpl);

unsafe impl Interface for DisplaysServices {
    type Impl = ffi::IADLXDisplayServices;
    type Vtable = ffi::IADLXDisplayServicesVtbl;
    const IID: &'static str = "IADLXDisplayServices";
}

impl DisplaysServices {
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

    pub fn get_displays(&self) -> Result<DisplayList> {
        let mut displays = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().GetDisplays.unwrap())(self.as_raw(), displays.as_mut_ptr()) };
        Error::from_result_with_assume_init_on_success(result, displays)
            .map(|displays| unsafe { DisplayList::from_raw(displays) })
    }

    //     pub GetFreeSync: ::std::option::Option<
    //     unsafe extern "C" fn(
    //         pThis: *mut IADLXDisplayServices,
    //         pDisplay: *mut IADLXDisplay,
    //         ppFreeSync: *mut *mut IADLXDisplayFreeSync,
    //     ) -> ADLX_RESULT,
    // >
}
