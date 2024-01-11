use std::mem::MaybeUninit;

use crate::bindings as ffi;

use super::{
    gpu_list::GpuList,
    interface::Interface,
    result::{Error, Result},
};

// TODO: Don't derive Clone
#[derive(Debug)]
#[doc(alias = "IADLXSystem")]
#[repr(transparent)]
// WARNING: IADLXSystem is not a refcounted IADLXInterface-like object!
pub struct PerformanceMonitoringServices(*mut ffi::IADLXPerformanceMonitoringServices);

// impl Interface for System {
//     type Vtable = ffi::IADLXSystemVtbl;
// }

impl PerformanceMonitoringServices {
    /// Creates an `Interface` by taking ownership of the `raw` COM interface pointer.
    ///
    /// # Safety
    ///
    /// The `raw` pointer must be owned by the caller and represent a valid COM interface pointer. In other words,
    /// it must point to a vtable beginning with the `IUnknown` function pointers and match the vtable of `Interface`.
    pub(crate) unsafe fn from_raw(raw: *mut ffi::IADLXPerformanceMonitoringServices) -> Self {
        Self(raw)
    }

    fn vtable(&self) -> &ffi::IADLXPerformanceMonitoringServicesVtbl {
        unsafe { &*(*self.0).pVtbl }
    }

    #[doc(alias = "GetHybridGraphicsType")]
    pub fn hybrid_graphics_type(&self) -> Result<ffi::ADLX_HG_TYPE> {
        let mut type_ = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().GetHybridGraphicsType.unwrap())(self.0, type_.as_mut_ptr()) };
        Error::from_result_with_assume_init_on_success(result, type_)
    }
    #[doc(alias = "GetGPUs")]
    pub fn get_gpus(&self) -> Result<GpuList> {
        let mut gpu_list = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().GetGPUs.unwrap())(self.0, gpu_list.as_mut_ptr()) };
        Error::from_result_with_assume_init_on_success(result, gpu_list)
            .map(|gpu_list| unsafe { GpuList::from_raw(gpu_list) })
    }
    #[doc(alias = "QueryInterface")]
    // TODO: Fish IID from Interface trait!
    pub fn cast<I: Interface>(&self, interface: &str) -> Result<I> {
        let interface_wide = interface
            .encode_utf16()
            .chain(std::iter::once(0u16))
            .collect::<Vec<_>>();
        let mut result__ = std::mem::MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().QueryInterface.unwrap())(
                self.0,
                interface_wide.as_ptr(),
                result__.as_mut_ptr(),
            )
        };
        Error::from_result(result).map(|()| unsafe { I::from_raw(result__.assume_init()) })
    }
    // #[doc(alias = "GetDisplaysServices")]
    // pub fn GetDisplaysServices(&self) -> Result<()> {
    //     let result = unsafe { (self.vtable().GetDisplaysServices.unwrap())(self.0) };
    //     Error::from_result(result)?;

    //     Ok(())
    // }
    // #[doc(alias = "GetDesktopsServices")]
    // pub fn GetDesktopsServices(&self) -> Result<()> {
    //     let result = unsafe { (self.vtable().GetDesktopsServices.unwrap())(self.0) };
    //     Error::from_result(result)?;

    //     Ok(())
    // }
    // #[doc(alias = "GetGPUsChangedHandling")]
    // pub fn GetGPUsChangedHandling(&self) -> Result<()> {
    //     let result = unsafe { (self.vtable().GetGPUsChangedHandling.unwrap())(self.0) };
    //     Error::from_result(result)?;

    //     Ok(())
    // }
    // #[doc(alias = "EnableLog")]
    // pub fn EnableLog(&self) -> Result<()> {
    //     let result = unsafe { (self.vtable().EnableLog.unwrap())(self.0) };
    //     Error::from_result(result)?;

    //     Ok(())
    // }
    // #[doc(alias = "Get3DSettingsServices")]
    // pub fn Get3DSettingsServices(&self) -> Result<()> {
    //     let result = unsafe { (self.vtable().Get3DSettingsServices.unwrap())(self.0) };
    //     Error::from_result(result)?;

    //     Ok(())
    // }
    // #[doc(alias = "GetGPUTuningServices")]
    // pub fn GetGPUTuningServices(&self) -> Result<()> {
    //     let result = unsafe { (self.vtable().GetGPUTuningServices.unwrap())(self.0) };
    //     Error::from_result(result)?;

    //     Ok(())
    // }
    // #[doc(alias = "GetPerformanceMonitoringServices")]
    // pub fn GetPerformanceMonitoringServices(&self) -> Result<()> {
    //     let result = unsafe { (self.vtable().GetPerformanceMonitoringServices.unwrap())(self.0) };
    //     Error::from_result(result)?;

    //     Ok(())
    // }
    // #[doc(alias = "TotalSystemRAM")]
    // pub fn TotalSystemRAM(&self) -> Result<()> {
    //     let result = unsafe { (self.vtable().TotalSystemRAM.unwrap())(self.0) };
    //     Error::from_result(result)?;

    //     Ok(())
    // }
    // #[doc(alias = "GetI2C")]
    // pub fn GetI2C(&self) -> Result<()> {
    //     let result = unsafe { (self.vtable().GetI2C.unwrap())(self.0) };
    //     Error::from_result(result)?;

    //     Ok(())
    // }
}
