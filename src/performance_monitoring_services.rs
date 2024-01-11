use std::mem::MaybeUninit;

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
}
