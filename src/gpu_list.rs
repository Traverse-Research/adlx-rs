use std::{mem::MaybeUninit, ops::Deref};

use super::{
    ffi,
    gpu::Gpu,
    interface::Interface,
    list::List,
    result::{Error, Result},
};

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_g_p_u_list/>
#[derive(Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXGPUList")]
pub struct GpuList(List);

unsafe impl Interface for GpuList {
    type Impl = ffi::IADLXGPUList;
    type Vtable = ffi::IADLXGPUListVtbl;
    const IID: &'static str = "IADLXGPUList";
}

impl Deref for GpuList {
    type Target = List;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GpuList {
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_g_p_u_list__at/>
    #[doc(alias = "At_GPUList")]
    pub fn at(&self, location: u32) -> Result<Gpu> {
        let mut gpu = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().At_GPUList.unwrap())(self.as_raw(), location, gpu.as_mut_ptr())
        };
        Error::from_result_with_assume_init_on_success(result, gpu)
            .map(|gpu| unsafe { Gpu::from_raw(gpu) })
    }
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_g_p_u_list__add__back/#doxid-d-o-x-i-a-d-l-x-g-p-u-list-add-back>
    #[doc(alias = "Add_Back_GPUList")]
    // TODO(Marijn): This API does not allow moves of derivatives, such as Gpu1.
    pub fn add_back(&self, gpu: Gpu) -> Result<()> {
        let result = unsafe {
            // TODO: Assume ownership is consumed here?
            (self.vtable().Add_Back_GPUList.unwrap())(self.as_raw(), gpu.into_raw())
        };
        Error::from_result(result)
    }

    pub fn iter(&self) -> GpuIterator {
        GpuIterator {
            list: self,
            i: 0,
        }
    }
}

pub struct GpuIterator<'a> {
    list: &'a GpuList,
    i: u32,
}

impl<'a> Iterator for GpuIterator<'a> {
    type Item = Gpu;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.list.size() {
            let gpu = self.list.at(self.i).unwrap();
            self.i += 1;
            Some(gpu)
        } else {
            None
        }
    }
}

impl ExactSizeIterator for GpuIterator<'_> {
    fn len(&self) -> usize {
        self.list.size() as usize
    }
}
