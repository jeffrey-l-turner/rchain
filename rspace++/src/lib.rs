pub mod diskconc;
pub mod diskseq;
pub mod example;
pub mod memconc;
pub mod memseq;
pub mod rspace;
pub mod shared;

use rspace::RSpace;
use shared::OptionResult;
use std::ffi::{c_char, CStr};

#[repr(C)]
pub struct Space<D, K> {
    rspace: RSpace<D, K>,
}

#[no_mangle]
pub extern "C" fn space_new() -> *mut Space<String, String> {
    Box::into_raw(Box::new(Space {
        rspace: RSpace::create().unwrap(),
    }))
}

#[no_mangle]
pub extern "C" fn space_get_once_durable_concurrent(
    rspace: *mut Space<String, String>,
    channel: *const c_char,
    entry: *const c_char,
) -> *mut Option<OptionResult<String, String>> {
    unsafe {
        let channel_str = CStr::from_ptr(channel).to_str().unwrap();
        let entry_str = { CStr::from_ptr(entry) }.to_string_lossy().into_owned();

        let result = (*rspace)
            .rspace
            .get_once_durable_concurrent(channel_str, entry_str);

        Box::into_raw(Box::new(result))
    }
}

#[no_mangle]
pub extern "C" fn space_print(rspace: *mut Space<String, String>, channel: *const c_char) -> () {
    unsafe {
        let channel_str = CStr::from_ptr(channel).to_str().unwrap();
        (*rspace).rspace.print_store(channel_str)
    }
}
