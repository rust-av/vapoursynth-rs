//! VapourSynth script-related things.

use std::ptr::{self, NonNull};
use std::sync::Once;
use std::sync::atomic::{AtomicPtr, Ordering};
use vapoursynth_sys as ffi;

/// A wrapper for the VSScript API.
#[derive(Debug, Clone, Copy)]
pub(crate) struct VSScriptAPI {
    handle: NonNull<ffi::VSSCRIPTAPI>,
}

unsafe impl Send for VSScriptAPI {}
unsafe impl Sync for VSScriptAPI {}

/// A cached VSScript API pointer.
static RAW_VSSCRIPT_API: AtomicPtr<ffi::VSSCRIPTAPI> = AtomicPtr::new(ptr::null_mut());

impl VSScriptAPI {
    /// Retrieves the VSScript API.
    ///
    /// Returns `None` on error, for example if the requested API version is not supported.
    #[inline]
    pub(crate) fn get() -> Option<Self> {
        // Check if we already have the API.
        let handle = RAW_VSSCRIPT_API.load(Ordering::Relaxed);

        let handle = if handle.is_null() {
            // Attempt retrieving it otherwise.
            let version = ffi::VSSCRIPT_API_MAJOR << 16 | ffi::VSSCRIPT_API_MINOR;
            let handle = unsafe { ffi::getVSScriptAPI(version as i32) } as *mut ffi::VSSCRIPTAPI;

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
                RAW_VSSCRIPT_API.store(handle, Ordering::Relaxed);
            }
            handle
        } else {
            handle
        };

        if handle.is_null() {
            None
        } else {
            Some(Self {
                handle: unsafe { NonNull::new_unchecked(handle) },
            })
        }
    }

    #[inline]
    pub(crate) fn handle(&self) -> &ffi::VSSCRIPTAPI {
        unsafe { self.handle.as_ref() }
    }
}

/// Ensures the VSScript API has been initialized.
#[inline]
pub(crate) fn maybe_initialize() {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        VSScriptAPI::get().expect("Failed to get VSScript API");
    });
}

mod errors;
pub use self::errors::{Error, VSScriptError};

mod environment;
pub use self::environment::{Environment, EvalFlags};
