use std::ffi::CStr;

use anyhow::Context;

use crate::bindings::{
    self, adlx_result_to_error, ADLX_VER_BUILD_NUM, ADLX_VER_MAJOR, ADLX_VER_MINOR,
    ADLX_VER_RELEASE,
};

struct AdlxFunctions {
    lib: libloading::Library,

    full_version_fn: bindings::ADLXQueryFullVersion_Fn,
    version_fn: bindings::ADLXQueryVersion_Fn,
    init_with_adl_fn: bindings::ADLXInitializeWithCallerAdl_Fn,
    init_with_incompatible_driver_fn: bindings::ADLXInitialize_Fn,
    init_fn: bindings::ADLXInitialize_Fn,
    terminate_fn: bindings::ADLXTerminate_Fn,
}

unsafe fn load_adlx_dll() -> anyhow::Result<AdlxFunctions> {
    let dll_name = CStr::from_bytes_with_nul(bindings::ADLX_DLL_NAME)
        .unwrap()
        .to_str()
        .unwrap();
    let lib = libloading::Library::new(dll_name).context("Failed to load amdadlx DLL")?;

    let full_version_fn = *lib
        .get(bindings::ADLX_QUERY_FULL_VERSION_FUNCTION_NAME)
        .context("Failed to get full version function")?;
    let version_fn = *lib
        .get(bindings::ADLX_QUERY_VERSION_FUNCTION_NAME)
        .context("Failed to get version function")?;
    let init_with_adl_fn = *lib
        .get(bindings::ADLX_INIT_WITH_CALLER_ADL_FUNCTION_NAME)
        .context("Failed to get initWithAdl function")?;
    let init_with_incompatible_driver_fn = *lib
        .get(bindings::ADLX_INIT_WITH_INCOMPATIBLE_DRIVER_FUNCTION_NAME)
        .context("Failed to get init with incompatible driver function")?;
    let init_fn = *lib
        .get(bindings::ADLX_INIT_FUNCTION_NAME)
        .context("Failed to get init function")?;
    let terminate_fn = *lib
        .get(bindings::ADLX_TERMINATE_FUNCTION_NAME)
        .context("Failed to get terminate function")?;

    Ok(AdlxFunctions {
        lib,

        full_version_fn,
        version_fn,
        init_with_adl_fn,
        init_with_incompatible_driver_fn,
        init_fn,
        terminate_fn,
    })
}

pub struct AdlxHelper {
    functions: AdlxFunctions,

    full_version: u64,
    version: String,
    system_services: Box<bindings::IADLXSystem>,
}

impl Drop for AdlxHelper {
    fn drop(&mut self) {
        unsafe {
            adlx_result_to_error((self.functions.terminate_fn.unwrap())())
                .expect("Failed to terminate ADLX");
        }
    }
}

pub fn init_helper() -> anyhow::Result<AdlxHelper> {
    let functions = unsafe { load_adlx_dll()? };

    let mut full_version = 0;

    unsafe {
        adlx_result_to_error((functions.full_version_fn.unwrap())(&mut full_version))?;
    }

    let version = unsafe {
        let mut version = std::ptr::null();
        adlx_result_to_error((functions.version_fn.unwrap())(&mut version))?;
        CStr::from_ptr(version).to_str()?.to_string()
    };

    // TODO: C++ helper does extra things if an ADL context is provided.
    // We don't support this currently because we are only implementing ADLX
    let system = unsafe {
        let mut system = std::ptr::null_mut();
        let init_full_version = ((ADLX_VER_MAJOR as u64) << 48)
            | ((ADLX_VER_MINOR as u64) << 32)
            | ((ADLX_VER_RELEASE as u64) << 16)
            | (ADLX_VER_BUILD_NUM as u64);
        adlx_result_to_error((functions.init_fn.unwrap())(init_full_version, &mut system))?;

        Box::from_raw(system)
    };
    dbg!(&version);
    Ok(AdlxHelper {
        functions,

        full_version,
        version,
        system_services: system,
    })
}

#[test]
fn just_a_test() {
    let helper = init_helper().unwrap();
}
