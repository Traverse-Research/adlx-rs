use std::{ffi::CStr, mem::MaybeUninit};

use crate::bindings as ffi;

use super::{
    interface::Interface,
    result::{Error, Result},
};

// TODO: Don't derive Clone
#[derive(Debug)]
#[doc(alias = "IADLXSystem")]
#[repr(transparent)]
// WARNING: IADLXSystem is not a refcounted IADLXInterface-like object!
pub struct Gpu(*mut ffi::IADLXGPU);

// impl Interface for System {
//     type Vtable = ffi::IADLXSystemVtbl;
// }

impl Gpu {
    /// Creates an `Interface` by taking ownership of the `raw` COM interface pointer.
    ///
    /// # Safety
    ///
    /// The `raw` pointer must be owned by the caller and represent a valid COM interface pointer. In other words,
    /// it must point to a vtable beginning with the `IUnknown` function pointers and match the vtable of `Interface`.
    pub(crate) unsafe fn from_raw(raw: *mut ffi::IADLXGPU) -> Self {
        Self(raw)
    }

    fn vtable(&self) -> &ffi::IADLXGPUVtbl {
        unsafe { &*(*self.0).pVtbl }
    }

    #[doc(alias = "Name")]
    pub fn name(&self) -> Result<String> {
        let mut name = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().Name.unwrap())(self.0, name.as_mut_ptr()) };
        Error::from_result_with_assume_init_on_success(result, name)
            .map(|name| unsafe { CStr::from_ptr(name).to_str().unwrap().to_string() })
    }
    #[doc(alias = "QueryInterface")]
    // TODO: Fish IID from Interface trait!
    pub fn cast<I: Interface>(&self, interface: &str) -> Result<I> {
        let interface_wide = interface
            .encode_utf16()
            .chain(std::iter::once(0u16))
            .collect::<Vec<_>>();
        let mut result__ = std::mem::MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().QueryInterface.unwrap())(
                self.0,
                interface_wide.as_ptr(),
                result__.as_mut_ptr(),
            )
        };
        Error::from_result(result).map(|()| unsafe { I::from_raw(result__.assume_init()) })
    }
}

impl Drop for Gpu {
    fn drop(&mut self) {
        unsafe {
            (self.vtable().Release.unwrap())(self.0);
        }
    }
}
