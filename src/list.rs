use super::{
    ffi,
    interface::{Interface, InterfaceImpl},
};

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_list/>
#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXList")]
pub struct List(InterfaceImpl);

unsafe impl Interface for List {
    type Impl = ffi::IADLXList;
    type Vtable = ffi::IADLXListVtbl;
    const IID: &'static str = "IADLXList";
}

impl List {
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_list__size/#doxid-d-o-x-i-a-d-l-x-list-size>
    #[doc(alias = "Size")]
    pub fn size(&self) -> u32 {
        unsafe { (self.vtable().Size.unwrap())(self.as_raw()) }
    }
}
