//! VapourSynth script-related things.

use std::env;
use std::ptr::NonNull;
use std::sync::OnceLock;
use std::sync::atomic::Ordering;
use vapoursynth_sys::{self as ffi, VSSCRIPT_LIB_NAMES, VSSCRIPT_PATH_VARIABLE, VSScriptAPILoader};

/// A wrapper for the VSScript API.
#[derive(Debug, Clone, Copy)]
pub(crate) struct VSScriptAPI {
    handle: NonNull<ffi::VSSCRIPTAPI>,
}

unsafe impl Send for VSScriptAPI {}
unsafe impl Sync for VSScriptAPI {}

static VSSCRIPT_API_LOADER: OnceLock<Option<VSScriptAPILoader>> = OnceLock::new();

impl VSScriptAPI {
    /// Retrieves the VSScript API.
    ///
    /// Returns `None` on error, for example if the requested API version is not supported.
    #[inline]
    pub(crate) fn get() -> Option<Self> {
        // Check if we already have loaded the library
        let handle = VSSCRIPT_API_LOADER
            .get_or_init(|| {
                // Attempt opening the VSScript library
                let loader = if let Ok(vsscript_path) = env::var(VSSCRIPT_PATH_VARIABLE) {
                    unsafe { VSScriptAPILoader::new(vsscript_path) }.ok()
                } else {
                    VSSCRIPT_LIB_NAMES
                        .iter()
                        .find_map(|path| unsafe { VSScriptAPILoader::new(*path) }.ok())
                }?;

                let version = ffi::VSSCRIPT_API_MAJOR << 16 | ffi::VSSCRIPT_API_MINOR;
                let handle =
                    unsafe { loader.getVSScriptAPI(version as i32) } as *mut ffi::VSSCRIPTAPI;

                if !handle.is_null() {
                    // Verify the VSScript API version.
                    let api_version = unsafe { ((*handle).getAPIVersion.unwrap())() };
                    let major = api_version >> 16;
                    let minor = api_version & 0xFFFF;

                    if major as u32 != ffi::VSSCRIPT_API_MAJOR {
                        panic!(
                            "Invalid VSScript major API version (expected: {}, got: {})",
                            ffi::VSSCRIPT_API_MAJOR,
                            major
                        );
                    } else if (minor as u32) < ffi::VSSCRIPT_API_MINOR {
                        panic!(
                            "Invalid VSScript minor API version (expected: >= {}, got: {})",
                            ffi::VSSCRIPT_API_MINOR,
                            minor
                        );
                    }

                    // If we successfully retrieved the API, cache it.
                    loader.RAW_VSSCRIPT_API.store(handle, Ordering::Relaxed);
                }

                Some(loader)
            })
            .as_ref()
            .map(|loader| loader.RAW_VSSCRIPT_API.load(Ordering::Relaxed));

        if let Some(ptr) = handle
            && !ptr.is_null()
        {
            Some(Self {
                handle: unsafe { NonNull::new_unchecked(ptr) },
            })
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn handle(&self) -> &ffi::VSSCRIPTAPI {
        unsafe { self.handle.as_ref() }
    }
}

mod errors;
pub use self::errors::{Error, VSScriptError};

mod environment;
pub use self::environment::{Environment, EvalFlags};
