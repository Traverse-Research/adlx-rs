use std::mem::MaybeUninit;

use crate::bindings as ffi;

use super::{
    gpu::Gpu,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
};

// TODO: Don't derive Clone
#[derive(Debug)]
#[doc(alias = "IADLXGPUList")]
#[repr(transparent)]
pub struct GpuList(InterfaceImpl);

unsafe impl Interface for GpuList {
    type Impl = ffi::IADLXGPUList;
    type Vtable = ffi::IADLXGPUListVtbl;
    const IID: &'static str = "IADLXGPUList";
}

impl GpuList {
    #[doc(alias = "Size")]
    pub fn size(&self) -> u32 {
        unsafe { (self.vtable().Size.unwrap())(self.imp()) }
    }

    #[doc(alias = "GpuAt")]
    pub fn gpu_at(&self, location: u32) -> Result<Gpu> {
        let mut gpu = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().At_GPUList.unwrap())(self.imp(), location, gpu.as_mut_ptr()) };
        Error::from_result_with_assume_init_on_success(result, gpu)
            .map(|gpu| unsafe { Gpu::from_raw(gpu) })
    }
}
