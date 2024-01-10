use std::{ffi::CStr, mem::MaybeUninit};

use anyhow::{ensure, Context, Result};

use super::{result::Error, system::System};
use crate::bindings as ffi;

// TODO: This should be a singleton
pub struct AdlxHelper {
    // Keepalive for the loaded library
    #[allow(dead_code)]
    lib: libloading::Library,
    system: super::system::System,
    terminate_fn: ffi::ADLXTerminate_Fn,
    // query_version_fn: ffi::ADLXQueryVersion_Fn,
    // query_full_version_fn: ffi::ADLXQueryFullVersion_Fn,
}

impl AdlxHelper {
    pub fn new() -> Result<Self> {
        fn load_symbol<T: Copy>(lib: &libloading::Library, name: &[u8]) -> Result<Option<T>> {
            let name_c = CStr::from_bytes_with_nul(name)?;
            let sym: Option<T> = *unsafe { lib.get(name) }
                .with_context(|| format!("Failed to get function symbol {name_c:?}"))?;
            // Keep the symbol wrapped in an `Option`, as that is what `bindgen` generates
            ensure!(sym.is_some(), "{name_c:?} cannot be NULL");
            Ok(sym)
        }

        unsafe {
            // libloading requires an OsString, not a CStr.
            let dll_name = CStr::from_bytes_with_nul(ffi::ADLX_DLL_NAME)
                .unwrap()
                .to_str()
                .unwrap();
            let lib = libloading::Library::new(dll_name)
                .with_context(|| format!("Failed to load `{dll_name}`"))?;

            let init_fn: ffi::ADLXInitialize_Fn = load_symbol(&lib, ffi::ADLX_INIT_FUNCTION_NAME)?;
            let terminate_fn: ffi::ADLXTerminate_Fn =
                load_symbol(&lib, ffi::ADLX_TERMINATE_FUNCTION_NAME)?;
            // let query_version_fn: ffi::ADLXQueryVersion_Fn =
            //     load_symbol(&lib, ffi::ADLX_QUERY_VERSION_FUNCTION_NAME)?;
            // let query_full_version_fn: ffi::ADLXQueryFullVersion_Fn =
            //     load_symbol(&lib, ffi::ADLX_QUERY_FULL_VERSION_FUNCTION_NAME)?;
            let mut system = MaybeUninit::uninit();
            let result = (init_fn.unwrap_unchecked())(1337, system.as_mut_ptr());
            let system = Error::from_result_with_assume_init_on_success(result, system)?;
            let system = System::from_raw(system);

            // TODO: Lifetime all returned interfaces such that they _CANNOT_ outlive AdlxHelper, which has the library open!
            Ok(Self {
                lib,
                system,
                terminate_fn,
                // query_version_fn,
                // query_full_version_fn,
            })
        }
    }

    pub fn system(&self) -> &System {
        &self.system
    }
}

impl Drop for AdlxHelper {
    fn drop(&mut self) {
        // SAFETY: Nullity checked at load-time
        let result = unsafe { (self.terminate_fn.unwrap_unchecked())() };
        if let Err(e) = Error::from_result(result) {
            eprintln!("Terminate failed with {e:?}")
        }
    }
}
