use std::mem::MaybeUninit;

use super::{
    ffi,
    gpu_list::GpuList,
    interface::{Interface, InterfaceImpl},
    performance_monitoring_services::PerformanceMonitoringServices,
    result::{Error, Result},
    DisplaysServices, ThreeDSettingsServices,
};

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_system/>
///
/// [`System`] is a singleton interface.  It looks similar to but is not compatible with
/// [`Interface`].
#[derive(Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXSystem")]
pub struct System(*mut ffi::IADLXSystem);

unsafe impl Send for System {}
unsafe impl Sync for System {}

impl System {
    /// Creates an [`Interface`] by taking ownership of the `raw` COM/ADLX interface pointer.
    ///
    /// # Safety
    ///
    /// The `raw` pointer must be owned by the caller and represent a valid [`ffi::IADLXSystem`]
    /// pointer. In other words, it must point to a vtable beginning with the
    /// [`ffi::IADLXSystemVtbl`] function pointers.
    pub(crate) unsafe fn from_raw(raw: *mut ffi::IADLXSystem) -> Self {
        Self(raw)
    }

    fn vtable(&self) -> &ffi::IADLXSystemVtbl {
        unsafe { &*(*self.0).pVtbl }
    }

    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_system__hybrid_graphics_type/>
    #[doc(alias = "GetHybridGraphicsType")]
    pub fn hybrid_graphics_type(&self) -> Result<ffi::ADLX_HG_TYPE> {
        let mut type_ = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().GetHybridGraphicsType.unwrap())(self.0, type_.as_mut_ptr()) };
        Error::from_result_with_assume_init_on_success(result, type_)
    }
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_system__get_g_p_us/>
    #[doc(alias = "GetGPUs")]
    pub fn gpus(&self) -> Result<GpuList> {
        let mut gpu_list = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().GetGPUs.unwrap())(self.0, gpu_list.as_mut_ptr()) };
        Error::from_result_with_assume_init_on_success(result, gpu_list)
            .map(|gpu_list| unsafe { GpuList::from_raw(gpu_list) })
    }
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_system__query_interface/>
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
        Error::from_result(result).map(|()| unsafe { I::from_raw(interface.assume_init().cast()) })
    }
    #[doc(alias = "GetDisplaysServices")]
    pub fn get_displays_services(&self) -> Result<DisplaysServices> {
        let mut displays_services = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GetDisplaysServices.unwrap())(self.0, displays_services.as_mut_ptr())
        };
        Error::from_result_with_assume_init_on_success(result, displays_services)
            .map(|displays_services| unsafe { DisplaysServices::from_raw(displays_services) })
    }
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
    #[doc(alias = "Get3DSettingsServices")]
    pub fn get_3d_settings_services(&self) -> Result<ThreeDSettingsServices> {
        let mut settings_service = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().Get3DSettingsServices.unwrap())(self.0, settings_service.as_mut_ptr())
        };
        Error::from_result_with_assume_init_on_success(result, settings_service)
            .map(|settings_service| unsafe { ThreeDSettingsServices::from_raw(settings_service) })
    }
    // #[doc(alias = "GetGPUTuningServices")]
    // pub fn GetGPUTuningServices(&self) -> Result<()> {
    //     let result = unsafe { (self.vtable().GetGPUTuningServices.unwrap())(self.0) };
    //     Error::from_result(result)?;

    //     Ok(())
    // }
    #[doc(alias = "GetPerformanceMonitoringServices")]
    pub fn performance_monitoring_services(&self) -> Result<PerformanceMonitoringServices> {
        let mut services = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GetPerformanceMonitoringServices.unwrap())(self.0, services.as_mut_ptr())
        };
        Error::from_result_with_assume_init_on_success(result, services)
            .map(|services| unsafe { PerformanceMonitoringServices::from_raw(services) })
    }
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

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_system1/>
#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXSystem1")]
pub struct System1(InterfaceImpl);

unsafe impl Interface for System1 {
    type Impl = ffi::IADLXSystem1;
    type Vtable = ffi::IADLXSystem1Vtbl;
    const IID: &'static str = "IADLXSystem1";
}

impl System1 {
    // /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_system1__get_power_tuning_services/>
    // #[doc(alias = "GetPowerTuningServices")]
    // pub fn power_tuning_services(&self) -> Result<PowerTuningServices> {
    //     let mut ret = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetPowerTuningServices.unwrap())(self.imp(), ret.as_mut_ptr())
    //     };
    //     let ret = Error::from_result_with_assume_init_on_success(result, ret)?;
    //     Ok(PowerTuningServices::from_raw(ret))
    // }
}
