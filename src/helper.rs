use crate::bindings;
use anyhow::Context;

const ADLX_DLL_NAME: &str = "amdadlx64.dll";
const ADLX_QUERY_FULL_VERSION_FUNCTION_NAME: &str = "ADLXQueryFullVersion";
const ADLX_QUERY_VERSION_FUNCTION_NAME: &str = "ADLXQueryVersion";
const ADLX_INIT_WITH_CALLER_ADL_FUNCTION_NAME: &str = "ADLXInitializeWithCallerAdl";
const ADLX_INIT_WITH_INCOMPATIBLE_DRIVER_FUNCTION_NAME: &str =
    "ADLXInitializeWithIncompatibleDriver";
const ADLX_INIT_FUNCTION_NAME: &str = "ADLXInitialize";
const ADLX_TERMINATE_FUNCTION_NAME: &str = "ADLXTerminate";

type AdlxQueryFullVersionFn = unsafe extern "C" fn(*mut u64) -> bindings::ADLX_RESULT;
type AdlxQueryVersionFn = unsafe extern "C" fn(*mut &str) -> bindings::ADLX_RESULT;
type AdlxInitWithCallerAdlFn = unsafe extern "C" fn(
    u64,
    *mut *mut bindings::IADLXSystem,
    *mut *mut bindings::IADLMapping,
) -> bindings::ADLX_RESULT;
type AdlxInitFn =
    unsafe extern "C" fn(u64, *mut *mut bindings::IADLXSystem) -> bindings::ADLX_RESULT;
type AdlxTerminateFn = unsafe extern "C" fn() -> bindings::ADLX_RESULT;

pub struct AdlxHelper {
    _lib: libloading::Library,

    pub full_version_fn: AdlxQueryFullVersionFn,
    pub version_fn: AdlxQueryVersionFn,
    pub init_with_adl_fn: AdlxInitWithCallerAdlFn,
    pub init_with_incompatible_driver_fn: AdlxInitFn,
    pub init_fn: AdlxInitFn,
    pub terminate_fn: AdlxTerminateFn,
}

pub fn init_helper() -> anyhow::Result<AdlxHelper> {
    let helper = unsafe {
        let lib = libloading::Library::new(ADLX_DLL_NAME).context("Failed to load amdadlx DLL")?;

        let full_version_fn = *lib
            .get(ADLX_QUERY_FULL_VERSION_FUNCTION_NAME.as_bytes())
            .context("Failed to get full version function")?;
        let version_fn = *lib
            .get(ADLX_QUERY_VERSION_FUNCTION_NAME.as_bytes())
            .context("Failed to get version function")?;
        let init_with_adl_fn = *lib
            .get(ADLX_INIT_WITH_CALLER_ADL_FUNCTION_NAME.as_bytes())
            .context("Failed to get initWithAdl function")?;
        let init_with_incompatible_driver_fn = *lib
            .get(ADLX_INIT_WITH_INCOMPATIBLE_DRIVER_FUNCTION_NAME.as_bytes())
            .context("Failed to get init with incompatible driver function")?;
        let init_fn = *lib
            .get(ADLX_INIT_FUNCTION_NAME.as_bytes())
            .context("Failed to get init function")?;
        let terminate_fn = *lib
            .get(ADLX_TERMINATE_FUNCTION_NAME.as_bytes())
            .context("Failed to get terminate function")?;

        AdlxHelper {
            _lib: lib,

            full_version_fn,
            version_fn,
            init_with_adl_fn,
            init_with_incompatible_driver_fn,
            init_fn,
            terminate_fn,
        }
    };

    Ok(helper)
}
