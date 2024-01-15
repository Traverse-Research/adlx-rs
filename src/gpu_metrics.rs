use std::mem::MaybeUninit;

use crate::list::List;

use super::{
    ffi,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
};

#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXGPUMetrics")]
pub struct GpuMetrics(InterfaceImpl);

unsafe impl Interface for GpuMetrics {
    type Impl = ffi::IADLXGPUMetrics;
    type Vtable = ffi::IADLXGPUMetricsVtbl;
    const IID: &'static str = "IADLXGPUMetrics";
}

impl GpuMetrics {
    #[doc(alias = "TimeStamp")]
    pub fn time_stamp(&self) -> Result<i64> {
        let mut time_stamp = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().TimeStamp.unwrap())(self.as_raw(), time_stamp.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, time_stamp)
    }

    #[doc(alias = "GPUUsage")]
    pub fn usage(&self) -> Result<f64> {
        let mut usage = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().GPUUsage.unwrap())(self.as_raw(), usage.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, usage)
    }

    #[doc(alias = "GPUClockSpeed")]
    pub fn clock_speed(&self) -> Result<i32> {
        let mut clock_speed = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GPUClockSpeed.unwrap())(self.as_raw(), clock_speed.as_mut_ptr())
        };

        Error::from_result_with_assume_init_on_success(result, clock_speed)
    }

    #[doc(alias = "GPUVRAMClockSpeed")]
    pub fn vram_clock_speed(&self) -> Result<i32> {
        let mut vram_clock_speed = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GPUVRAMClockSpeed.unwrap())(self.as_raw(), vram_clock_speed.as_mut_ptr())
        };

        Error::from_result_with_assume_init_on_success(result, vram_clock_speed)
    }

    #[doc(alias = "GPUPower")]
    pub fn power(&self) -> Result<f64> {
        let mut power = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().GPUPower.unwrap())(self.as_raw(), power.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, power)
    }

    #[doc(alias = "GPUTotalBoardPower")]
    pub fn total_board_power(&self) -> Result<f64> {
        let mut total_board_power = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GPUTotalBoardPower.unwrap())(
                self.as_raw(),
                total_board_power.as_mut_ptr(),
            )
        };

        Error::from_result_with_assume_init_on_success(result, total_board_power)
    }

    #[doc(alias = "GPUVoltage")]
    pub fn voltage(&self) -> Result<i32> {
        let mut voltage = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().GPUVoltage.unwrap())(self.as_raw(), voltage.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, voltage)
    }

    #[doc(alias = "GPUVRAM")]
    pub fn vram(&self) -> Result<i32> {
        let mut vram = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().GPUVRAM.unwrap())(self.as_raw(), vram.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, vram)
    }

    #[doc(alias = "GPUFanSpeed")]
    pub fn fan_speed(&self) -> Result<i32> {
        let mut fan_speed = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().GPUFanSpeed.unwrap())(self.as_raw(), fan_speed.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, fan_speed)
    }

    #[doc(alias = "GPUTemperature")]
    pub fn temperature(&self) -> Result<f64> {
        let mut temperature = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GPUTemperature.unwrap())(self.as_raw(), temperature.as_mut_ptr())
        };

        Error::from_result_with_assume_init_on_success(result, temperature)
    }

    #[doc(alias = "GPUIntakeTemperature")]
    pub fn intake_temperature(&self) -> Result<f64> {
        let mut intake_temperature = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GPUIntakeTemperature.unwrap())(
                self.as_raw(),
                intake_temperature.as_mut_ptr(),
            )
        };

        Error::from_result_with_assume_init_on_success(result, intake_temperature)
    }

    #[doc(alias = "GPUHotspotTemperature")]
    pub fn hotspot_temperature(&self) -> Result<f64> {
        let mut hotspot_temperature = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GPUHotspotTemperature.unwrap())(
                self.as_raw(),
                hotspot_temperature.as_mut_ptr(),
            )
        };

        Error::from_result_with_assume_init_on_success(result, hotspot_temperature)
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXGPUMetricsList")]
pub struct GpuMetricsList(List);

unsafe impl Interface for GpuMetricsList {
    type Impl = ffi::IADLXGPUMetricsList;
    type Vtable = ffi::IADLXGPUMetricsListVtbl;
    const IID: &'static str = "IADLXGpuMetricsList";
}

impl GpuMetricsList {
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_g_p_u_list__at/>
    #[doc(alias = "At_GPUMetricsList")]
    pub fn at(&self, location: u32) -> Result<GpuMetrics> {
        let mut metrics = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().At_GPUMetricsList.unwrap())(
                self.as_raw(),
                location,
                metrics.as_mut_ptr(),
            )
        };
        Error::from_result_with_assume_init_on_success(result, metrics)
            .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    }
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_g_p_u_list__add__back/#doxid-d-o-x-i-a-d-l-x-g-p-u-list-add-back>
    #[doc(alias = "Add_Back_GPUMetricsList")]
    pub fn add_back(&self, gpu_metrics: GpuMetrics) -> Result<()> {
        let result = unsafe {
            // TODO: Assume ownership is consumed here?
            (self.vtable().Add_Back_GPUMetricsList.unwrap())(self.as_raw(), gpu_metrics.into_raw())
        };
        Error::from_result(result)
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXGPUMetricsSupport")]
pub struct GpuMetricsSupport(InterfaceImpl);

unsafe impl Interface for GpuMetricsSupport {
    type Impl = ffi::IADLXGPUMetricsSupport;
    type Vtable = ffi::IADLXGPUMetricsSupportVtbl;
    const IID: &'static str = "IADLXGpuMetricsSupport";
}

impl GpuMetricsSupport {
    #[doc(alias = "IsSupportedGPUUsage")]
    pub fn is_supported_gpu_usage(&self) -> Result<bool> {
        unsafe {
            let mut supported = MaybeUninit::uninit();
            let res =
                (self.vtable().IsSupportedGPUUsage.unwrap())(self.as_raw(), supported.as_mut_ptr());
            Error::from_result_with_assume_init_on_success(res, supported)
                .map(|supported| supported != 0)
        }
    }
    #[doc(alias = "IsSupportedGPUClockSpeed")]
    pub fn is_supported_gpu_clock_speed(&self) -> Result<bool> {
        unsafe {
            let mut supported = MaybeUninit::uninit();
            let res = (self.vtable().IsSupportedGPUClockSpeed.unwrap())(
                self.as_raw(),
                supported.as_mut_ptr(),
            );
            Error::from_result_with_assume_init_on_success(res, supported)
                .map(|supported| supported != 0)
        }
    }
    #[doc(alias = "IsSupportedGPUVRAMClockSpeed")]
    pub fn is_supported_gpu_vram_clock_speed(&self) -> Result<bool> {
        unsafe {
            let mut supported = MaybeUninit::uninit();
            let res = (self.vtable().IsSupportedGPUVRAMClockSpeed.unwrap())(
                self.as_raw(),
                supported.as_mut_ptr(),
            );
            Error::from_result_with_assume_init_on_success(res, supported)
                .map(|supported| supported != 0)
        }
    }
    #[doc(alias = "IsSupportedGPUTemperature")]
    pub fn is_supported_gpu_temperature(&self) -> Result<bool> {
        unsafe {
            let mut supported = MaybeUninit::uninit();
            let res = (self.vtable().IsSupportedGPUTemperature.unwrap())(
                self.as_raw(),
                supported.as_mut_ptr(),
            );
            Error::from_result_with_assume_init_on_success(res, supported)
                .map(|supported| supported != 0)
        }
    }
    #[doc(alias = "IsSupportedGPUHotspotTemperature")]
    pub fn is_supported_gpu_hotspot_temperature(&self) -> Result<bool> {
        unsafe {
            let mut supported = MaybeUninit::uninit();
            let res = (self.vtable().IsSupportedGPUHotspotTemperature.unwrap())(
                self.as_raw(),
                supported.as_mut_ptr(),
            );
            Error::from_result_with_assume_init_on_success(res, supported)
                .map(|supported| supported != 0)
        }
    }
    #[doc(alias = "IsSupportedGPUPower")]
    pub fn is_supported_gpu_power(&self) -> Result<bool> {
        unsafe {
            let mut supported = MaybeUninit::uninit();
            let res =
                (self.vtable().IsSupportedGPUPower.unwrap())(self.as_raw(), supported.as_mut_ptr());
            Error::from_result_with_assume_init_on_success(res, supported)
                .map(|supported| supported != 0)
        }
    }
    #[doc(alias = "IsSupportedGPUTotalBoardPower")]
    pub fn is_supported_gpu_total_board_power(&self) -> Result<bool> {
        unsafe {
            let mut supported = MaybeUninit::uninit();
            let res = (self.vtable().IsSupportedGPUTotalBoardPower.unwrap())(
                self.as_raw(),
                supported.as_mut_ptr(),
            );
            Error::from_result_with_assume_init_on_success(res, supported)
                .map(|supported| supported != 0)
        }
    }
    #[doc(alias = "IsSupportedGPUFanSpeed")]
    pub fn is_supported_gpu_fan_speed(&self) -> Result<bool> {
        unsafe {
            let mut supported = MaybeUninit::uninit();
            let res = (self.vtable().IsSupportedGPUFanSpeed.unwrap())(
                self.as_raw(),
                supported.as_mut_ptr(),
            );
            Error::from_result_with_assume_init_on_success(res, supported)
                .map(|supported| supported != 0)
        }
    }
    #[doc(alias = "IsSupportedGPUVRAM")]
    pub fn is_supported_gpu_vram(&self) -> Result<bool> {
        unsafe {
            let mut supported = MaybeUninit::uninit();
            let res =
                (self.vtable().IsSupportedGPUVRAM.unwrap())(self.as_raw(), supported.as_mut_ptr());
            Error::from_result_with_assume_init_on_success(res, supported)
                .map(|supported| supported != 0)
        }
    }
    #[doc(alias = "IsSupportedGPUVoltage")]
    pub fn is_supported_gpu_voltage(&self) -> Result<bool> {
        unsafe {
            let mut supported = MaybeUninit::uninit();
            let res = (self.vtable().IsSupportedGPUVoltage.unwrap())(
                self.as_raw(),
                supported.as_mut_ptr(),
            );
            Error::from_result_with_assume_init_on_success(res, supported)
                .map(|supported| supported != 0)
        }
    }
    #[doc(alias = "IsSupportedGPUIntakeTemperature")]
    pub fn is_supported_gpu_intake_temperature(&self) -> Result<bool> {
        unsafe {
            let mut supported = MaybeUninit::uninit();
            let res = (self.vtable().IsSupportedGPUIntakeTemperature.unwrap())(
                self.as_raw(),
                supported.as_mut_ptr(),
            );
            Error::from_result_with_assume_init_on_success(res, supported)
                .map(|supported| supported != 0)
        }
    }

    #[doc(alias = "GetGPUUsageRange")]
    pub fn gpu_usage_range(&self) -> Result<std::ops::Range<i32>> {
        unsafe {
            let mut min = MaybeUninit::uninit();
            let mut max = MaybeUninit::uninit();
            let res = (self.vtable().GetGPUUsageRange.unwrap())(
                self.as_raw(),
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            Error::from_result(res).map(|()| {
                let min = min.assume_init();
                let max = max.assume_init();
                min..max
            })
        }
    }
    #[doc(alias = "GetGPUClockSpeedRange")]
    pub fn gpu_clock_speed_range(&self) -> Result<std::ops::Range<i32>> {
        unsafe {
            let mut min = MaybeUninit::uninit();
            let mut max = MaybeUninit::uninit();
            let res = (self.vtable().GetGPUClockSpeedRange.unwrap())(
                self.as_raw(),
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            Error::from_result(res).map(|()| {
                let min = min.assume_init();
                let max = max.assume_init();
                min..max
            })
        }
    }
    #[doc(alias = "GetGPUVRAMClockSpeedRange")]
    pub fn gpu_vram_clock_speed_range(&self) -> Result<std::ops::Range<i32>> {
        unsafe {
            let mut min = MaybeUninit::uninit();
            let mut max = MaybeUninit::uninit();
            let res = (self.vtable().GetGPUVRAMClockSpeedRange.unwrap())(
                self.as_raw(),
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            Error::from_result(res).map(|()| {
                let min = min.assume_init();
                let max = max.assume_init();
                min..max
            })
        }
    }
    #[doc(alias = "GetGPUTemperatureRange")]
    pub fn gpu_temperature_range(&self) -> Result<std::ops::Range<i32>> {
        unsafe {
            let mut min = MaybeUninit::uninit();
            let mut max = MaybeUninit::uninit();
            let res = (self.vtable().GetGPUTemperatureRange.unwrap())(
                self.as_raw(),
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            Error::from_result(res).map(|()| {
                let min = min.assume_init();
                let max = max.assume_init();
                min..max
            })
        }
    }
    #[doc(alias = "GetGPUHotspotTemperatureRange")]
    pub fn gpu_hotspot_temperature_range(&self) -> Result<std::ops::Range<i32>> {
        unsafe {
            let mut min = MaybeUninit::uninit();
            let mut max = MaybeUninit::uninit();
            let res = (self.vtable().GetGPUHotspotTemperatureRange.unwrap())(
                self.as_raw(),
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            Error::from_result(res).map(|()| {
                let min = min.assume_init();
                let max = max.assume_init();
                min..max
            })
        }
    }
    #[doc(alias = "GetGPUPowerRange")]
    pub fn gpu_power_range(&self) -> Result<std::ops::Range<i32>> {
        unsafe {
            let mut min = MaybeUninit::uninit();
            let mut max = MaybeUninit::uninit();
            let res = (self.vtable().GetGPUPowerRange.unwrap())(
                self.as_raw(),
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            Error::from_result(res).map(|()| {
                let min = min.assume_init();
                let max = max.assume_init();
                min..max
            })
        }
    }
    #[doc(alias = "GetGPUFanSpeedRange")]
    pub fn gpu_fan_speed_range(&self) -> Result<std::ops::Range<i32>> {
        unsafe {
            let mut min = MaybeUninit::uninit();
            let mut max = MaybeUninit::uninit();
            let res = (self.vtable().GetGPUFanSpeedRange.unwrap())(
                self.as_raw(),
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            Error::from_result(res).map(|()| {
                let min = min.assume_init();
                let max = max.assume_init();
                min..max
            })
        }
    }
    #[doc(alias = "GetGPUVRAMRange")]
    pub fn gpu_vran_range(&self) -> Result<std::ops::Range<i32>> {
        unsafe {
            let mut min = MaybeUninit::uninit();
            let mut max = MaybeUninit::uninit();
            let res = (self.vtable().GetGPUVRAMRange.unwrap())(
                self.as_raw(),
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            Error::from_result(res).map(|()| {
                let min = min.assume_init();
                let max = max.assume_init();
                min..max
            })
        }
    }
    #[doc(alias = "GetGPUVoltageRange")]
    pub fn gpu_voltage_range(&self) -> Result<std::ops::Range<i32>> {
        unsafe {
            let mut min = MaybeUninit::uninit();
            let mut max = MaybeUninit::uninit();
            let res = (self.vtable().GetGPUVoltageRange.unwrap())(
                self.as_raw(),
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            Error::from_result(res).map(|()| {
                let min = min.assume_init();
                let max = max.assume_init();
                min..max
            })
        }
    }
    #[doc(alias = "GetGPUTotalBoardPowerRange")]
    pub fn gpu_total_board_power_range(&self) -> Result<std::ops::Range<i32>> {
        unsafe {
            let mut min = MaybeUninit::uninit();
            let mut max = MaybeUninit::uninit();
            let res = (self.vtable().GetGPUTotalBoardPowerRange.unwrap())(
                self.as_raw(),
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            Error::from_result(res).map(|()| {
                let min = min.assume_init();
                let max = max.assume_init();
                min..max
            })
        }
    }
    #[doc(alias = "GetGPUIntakeTemperatureRange")]
    pub fn gpu_intake_temperature_range(&self) -> Result<std::ops::Range<i32>> {
        unsafe {
            let mut min = MaybeUninit::uninit();
            let mut max = MaybeUninit::uninit();
            let res = (self.vtable().GetGPUIntakeTemperatureRange.unwrap())(
                self.as_raw(),
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            Error::from_result(res).map(|()| {
                let min = min.assume_init();
                let max = max.assume_init();
                min..max
            })
        }
    }
}
