use std::ffi::CStr;

use anyhow::Context;

use crate::bindings;

pub struct AdlxHelper {
    lib: libloading::Library,
    init_fn: bindings::ADLXInitialize_Fn,
}

pub fn init_helper() -> anyhow::Result<AdlxHelper> {
    let helper = unsafe {
        // libloading requires an OsString, not a CStr.
        let dll_name = CStr::from_bytes_with_nul(bindings::ADLX_DLL_NAME)
            .unwrap()
            .to_str()
            .unwrap();
        let lib = libloading::Library::new(dll_name)
            .with_context(|| format!("Failed to load `{dll_name}`"))?;

        let init_fn = *lib
            .get(bindings::ADLX_INIT_FUNCTION_NAME)
            .context("Failed to get init function")?;

        AdlxHelper { lib, init_fn }
    };

    Ok(helper)
}
