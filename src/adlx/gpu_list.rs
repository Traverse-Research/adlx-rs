use std::mem::MaybeUninit;

use crate::bindings::{self as ffi};

use super::{
    gpu::Gpu,
    interface::{Interface, ADLXInterface},
    result::{Error, Result},
};

#[derive(Clone, Debug)]
#[doc(alias = "IADLXGPUList")]
#[repr(transparent)]
pub struct GpuList(ADLXInterface);

unsafe impl Interface for GpuList {
    type Vtable = ffi::IADLXGPUListVtbl;

    const IID: &'static str = "IADLXGPUList";
}

impl GpuList {
    #[doc(alias = "Size")]
    pub fn size(&self) -> u32 {
        unsafe {
            (self.vtable().Size.unwrap())(
                // TODO: Make it easier to get the actual self pointer type, while maintaining lifetiming from IADLXInterface
                self.0.cast(),
            )
        }
    }

    #[doc(alias = "GpuAt")]
    pub fn gpu_at(&self, location: u32) -> Result<Gpu> {
        let mut gpu = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().At_GPUList.unwrap())(
                // TODO: Make it easier to get the actual self pointer type, while maintaining lifetiming from IADLXInterface
                self.0.cast(),
                location,
                gpu.as_mut_ptr(),
            )
        };
        Error::from_result_with_assume_init_on_success(result, gpu)
            .map(|gpu| unsafe { Gpu::from_raw(gpu) })
    }
}
