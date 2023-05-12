pub mod diskconc;
pub mod diskseq;
pub mod memconc;
pub mod memseq;
pub mod rspace;
pub mod rtypes;

use prost::Message;
use rspace::RSpace;
use rtypes::rtypes::{OptionResult, Receive, Send};
use serde_json;
use std::ffi::{c_char, CStr, CString};

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
) -> *const c_char {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result_option = (*rspace).rspace.get_once_durable_concurrent(pdata);

        if result_option.is_some() {
            let result = result_option.unwrap();
            let result_string = serde_json::to_string(&result).unwrap();
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        } else {
            let result_string = "";
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        }

        // let array = vec!["Hello", "World"];

        // let ptrs: Vec<_> = array
        //     .iter()
        //     .map(|s| CString::new(*s).unwrap().into_raw())
        //     .collect();

        // let boxed_ptrs = ptrs.into_boxed_slice();

        // Box::into_raw(boxed_ptrs) as *const *const c_char

        // &result as *const OptionResult // return type *const OptionResult for Rust. ?? for Scala

        // Box::into_raw(Box::new(result)) // return type *mut Option<OptionResult> for Rust. ?? for Scala

        // 40 as *const i32 // return type *const i32 for Rust. Int for Scala

        // let hello_string = "Hello from Rust!".to_string();
        // let c_string = std::ffi::CString::new(hello_string).expect("Failed to create CString");
        // c_string.into_raw() // return type *const c_char for Rust. String for Scala

        // hello_string.as_ptr() as *const u8 // return type *const u8 for Rust. Pointer, Unit, or Long for Scala
    }
}

#[no_mangle]
pub extern "C" fn space_get_once_non_durable_concurrent(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *const c_char {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result_option = (*rspace).rspace.get_once_non_durable_concurrent(pdata);

        if result_option.is_some() {
            let result = result_option.unwrap();
            let result_string = serde_json::to_string(&result).unwrap();
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        } else {
            let result_string = "";
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        }
    }
}

#[no_mangle]
pub extern "C" fn space_get_once_durable_sequential(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *const c_char {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result_option = (*rspace).rspace.get_once_durable_sequential(pdata);

        if result_option.is_some() {
            let result = result_option.unwrap();
            let result_string = serde_json::to_string(&result).unwrap();
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        } else {
            let result_string = "";
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        }
    }
}

#[no_mangle]
pub extern "C" fn space_get_once_non_durable_sequential(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *const c_char {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result_option = (*rspace).rspace.get_once_non_durable_sequential(pdata);

        if result_option.is_some() {
            let result = result_option.unwrap();
            let result_string = serde_json::to_string(&result).unwrap();
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        } else {
            let result_string = "";
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        }
    }
}

// Verb Set 2
#[no_mangle]
pub extern "C" fn space_get_always_durable_concurrent(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *const c_char {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result_option = (*rspace).rspace.get_always_durable_concurrent(pdata);

        if result_option.is_some() {
            let result = result_option.unwrap();
            let result_string = serde_json::to_string(&result).unwrap();
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        } else {
            let result_string = "";
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        }
    }
}

#[no_mangle]
pub extern "C" fn space_get_always_non_durable_concurrent(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *const c_char {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result_option = (*rspace).rspace.get_always_non_durable_concurrent(pdata);

        if result_option.is_some() {
            let result = result_option.unwrap();
            let result_string = serde_json::to_string(&result).unwrap();
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        } else {
            let result_string = "";
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        }
    }
}

#[no_mangle]
pub extern "C" fn space_get_always_durable_sequential(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *const c_char {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result_option = (*rspace).rspace.get_always_durable_sequential(pdata);

        if result_option.is_some() {
            let result = result_option.unwrap();
            let result_string = serde_json::to_string(&result).unwrap();
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        } else {
            let result_string = "";
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        }
    }
}

#[no_mangle]
pub extern "C" fn space_get_always_non_durable_sequential(
    rspace: *mut Space,
    pdata_ptr: *const u8,
    pdata_len: usize,
) -> *const c_char {
    unsafe {
        let pdata_buf = std::slice::from_raw_parts(pdata_ptr, pdata_len);
        let pdata = Send::decode(pdata_buf).unwrap();

        let result_option = (*rspace).rspace.get_always_non_durable_sequential(pdata);

        if result_option.is_some() {
            let result = result_option.unwrap();
            let result_string = serde_json::to_string(&result).unwrap();
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        } else {
            let result_string = "";
            let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
            c_string.into_raw()
        }
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

        let result_option = (*rspace).rspace.put_once_durable_concurrent(cdata);

        // if result_option.is_some() {
        //     let result = result_option.unwrap();
        //     let result_string = serde_json::to_string(&result).unwrap();
        //     // let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
        //     // c_string.into_raw()
        // } else {
        //     let result_string = "";
        //     let c_string = std::ffi::CString::new(result_string).expect("Failed to create CString");
        //     c_string.into_raw()
        // }

        // let array = vec!["Hello", "World"];

        // let ptrs: Vec<_> = array
        //     .iter()
        //     .map(|s| CString::new(*s).unwrap().into_raw())
        //     .collect();

        // let boxed_ptrs = ptrs.into_boxed_slice();

        // Box::into_raw(boxed_ptrs) as *const *const c_char

        Box::into_raw(Box::new(result_option))
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
pub extern "C" fn is_empty(rspace: *mut Space) -> bool {
    unsafe { (*rspace).rspace.is_empty() }
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
