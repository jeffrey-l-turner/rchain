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
use rtypes::rtypes::{OptionResult, Receive, Send};
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

#[no_mangle]
pub extern "C" fn space_get_once_non_durable_concurrent(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *mut Option<OptionResult> {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result = (*rspace).rspace.get_once_non_durable_concurrent(pdata);

        Box::into_raw(Box::new(result))
    }
}

#[no_mangle]
pub extern "C" fn space_get_once_durable_sequential(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *mut Option<OptionResult> {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result = (*rspace).rspace.get_once_durable_sequential(pdata);

        Box::into_raw(Box::new(result))
    }
}

#[no_mangle]
pub extern "C" fn space_get_once_non_durable_sequential(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *mut Option<OptionResult> {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result = (*rspace).rspace.get_once_non_durable_sequential(pdata);

        Box::into_raw(Box::new(result))
    }
}

// Verb Set 2
#[no_mangle]
pub extern "C" fn space_get_always_durable_concurrent(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *mut Option<OptionResult> {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result = (*rspace).rspace.get_always_durable_concurrent(pdata);

        Box::into_raw(Box::new(result))
    }
}

#[no_mangle]
pub extern "C" fn space_get_always_non_durable_concurrent(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *mut Option<OptionResult> {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result = (*rspace).rspace.get_always_non_durable_concurrent(pdata);

        Box::into_raw(Box::new(result))
    }
}

#[no_mangle]
pub extern "C" fn space_get_always_durable_sequential(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *mut Option<OptionResult> {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result = (*rspace).rspace.get_always_durable_sequential(pdata);

        Box::into_raw(Box::new(result))
    }
}

#[no_mangle]
pub extern "C" fn space_get_always_non_durable_sequential(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *mut Option<OptionResult> {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result = (*rspace).rspace.get_always_non_durable_sequential(pdata);

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
pub extern "C" fn space_put_once_non_durable_concurrent(
    rspace: *mut Space,
    cdata_ptr: *const u8,
    cdata_len: usize,
) -> *mut Option<Vec<OptionResult>> {
    unsafe {
        let cdata_buf = std::slice::from_raw_parts(cdata_ptr, cdata_len);
        let cdata = Receive::decode(cdata_buf).unwrap();

        let result = (*rspace).rspace.put_once_non_durable_concurrent(cdata);

        Box::into_raw(Box::new(result))
    }
}

#[no_mangle]
pub extern "C" fn space_put_once_durable_sequential(
    rspace: *mut Space,
    cdata_ptr: *const u8,
    cdata_len: usize,
) -> *mut Option<Vec<OptionResult>> {
    unsafe {
        let cdata_buf = std::slice::from_raw_parts(cdata_ptr, cdata_len);
        let cdata = Receive::decode(cdata_buf).unwrap();

        let result = (*rspace).rspace.put_once_durable_sequential(cdata);

        Box::into_raw(Box::new(result))
    }
}

#[no_mangle]
pub extern "C" fn space_put_once_non_durable_sequential(
    rspace: *mut Space,
    cdata_ptr: *const u8,
    cdata_len: usize,
) -> *mut Option<Vec<OptionResult>> {
    unsafe {
        let cdata_buf = std::slice::from_raw_parts(cdata_ptr, cdata_len);
        let cdata = Receive::decode(cdata_buf).unwrap();

        let result = (*rspace).rspace.put_once_non_durable_sequential(cdata);

        Box::into_raw(Box::new(result))
    }
}

// Verb Set 4
#[no_mangle]
pub extern "C" fn space_put_always_durable_concurrent(
    rspace: *mut Space,
    cdata_ptr: *const u8,
    cdata_len: usize,
) -> *mut Option<Vec<OptionResult>> {
    unsafe {
        let cdata_buf = std::slice::from_raw_parts(cdata_ptr, cdata_len);
        let cdata = Receive::decode(cdata_buf).unwrap();

        let result = (*rspace).rspace.put_always_durable_concurrent(cdata);

        Box::into_raw(Box::new(result))
    }
}

#[no_mangle]
pub extern "C" fn space_put_always_non_durable_concurrent(
    rspace: *mut Space,
    cdata_ptr: *const u8,
    cdata_len: usize,
) -> *mut Option<Vec<OptionResult>> {
    unsafe {
        let cdata_buf = std::slice::from_raw_parts(cdata_ptr, cdata_len);
        let cdata = Receive::decode(cdata_buf).unwrap();

        let result = (*rspace).rspace.put_always_non_durable_concurrent(cdata);

        Box::into_raw(Box::new(result))
    }
}

#[no_mangle]
pub extern "C" fn space_put_always_durable_sequential(
    rspace: *mut Space,
    cdata_ptr: *const u8,
    cdata_len: usize,
) -> *mut Option<Vec<OptionResult>> {
    unsafe {
        let cdata_buf = std::slice::from_raw_parts(cdata_ptr, cdata_len);
        let cdata = Receive::decode(cdata_buf).unwrap();

        let result = (*rspace).rspace.put_always_durable_sequential(cdata);

        Box::into_raw(Box::new(result))
    }
}

#[no_mangle]
pub extern "C" fn space_put_always_non_durable_sequential(
    rspace: *mut Space,
    cdata_ptr: *const u8,
    cdata_len: usize,
) -> *mut Option<Vec<OptionResult>> {
    unsafe {
        let cdata_buf = std::slice::from_raw_parts(cdata_ptr, cdata_len);
        let cdata = Receive::decode(cdata_buf).unwrap();

        let result = (*rspace).rspace.put_always_non_durable_sequential(cdata);

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
