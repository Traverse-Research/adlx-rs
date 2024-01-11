use std::mem::MaybeUninit;

use super::{
    ffi,
    gpu::Gpu,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
};

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_g_p_u_list/>
#[derive(Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXGPUList")]
// TODO(Marijn): This type inherits IADLXList; we should model inheritance!
pub struct GpuList(InterfaceImpl);

unsafe impl Interface for GpuList {
    type Impl = ffi::IADLXGPUList;
    type Vtable = ffi::IADLXGPUListVtbl;
    const IID: &'static str = "IADLXGPUList";
}

impl GpuList {
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_list__size/#doxid-d-o-x-i-a-d-l-x-list-size>
    #[doc(alias = "Size")]
    pub fn size(&self) -> u32 {
        unsafe { (self.vtable().Size.unwrap())(self.imp()) }
    }

    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_g_p_u_list__at/>
    #[doc(alias = "At_GPUList")]
    pub fn gpu_at(&self, location: u32) -> Result<Gpu> {
        let mut gpu = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().At_GPUList.unwrap())(self.imp(), location, gpu.as_mut_ptr()) };
        Error::from_result_with_assume_init_on_success(result, gpu)
            .map(|gpu| unsafe { Gpu::from_raw(gpu) })
    }
}
