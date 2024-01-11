use std::{ffi::CStr, mem::MaybeUninit};

use super::{
    interface::{ADLXInterface, Interface},
    result::{Error, Result},
};
use crate::bindings as ffi;

// TODO: Don't derive Clone
#[derive(Clone, Debug)]
#[doc(alias = "IADLXGPU")]
#[repr(transparent)]
pub struct Gpu(ADLXInterface);

unsafe impl Interface for Gpu {
    type Vtable = ffi::IADLXGPUVtbl;
    const IID: &'static str = "IADLXGPU";
}

impl Gpu {
    #[doc(alias = "Name")]
    pub fn name(&self) -> Result<String> {
        let mut name = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().Name.unwrap())(self.0, name.as_mut_ptr()) };
        Error::from_result_with_assume_init_on_success(result, name)
            .map(|name| unsafe { CStr::from_ptr(name).to_str().unwrap().to_string() })
    }
}
