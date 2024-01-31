//! `ADLX` interfaces look identical to COM objects.

use super::{
    ffi,
    result::{Error, Result},
};

/// # Safety
/// All `unsafe` implementers of this trait should guarantee [`ffi::IADLXInterface`] is the base
/// type.
///
/// The struct must have the same layout as [`*mut IADLXInterface`][ffi::IADLXInterface], preferably
/// in terms of [`InterfaceImpl`] for automatic refcount management.
pub unsafe trait Interface: Sized {
    type Impl;
    type Vtable;
    const IID: &'static str;

    /// A reference to the interface's vtable
    #[doc(hidden)]
    fn vtable(&self) -> &Self::Vtable {
        // SAFETY: the implementor of the trait guarantees that `Self` is castable to its vtable
        unsafe { self.assume_vtable::<Self>() }
    }

    /// Cast this interface as a reference to the supplied interfaces [`Self::Vtable`]
    ///
    /// # Safety
    ///
    /// This is safe if `T` is an equivalent interface to `Self` or a super interface.
    /// In other words, `T::Vtable` must be equivalent to the beginning of [`Self::Vtable`].
    #[doc(hidden)]
    unsafe fn assume_vtable<T: Interface>(&self) -> &T::Vtable {
        let base_vtable = (*self.as_raw().cast::<ffi::IADLXInterface>()).pVtbl;
        &*<*const _>::cast(base_vtable)
    }

    /// Creates an [`Interface`] by taking ownership of the `raw` COM/ADLX interface pointer.
    ///
    /// # Safety
    ///
    /// The `raw` pointer must be owned by the caller and represent a valid ADLX interface pointer.
    /// In other words, it must point to a vtable beginning with the [`ffi::IADLXInterfaceVtbl`]
    /// function pointers and match the vtable of [`Self::Vtable`].
    unsafe fn from_raw(raw: *mut Self::Impl) -> Self {
        std::mem::transmute_copy(&raw)
    }

    /// Returns the raw COM/ADLX interface pointer and abandons ownership. It is the caller's
    /// responsibility to release the COM/ADLX interface pointer.
    fn into_raw(self) -> *mut Self::Impl {
        // SAFETY: implementors of this trait must guarantee that the implementing type has a pointer in-memory representation
        let raw = self.as_raw();
        std::mem::forget(self);
        raw
    }

    /// Returns the raw COM/ADLX interface pointer. The resulting pointer continues to be owned by
    /// the [`Interface`] implementation.
    // TODO: With a generic on `InterfaceImpl` trait implementers (via derive-macro-generated
    // implementation) could provide their type-safe pointer value
    fn as_raw(&self) -> *mut Self::Impl {
        unsafe { std::mem::transmute_copy(self) }
    }

    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_interface__query_interface/>
    // TODO(Marijn): Or deref every interface to `InterfaceImpl`, per a true interface hierarchy?
    #[doc(alias = "QueryInterface")]
    fn cast<I: Interface>(&self) -> Result<I> {
        let interface: InterfaceImpl = unsafe { std::mem::transmute_copy(self) };
        interface.cast()
    }
}

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_interface/>
///
/// All `IADLX*` types are expected to own this object, and access the vtable by implementing
/// [`Interface`] with the appropriate owning type set in [`Interface::Impl`] and corresponding
/// vtable in [`Interface::Vtable`].
///
/// Owning this type means proper [`Drop`] and [`Clone`] semantics, and less manual conversions like
/// e.g. [`Interface::as_raw()`].
// TODO(Marijn): We could also achieve that by creating a little macro that provides the conversions, while being more type-safe!
#[derive(Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXInterface")]
pub struct InterfaceImpl<'lib>(*mut ffi::IADLXInterface, std::marker::PhantomData<&'lib ()>);

unsafe impl Interface for InterfaceImpl<'_> {
    type Impl = ffi::IADLXInterface;
    type Vtable = ffi::IADLXInterfaceVtbl;
    const IID: &'static str = "IADLXInterface";
}

impl InterfaceImpl<'_> {
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_interface__query_interface/>
    #[doc(alias = "QueryInterface")]
    pub fn cast<I: Interface>(&self) -> Result<I> {
        let interface_name = I::IID
            // TODO: Use windows-rs' helpers to create static wchars?
            .encode_utf16()
            .chain(std::iter::once(0u16))
            .collect::<Vec<_>>();
        let mut interface = std::mem::MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().QueryInterface.unwrap())(
                self.0,
                interface_name.as_ptr(),
                interface.as_mut_ptr(),
            )
        };
        Error::from_result(result).map(|()| unsafe { I::from_raw(interface.assume_init().cast()) })
    }
}

impl Drop for InterfaceImpl<'_> {
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_interface__release/>
    #[doc(alias = "Release")]
    fn drop(&mut self) {
        let _rc = unsafe { (self.vtable().Release.unwrap())(self.0) };
    }
}

impl Clone for InterfaceImpl<'_> {
    /// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_interface__acquire/>
    #[doc(alias = "Acquire")]
    fn clone(&self) -> Self {
        let _rc = unsafe { (self.vtable().Acquire.unwrap())(self.0) };
        Self(self.0, std::marker::PhantomData)
    }
}

unsafe impl Send for InterfaceImpl<'_> {}
unsafe impl Sync for InterfaceImpl<'_> {}
