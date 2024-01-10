use anyhow::Context;
use crate::bindings;

const ADLX_DLL_NAME: &str = "amdadlx64.dll";
const ADLX_QUERY_FULL_VERSION_FUNCTION_NAME: &str = "ADLXQueryFullVersion";
const ADLX_QUERY_VERSION_FUNCTION_NAME: &str = "ADLXQuerVersion";
const ADLX_INIT_WITH_CALLER_ADL_FUNCTION_NAME: &str = "ADLXInitializeWithCalledAdl";
const ADLX_INIT_FUNCTION_NAME: &str = "ADLXInitialize";
const ADLX_TERMINATE_FUNCTION_NAME: &str = "ADLXTerminate";

type AdlxInitFunction = unsafe extern fn(u64, *mut *mut bindings::IADLXSystem) -> bindings::ADLX_RESULT;

pub struct AdlxHelper {
    lib: libloading::Library,

    init_fn: AdlxInitFunction,
}

pub fn init_helper() -> anyhow::Result<AdlxHelper> {
    let helper = unsafe {
        let lib = libloading::Library::new(ADLX_DLL_NAME).context("Failed to load amdadlx DLL")?;

        let init_fn = *lib.get(ADLX_INIT_FUNCTION_NAME.as_bytes()).context("Failed to get init function")?;

        AdlxHelper {
            lib,
            init_fn,
        }
    };

    Ok(helper)
}