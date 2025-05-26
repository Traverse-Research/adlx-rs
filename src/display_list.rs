use std::{mem::MaybeUninit, ops::Deref};

use super::{
    ffi,
    interface::Interface,
    list::List,
    result::{Error, Result},
    Display,
};

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_display_list/>
#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXDisplayList")]
pub struct DisplayList(List);

unsafe impl Interface for DisplayList {
    type Impl = ffi::IADLXDisplayList;
    type Vtable = ffi::IADLXDisplayListVtbl;
    const IID: &'static str = "IADLXDisplayList";
}

impl Deref for DisplayList {
    type Target = List;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DisplayList {
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_display_list__at/>
    #[doc(alias = "At_DisplayList")]
    pub fn at(&self, location: u32) -> Result<Display> {
        let mut display = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().At_DisplayList.unwrap())(self.as_raw(), location, display.as_mut_ptr())
        };
        Error::from_result_with_assume_init_on_success(result, display)
            .map(|display| unsafe { Display::from_raw(display) })
    }

    pub fn iter(&self) -> DisplayIterator {
        DisplayIterator { list: self, i: 0 }
    }
}

pub struct DisplayIterator<'a> {
    list: &'a DisplayList,
    i: u32,
}

impl Iterator for DisplayIterator<'_> {
    type Item = Display;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.list.size() {
            let display = self.list.at(self.i).unwrap();
            self.i += 1;
            Some(display)
        } else {
            None
        }
    }
}

impl ExactSizeIterator for DisplayIterator<'_> {
    fn len(&self) -> usize {
        self.list.size() as usize
    }
}
