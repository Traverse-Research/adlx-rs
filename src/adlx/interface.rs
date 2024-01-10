//! `ADLX` interfaces look identical to COM objects.

use crate::bindings as ffi;

/// # Safety
/// By implementing this trait you guarantee that
pub unsafe trait Interface: Sized {
    type Vtable;
    const IID: &'static str;

    /// A reference to the interface's vtable
    #[doc(hidden)]
    fn vtable(&self) -> &Self::Vtable {
        // SAFETY: the implementor of the trait guarantees that `Self` is castable to its vtable
        unsafe { self.assume_vtable::<Self>() }
    }

    /// Cast this interface as a reference to the supplied interfaces `Vtable`
    ///
    /// # Safety
    ///
    /// This is safe if `T` is an equivalent interface to `Self` or a super interface.
    /// In other words, `T::Vtable` must be equivalent to the beginning of `Self::Vtable`.
    #[doc(hidden)]
    unsafe fn assume_vtable<T: Interface>(&self) -> &T::Vtable {
        let base_vtable = self.as_adlx_interface().pVtbl;
        &*<*const _>::cast(base_vtable)
    }

    /// # Safety
    /// All `unsafe` implementers of this trait should guarantee [`ffi::IADLXInterface`] is the base type.
    ///
    /// This function exists so that not everyone needs to implement this function in terms of `{ self.0 }`.
    fn as_adlx_interface(&self) -> &ffi::IADLXInterface {
        unsafe { std::mem::transmute_copy(self) }
    }

    /// Creates an `Interface` by taking ownership of the `raw` COM interface pointer.
    ///
    /// # Safety
    ///
    /// The `raw` pointer must be owned by the caller and represent a valid COM interface pointer. In other words,
    /// it must point to a vtable beginning with the `IUnknown` function pointers and match the vtable of `Interface`.
    unsafe fn from_raw(raw: *mut std::ffi::c_void) -> Self {
        std::mem::transmute_copy(&raw)
    }

    // fn from_abi(ptr: *mut c_void) -> Self {}
}

/// All `IADLX*` types are expected to own this object, and access the vtable by implementing
/// [`Interface`] with the appropriate type set in [`Interface::Vtable`].  Owning this type
/// means proper [`Drop`] and [`Clone`] semantics, and less manual conversions like e.g.
/// `as_adlx_interface()`.
// TODO(Marijn): We could also achieve that by creating a little macro that provides the conversions, while being more type-safe!
#[derive(Debug)]
#[repr(transparent)]
pub struct ADLXInterface(*mut ffi::IADLXInterface);

unsafe impl Interface for ADLXInterface {
    type Vtable = ffi::IADLXInterfaceVtbl;
    const IID: &'static str = "IADLXInterface";
}

impl Drop for ADLXInterface {
    fn drop(&mut self) {
        let _rc = unsafe { (self.vtable().Release.unwrap())(self.0) };
    }
}

impl Clone for ADLXInterface {
    fn clone(&self) -> Self {
        let _rc = unsafe { (self.vtable().Acquire.unwrap())(self.0) };
        Self(self.0)
    }
}
