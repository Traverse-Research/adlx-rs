use std::mem::MaybeUninit;

use crate::gpu_metrics::GpuMetricsSupport;

use super::{
    ffi,
    gpu::Gpu,
    gpu_metrics::GpuMetrics,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
};

#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXPerformanceMonitoringServices")]
pub struct PerformanceMonitoringServices(InterfaceImpl);

unsafe impl Interface for PerformanceMonitoringServices {
    type Impl = ffi::IADLXPerformanceMonitoringServices;
    type Vtable = ffi::IADLXPerformanceMonitoringServicesVtbl;
    const IID: &'static str = "IADLXPerformanceMonitoringServices";
}

impl PerformanceMonitoringServices {
    #[doc(alias = "GetCurrentGPUMetrics")]
    pub fn current_gpu_metrics(&self, gpu: &Gpu) -> Result<GpuMetrics> {
        let mut metrics = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GetCurrentGPUMetrics.unwrap())(
                self.as_raw(),
                gpu.as_raw(),
                metrics.as_mut_ptr(),
            )
        };
        Error::from_result_with_assume_init_on_success(result, metrics)
            .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    }

    #[doc(alias = "GetSamplingIntervalRange")]
    // pub fn GetSamplingIntervalRange(&self, ADLX_IntRange* range) -> Result<!> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "SetSamplingInterval")]
    // pub fn SetSamplingInterval(&mut self, interval_in_ms: i32) -> Result<()> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "GetSamplingInterval")]
    // pub fn GetSamplingInterval(&self) -> Result<i32> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "GetMaxPerformanceMetricsHistorySizeRange")]
    // pub fn GetMaxPerformanceMetricsHistorySizeRange(&self, ADLX_IntRange* range) -> Result<!> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "SetMaxPerformanceMetricsHistorySize")]
    // pub fn SetMaxPerformanceMetricsHistorySize(&mut self, size_in_sec: i32) -> Result<()> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "GetMaxPerformanceMetricsHistorySize")]
    // pub fn GetMaxPerformanceMetricsHistorySize(&self) -> Result<i32> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "ClearPerformanceMetricsHistory")]
    // pub fn ClearPerformanceMetricsHistory(&self) -> Result<()> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "GetCurrentPerformanceMetricsHistorySize")]
    // pub fn GetCurrentPerformanceMetricsHistorySize(&self) -> Result<i32> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "StartPerformanceMetricsTracking")]
    // pub fn StartPerformanceMetricsTracking(&mut self) -> Result<()> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "StopPerformanceMetricsTracking")]
    // pub fn StopPerformanceMetricsTracking(&mut self) -> Result<()> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "GetAllMetricsHistory")]
    // pub fn GetAllMetricsHistory(&self, start_in_ms: i32, stop_in_ms: i32, IADLXAllMetricsList** ppMetricsList) -> Result<!> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "GetGPUMetricsHistory")]
    // pub fn GetGPUMetricsHistory(&self, gpu: &Gpu, start_in_ms: i32, stop_in_ms: i32, IADLXGPUMetricsList** ppMetricsList) -> Result<!> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "GetSystemMetricsHistory")]
    // pub fn GetSystemMetricsHistory(&self, start_in_ms: i32, stop_in_ms: i32, IADLXSystemMetricsList** ppMetricsList) -> Result<!> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "GetFPSHistory")]
    // pub fn GetFPSHistory(&self, start_in_ms: i32, stop_in_ms: i32, IADLXFPSList** ppMetricsList) -> Result<!> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "GetCurrentAllMetrics")]
    // pub fn GetCurrentAllMetrics(&self, IADLXAllMetrics** ppMetrics) -> Result<!> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "GetCurrentSystemMetrics")]
    // pub fn GetCurrentSystemMetrics(&self, IADLXSystemMetrics** ppMetrics) -> Result<!> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "GetCurrentFPS")]
    // pub fn GetCurrentFPS(&self, IADLXFPS** ppMetrics) -> Result<!> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
    // #[doc(alias = "GetSupportedGPUMetrics")]
    pub fn supported_gpu_metrics(&self, gpu: &Gpu) -> Result<GpuMetricsSupport> {
        let mut support = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GetSupportedGPUMetrics.unwrap())(
                self.as_raw(),
                gpu.as_raw(),
                support.as_mut_ptr(),
            )
        };
        Error::from_result_with_assume_init_on_success(result, support)
            .map(|support| unsafe { GpuMetricsSupport::from_raw(support) })
    }
    // #[doc(alias = "GetSupportedSystemMetrics")]
    // pub fn GetSupportedSystemMetrics(&self) -> Result<!> {
    //     let mut metrics = MaybeUninit::uninit();
    //     let result = unsafe {
    //         (self.vtable().GetCurrentGPUMetrics.unwrap())(
    //             self.as_raw(),
    //             gpu.as_raw(),
    //             metrics.as_mut_ptr(),
    //         )
    //     };
    //     Error::from_result_with_assume_init_on_success(result, metrics)
    //         .map(|metrics| unsafe { GpuMetrics::from_raw(metrics) })
    // }
}
