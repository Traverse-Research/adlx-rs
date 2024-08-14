use std::mem::MaybeUninit;

use super::{
    ffi,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
    Gpu,
};

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
    pub fn get_gpu(&self) -> Result<Gpu> {
        let mut gpu = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().GetGPU.unwrap())(self.as_raw(), gpu.as_mut_ptr()) };
        Error::from_result_with_assume_init_on_success(result, gpu)
            .map(|gpu| unsafe { Gpu::from_raw(gpu) })
    }
}
