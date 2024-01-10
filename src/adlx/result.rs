use std::{fmt, mem::MaybeUninit};

use crate::bindings as ffi;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Error(ffi::ADLX_RESULT);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            ffi::ADLX_RESULT_ADLX_OK => "This result indicates success",
            ffi::ADLX_RESULT_ADLX_ALREADY_ENABLED => "The asked action is already enabled",
            ffi::ADLX_RESULT_ADLX_ALREADY_INITIALIZED => {
                "This result indicates that ADLX has a unspecified type of initialization"
            }
            ffi::ADLX_RESULT_ADLX_FAIL => "This result indicates an unspecified failure",
            ffi::ADLX_RESULT_ADLX_INVALID_ARGS => "The arguments are invalid",
            ffi::ADLX_RESULT_ADLX_BAD_VER => {
                "The asked version is incompatible with the current version"
            }
            ffi::ADLX_RESULT_ADLX_UNKNOWN_INTERFACE => {
                "This result indicates that an unknown interface was asked"
            }
            ffi::ADLX_RESULT_ADLX_TERMINATED => {
                "The calls were made in an interface after ADLX was terminated"
            }
            ffi::ADLX_RESULT_ADLX_ADL_INIT_ERROR => "The ADL initialization failed",
            ffi::ADLX_RESULT_ADLX_NOT_FOUND => "The item is not found",
            ffi::ADLX_RESULT_ADLX_INVALID_OBJECT => "The method was called into an invalid object",
            ffi::ADLX_RESULT_ADLX_ORPHAN_OBJECTS => {
                "This result indicates that ADLX was terminated with outstanding ADLX objects. Any \
                 interface obtained from ADLX points to invalid memory and calls in their methods \
                 will result in unexpected behavior"
            }
            ffi::ADLX_RESULT_ADLX_NOT_SUPPORTED => "The asked feature is not supported",
            ffi::ADLX_RESULT_ADLX_PENDING_OPERATION => {
                "This result indicates a failure due to an operation currently in progress"
            }
            ffi::ADLX_RESULT_ADLX_GPU_INACTIVE => "The GPU is inactive",

            x => return write!(f, "Unknown ADLX_RESULT `{x}`"),
        })
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADLX_RESULT({self})")
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn from_result(result: ffi::ADLX_RESULT) -> Result<(), Self> {
        match result {
            ffi::ADLX_RESULT_ADLX_OK => Ok(()),
            x => Err(Self(x)),
        }
    }

    pub fn from_result_with_assume_init_on_success<T>(
        result: ffi::ADLX_RESULT,
        ret: MaybeUninit<T>,
    ) -> Result<T, Self> {
        Self::from_result(result).map(|()| unsafe { ret.assume_init() })
    }
}
