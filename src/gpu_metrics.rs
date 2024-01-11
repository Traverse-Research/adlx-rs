use std::mem::MaybeUninit;

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
