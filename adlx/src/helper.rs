use std::{ffi::CStr, mem::MaybeUninit};

use anyhow::{ensure, Context, Result};

use super::{ffi, result::Error, system::System};

// Taken from ADLXDefines.h.
// Refer to ADLX_MAKE_FULL_VERSION and ADLX_FULL_VERSION.
const HEADER_VERSION: u64 = ((ffi::ADLX_VER_MAJOR as u64) << 48)
    | ((ffi::ADLX_VER_MINOR as u64) << 32)
    | ((ffi::ADLX_VER_RELEASE as u64) << 16)
    | (ffi::ADLX_VER_BUILD_NUM as u64);

// TODO: This should be a singleton
struct AdlxFunctions {
    _lib: libloading::Library,

    full_version_fn: ffi::ADLXQueryFullVersion_Fn,
    version_fn: ffi::ADLXQueryVersion_Fn,
    _init_with_adl_fn: ffi::ADLXInitializeWithCallerAdl_Fn,
    _init_with_incompatible_driver_fn: ffi::ADLXInitialize_Fn,
    init_fn: ffi::ADLXInitialize_Fn,
    terminate_fn: ffi::ADLXTerminate_Fn,
}

impl AdlxFunctions {
    unsafe fn load() -> anyhow::Result<Self> {
        let dll_name = CStr::from_bytes_with_nul(ffi::ADLX_DLL_NAME)
            .unwrap()
            .to_str()
            .unwrap();
        let lib = libloading::Library::new(dll_name)
            .with_context(|| format!("Failed to load `{dll_name}`"))?;

        fn load_symbol<T: Copy>(lib: &libloading::Library, name: &[u8]) -> Result<Option<T>> {
            let name_c = CStr::from_bytes_with_nul(name)?;
            let sym: Option<T> = *unsafe { lib.get(name) }
                .with_context(|| format!("Failed to get function symbol {name_c:?}"))?;
            // Keep the symbol wrapped in an `Option`, as that is what `bindgen` generates
            ensure!(sym.is_some(), "{name_c:?} cannot be NULL");
            Ok(sym)
        }

        let full_version_fn: ffi::ADLXQueryFullVersion_Fn =
            load_symbol(&lib, ffi::ADLX_QUERY_FULL_VERSION_FUNCTION_NAME)?;
        let version_fn: ffi::ADLXQueryVersion_Fn =
            load_symbol(&lib, ffi::ADLX_QUERY_VERSION_FUNCTION_NAME)?;
        let init_with_adl_fn: ffi::ADLXInitializeWithCallerAdl_Fn =
            load_symbol(&lib, ffi::ADLX_INIT_WITH_CALLER_ADL_FUNCTION_NAME)?;
        let init_with_incompatible_driver_fn: ffi::ADLXInitialize_Fn =
            load_symbol(&lib, ffi::ADLX_INIT_WITH_INCOMPATIBLE_DRIVER_FUNCTION_NAME)?;
        let init_fn: ffi::ADLXInitialize_Fn = load_symbol(&lib, ffi::ADLX_INIT_FUNCTION_NAME)?;
        let terminate_fn: ffi::ADLXTerminate_Fn =
            load_symbol(&lib, ffi::ADLX_TERMINATE_FUNCTION_NAME)?;

        Ok(Self {
            _lib: lib,

            full_version_fn,
            version_fn,
            _init_with_adl_fn: init_with_adl_fn,
            _init_with_incompatible_driver_fn: init_with_incompatible_driver_fn,
            init_fn,
            terminate_fn,
        })
    }
}

pub struct AdlxHelper {
    functions: AdlxFunctions,

    system: super::system::System,

    full_version: u64,
    version: String,
}

impl AdlxHelper {
    pub fn new() -> Result<Self> {
        let functions = unsafe { AdlxFunctions::load()? };

        let full_version = unsafe {
            let mut full_version = MaybeUninit::uninit();
            let result = (functions.full_version_fn.unwrap())(full_version.as_mut_ptr());

            Error::from_result_with_assume_init_on_success(result, full_version)?
        };

        let version = unsafe {
            let mut version = MaybeUninit::uninit();
            let result = (functions.version_fn.unwrap())(version.as_mut_ptr());
            Error::from_result_with_assume_init_on_success(result, version)
                .map(|version| CStr::from_ptr(version).to_str().unwrap().to_string())?
        };

        // TODO: C++ helper does extra things if an ADL context is provided.
        // We don't support this currently because we are only implementing ADLX
        let system = unsafe {
            let mut system = std::ptr::null_mut();

            Error::from_result((functions.init_fn.unwrap())(HEADER_VERSION, &mut system))?;

            System::from_raw(system)
        };
        dbg!(&version);
        Ok(AdlxHelper {
            functions,

            full_version,
            version,
            system,
        })
    }

    pub fn system(&self) -> &System {
        &self.system
    }

    pub fn full_version(&self) -> u64 {
        self.full_version
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

impl Drop for AdlxHelper {
    fn drop(&mut self) {
        // SAFETY: Nullity checked at load-time
        let result = unsafe { (self.functions.terminate_fn.unwrap_unchecked())() };
        if let Err(e) = Error::from_result(result) {
            eprintln!("Terminate failed with {e:?}")
        }
    }
}
