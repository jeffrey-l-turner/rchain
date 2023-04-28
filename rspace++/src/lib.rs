pub mod diskconc;
pub mod diskseq;
pub mod example;
pub mod memconc;
pub mod memseq;
pub mod rspace;
pub mod rtypes;
pub mod shared;

use prost::Message;
use rspace::RSpace;
use rtypes::rtypes::{Receive, Send};
use shared::{OptionResult, Pattern};
use std::ffi::{c_char, CStr};

#[repr(C)]
pub struct Space {
    rspace: RSpace<rtypes::rtypes::Send, rtypes::rtypes::Receive>,
}

#[no_mangle]
pub extern "C" fn space_new() -> *mut Space {
    Box::into_raw(Box::new(Space {
        rspace: RSpace::create().unwrap(),
    }))
}

// Verb Set 1
#[no_mangle]
pub extern "C" fn space_get_once_durable_concurrent(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *mut Option<OptionResult> {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result = (*rspace).rspace.get_once_durable_concurrent(pdata);

        Box::into_raw(Box::new(result))
    }
}

// Verb Set 3
#[no_mangle]
pub extern "C" fn space_put_once_durable_concurrent(
    rspace: *mut Space,
    cdata_ptr: *const u8,
    cdata_len: usize,
) -> *mut Option<Vec<OptionResult>> {
    unsafe {
        let cdata_buf = std::slice::from_raw_parts(cdata_ptr, cdata_len);
        let cdata = Receive::decode(cdata_buf).unwrap();

        let result = (*rspace).rspace.put_once_durable_concurrent(cdata);

        Box::into_raw(Box::new(result))
    }
}

#[no_mangle]
pub extern "C" fn space_print(rspace: *mut Space, channel: *const c_char) -> () {
    unsafe {
        let channel_str = CStr::from_ptr(channel).to_str().unwrap();
        (*rspace).rspace.print_store(channel_str)
    }
}

#[no_mangle]
pub extern "C" fn space_clear(rspace: *mut Space) -> () {
    unsafe {
        (*rspace).rspace.clear_store();
    }
}

/*
#[no_mangle]
pub extern "C" fn space_put_once_durable_concurrent(
    // raw pointers. c_char represents C string type
    rspace: *mut Space<c_char, c_char>,
    channels: *const Vec<c_char>,
    patterns: *const Vec<Pattern<c_char>>,
    continuation: *const c_char,
) -> *mut Option<Vec<OptionResult<c_char, c_char>>> {
    unsafe {
        // dereference pointers
        // TODO: channel should be type String not &str
        let channels_values: Vec<&str> = (*channels)
            .iter()
            .map(|ptr| CStr::from_ptr(ptr).to_str().unwrap())
            .collect();
        let patterns_values = (&*patterns).to_vec();
        let continuation_value = *continuation;

        let result = (*rspace).rspace.put_once_durable_concurrent(
            channels_values,
            patterns_values,
            continuation_value,
        );

        Box::into_raw(Box::new(result))
    }
}
*/
