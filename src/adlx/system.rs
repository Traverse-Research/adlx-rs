use std::mem::MaybeUninit;

use crate::bindings::{self as ffi, IADLXInterface};

use super::{
    gpu_list::GpuList,
    interface::Interface,
    result::{Error, Result},
};

/// [`System`] is a singleton interface.  It looks similar to but is not compatible with
/// [`Interface`].
#[derive(Debug)]
#[doc(alias = "IADLXSystem")]
#[repr(transparent)]
pub struct System(*mut ffi::IADLXSystem);

impl System {
    /// Creates an `Interface` by taking ownership of the `raw` COM interface pointer.
    ///
    /// # Safety
    ///
    /// The `raw` pointer must be owned by the caller and represent a valid COM interface pointer. In other words,
    /// it must point to a vtable beginning with the `IUnknown` function pointers and match the vtable of `Interface`.
    pub(crate) unsafe fn from_raw(raw: *mut ffi::IADLXSystem) -> Self {
        Self(raw)
    }

    fn vtable(&self) -> &ffi::IADLXSystemVtbl {
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
        todo!("MARIIIIJN")
        // Error::from_result_with_assume_init_on_success(result, gpu_list)
        //     .map(|gpu_list| unsafe { GpuList::from_raw(gpu_list) })
    }
    #[doc(alias = "QueryInterface")]
    pub fn cast<I: Interface>(&self) -> Result<I> {
        let interface_name = I::IID
            // TODO: Use windows-rs' helpers to create static wchars?
            .encode_utf16()
            .chain(std::iter::once(0u16))
            .collect::<Vec<_>>();
        let mut interface = std::mem::MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().QueryInterface.unwrap())(
                self.0,
                interface_name.as_ptr(),
                interface.as_mut_ptr(),
            )
        };
        Error::from_result(result).map(|()| unsafe { I::from_raw(interface.assume_init()) })
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

#[derive(Clone, Debug)]
#[doc(alias = "IADLXSystem1")]
#[repr(transparent)]
pub struct System1(IADLXInterface);

unsafe impl Interface for System1 {
    type Vtable = ffi::IADLXSystem1Vtbl;
    const IID: &'static str = "IADLXSystem1";
}

impl System1 {
    // #[doc(alias = "GetPowerTuningServices")]
    // pub fn power_tuning_services(&self) -> Result<PowerTuningServices> {
    //     let mut ret = MaybeUninit::uninit();
    //     let result = self.vtable().GetPowerTuningServices(
    //         // TODO: Make it easier to get the actual self pointer type, while maintaining lifetiming from IADLXInterface
    //         self.0.cast(),
    //         ret.as_mut_ptr(),
    //     );
    //     let ret = Error::from_result_with_assume_init_on_success(result, ret)?;
    //     Ok(PowerTuningServices::from_raw(ret))
    // }
}
