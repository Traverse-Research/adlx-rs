#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub(crate) fn adlx_result_to_error(res: ADLX_RESULT) -> anyhow::Result<()> {
    match res {
        ADLX_RESULT_ADLX_OK => Ok(()),
        ADLX_RESULT_ADLX_ALREADY_ENABLED => {
            Err(anyhow::anyhow!("ADLX_RESULT_ADLX_ALREADY_ENABLED"))
        }
        ADLX_RESULT_ADLX_ALREADY_INITIALIZED => {
            Err(anyhow::anyhow!("ADLX_RESULT_ADLX_ALREADY_INITIALIZED"))
        }
        ADLX_RESULT_ADLX_FAIL => Err(anyhow::anyhow!("ADLX_RESULT_ADLX_FAIL")),
        ADLX_RESULT_ADLX_INVALID_ARGS => Err(anyhow::anyhow!("ADLX_RESULT_ADLX_INVALID_ARGS")),
        ADLX_RESULT_ADLX_BAD_VER => Err(anyhow::anyhow!("ADLX_RESULT_ADLX_BAD_VER")),
        ADLX_RESULT_ADLX_UNKNOWN_INTERFACE => {
            Err(anyhow::anyhow!("ADLX_RESULT_ADLX_UNKNOWN_INTERFACE"))
        }
        ADLX_RESULT_ADLX_TERMINATED => Err(anyhow::anyhow!("ADLX_RESULT_ADLX_TERMINATED")),
        ADLX_RESULT_ADLX_ADL_INIT_ERROR => Err(anyhow::anyhow!("ADLX_RESULT_ADLX_ADL_INIT_ERROR")),
        ADLX_RESULT_ADLX_NOT_FOUND => Err(anyhow::anyhow!("ADLX_RESULT_ADLX_NOT_FOUND")),
        ADLX_RESULT_ADLX_INVALID_OBJECT => Err(anyhow::anyhow!("ADLX_RESULT_ADLX_INVALID_OBJECT")),
        ADLX_RESULT_ADLX_ORPHAN_OBJECTS => Err(anyhow::anyhow!("ADLX_RESULT_ADLX_ORPHAN_OBJECTS")),
        ADLX_RESULT_ADLX_NOT_SUPPORTED => Err(anyhow::anyhow!("ADLX_RESULT_ADLX_NOT_SUPPORTED")),
        ADLX_RESULT_ADLX_PENDING_OPERATION => {
            Err(anyhow::anyhow!("ADLX_RESULT_ADLX_PENDING_OPERATION"))
        }
        ADLX_RESULT_ADLX_GPU_INACTIVE => Err(anyhow::anyhow!("ADLX_RESULT_ADLX_GPU_INACTIVE")),
        _ => Err(anyhow::anyhow!("Unknown ADLX_RESULT error")),
    }
}
