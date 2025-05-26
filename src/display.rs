use std::mem::MaybeUninit;

use super::{
    ffi,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
    Gpu,
};

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_display/>
#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXDisplay")]
pub struct Display(InterfaceImpl);

unsafe impl Interface for Display {
    type Impl = ffi::IADLXDisplay;
    type Vtable = ffi::IADLXDisplayVtbl;
    const IID: &'static str = "IADLXDisplay";
}

impl Display {
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_display__get_g_p_u/>
    #[doc(alias = "GetGPU")]
    pub fn get_gpu(&self) -> Result<Gpu> {
        let mut gpu = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().GetGPU.unwrap())(self.as_raw(), gpu.as_mut_ptr()) };
        Error::from_result_with_assume_init_on_success(result, gpu)
            .map(|gpu| unsafe { Gpu::from_raw(gpu) })
    }
}
