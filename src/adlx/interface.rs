//! `ADLX` interfaces look identical to COM objects.

use crate::bindings as ffi;

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
        let base_vtable = (*self.imp().cast::<ffi::IADLXInterface>()).pVtbl;
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

    // TODO: With a generic on `InterfaceImpl` trait implementers (via derive-macro-generated
    // implementation) could provide their type-safe pointer value
    fn imp(&self) -> *mut Self::Impl {
        unsafe { std::mem::transmute_copy(self) }
    }
}

/// All `IADLX*` types are expected to own this object, and access the vtable by implementing
/// [`Interface`] with the appropriate owning type set in [`Interface::Impl`] and corresponding
/// vtable in [`Interface::Vtable`].
///
/// Owning this type means proper [`Drop`] and [`Clone`] semantics, and less manual conversions like
/// e.g. [`Interface::imp()`].
// TODO(Marijn): We could also achieve that by creating a little macro that provides the conversions, while being more type-safe!
#[derive(Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXInterface")]
pub struct InterfaceImpl(*mut ffi::IADLXInterface);

unsafe impl Interface for InterfaceImpl {
    type Impl = ffi::IADLXInterface;
    type Vtable = ffi::IADLXInterfaceVtbl;
    const IID: &'static str = "IADLXInterface";
}

impl Drop for InterfaceImpl {
    fn drop(&mut self) {
        let _rc = unsafe { (self.vtable().Release.unwrap())(self.0) };
    }
}

impl Clone for InterfaceImpl {
    fn clone(&self) -> Self {
        let _rc = unsafe { (self.vtable().Acquire.unwrap())(self.0) };
        Self(self.0)
    }
}
