use std::mem::MaybeUninit;

use super::{
    ffi,
    gpu::Gpu,
    gpu_metrics::{GpuMetrics, GpuMetricsList, GpuMetricsSupport},
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
    pub fn sampling_interval_range(&self) -> Result<ffi::ADLX_IntRange> {
        let mut range = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GetSamplingIntervalRange.unwrap())(self.as_raw(), range.as_mut_ptr())
        };
        Error::from_result_with_assume_init_on_success(result, range)
    }

    #[doc(alias = "SetSamplingInterval")]
    /// `interval_in_ms` must be in the range provided by [`Self::sampling_interval_range()`].
    pub fn set_sampling_interval(&mut self, interval_in_ms: i32) -> Result<()> {
        let result =
            unsafe { (self.vtable().SetSamplingInterval.unwrap())(self.as_raw(), interval_in_ms) };
        Error::from_result(result)
    }

    #[doc(alias = "GetSamplingInterval")]
    pub fn sampling_interval(&self) -> Result<i32> {
        let mut interval = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GetSamplingInterval.unwrap())(self.as_raw(), interval.as_mut_ptr())
        };
        Error::from_result_with_assume_init_on_success(result, interval)
    }

    #[doc(alias = "GetMaxPerformanceMetricsHistorySizeRange")]
    pub fn max_performance_metrics_history_range(&self) -> Result<ffi::ADLX_IntRange> {
        let mut range = MaybeUninit::uninit();
        let result = unsafe {
            (self
                .vtable()
                .GetMaxPerformanceMetricsHistorySizeRange
                .unwrap())(self.as_raw(), range.as_mut_ptr())
        };
        Error::from_result_with_assume_init_on_success(result, range)
    }

    #[doc(alias = "SetMaxPerformanceMetricsHistorySize")]
    /// `size_in_sec` must be in the range provided by [`Self::max_performance_metrics_history_range()`].
    pub fn set_max_performance_metrics_history_size(&mut self, size_in_sec: i32) -> Result<()> {
        let result = unsafe {
            (self.vtable().SetMaxPerformanceMetricsHistorySize.unwrap())(self.as_raw(), size_in_sec)
        };
        Error::from_result(result)
    }

    #[doc(alias = "GetMaxPerformanceMetricsHistorySize")]
    pub fn max_performance_metrics_history_size(&self) -> Result<i32> {
        let mut size = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GetMaxPerformanceMetricsHistorySize.unwrap())(
                self.as_raw(),
                size.as_mut_ptr(),
            )
        };
        Error::from_result_with_assume_init_on_success(result, size)
    }

    #[doc(alias = "ClearPerformanceMetricsHistory")]
    pub fn clear_performance_metrics_history(&self) -> Result<()> {
        let result =
            unsafe { (self.vtable().ClearPerformanceMetricsHistory.unwrap())(self.as_raw()) };
        Error::from_result(result)
    }

    #[doc(alias = "GetCurrentPerformanceMetricsHistorySize")]
    pub fn current_performance_metrics_history_size_in_sec(&self) -> Result<i32> {
        let mut size = MaybeUninit::uninit();
        let result = unsafe {
            (self
                .vtable()
                .GetCurrentPerformanceMetricsHistorySize
                .unwrap())(self.as_raw(), size.as_mut_ptr())
        };
        Error::from_result_with_assume_init_on_success(result, size)
    }

    #[doc(alias = "StartPerformanceMetricsTracking")]
    pub fn start_performance_tracking(&mut self) -> Result<()> {
        let result =
            unsafe { (self.vtable().StartPerformanceMetricsTracking.unwrap())(self.as_raw()) };
        Error::from_result(result)
    }

    #[doc(alias = "StopPerformanceMetricsTracking")]
    pub fn stop_performance_tracking(&mut self) -> Result<()> {
        let result =
            unsafe { (self.vtable().StopPerformanceMetricsTracking.unwrap())(self.as_raw()) };
        Error::from_result(result)
    }

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

    #[doc(alias = "GetGPUMetricsHistory")]
    pub fn gpu_metrics_history(
        &self,
        gpu: &Gpu,
        start_in_ms: i32,
        stop_in_ms: i32,
    ) -> Result<GpuMetricsList> {
        let mut metrics_list = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GetGPUMetricsHistory.unwrap())(
                self.as_raw(),
                gpu.as_raw(),
                start_in_ms,
                stop_in_ms,
                metrics_list.as_mut_ptr(),
            )
        };
        Error::from_result_with_assume_init_on_success(result, metrics_list)
            .map(|metrics_list| unsafe { GpuMetricsList::from_raw(metrics_list) })
    }

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
