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
    pdata_buf: &[u8],
) -> *mut Option<OptionResult> {
    // let channel_str = CStr::from_ptr(channel).to_str().unwrap();
    // let entry_str = { CStr::from_ptr(entry) }.to_string_lossy().into_owned();

    let pdata = Send::decode(pdata_buf).unwrap();

    let result = (unsafe { *rspace })
        .rspace
        .get_once_durable_concurrent(pdata);

    Box::into_raw(Box::new(result))

    // let pdata = Send::decode(pdata_buf).unwrap();
    // println!("{:?}", pdata)
}

// Verb Set 3
// #[no_mangle]
// pub extern "C" fn space_put_once_durable_concurrent(
//     rspace: *mut Space,
//     channels: *const Vec<c_char>,
//     patterns: *const Vec<Pattern<String>>,
//     continuation: *const c_char,
// ) -> *mut Option<Vec<OptionResult>> {
//     unsafe {
//         let channels_values = (*channels)
//             .iter()
//             .map(|ptr| CStr::from_ptr(ptr).to_str().unwrap())
//             .collect();
//         let patterns_values = (&*patterns).to_vec();
//         let continuation_value = { CStr::from_ptr(continuation) }
//             .to_string_lossy()
//             .into_owned();

//         let result = (*rspace).rspace.put_once_durable_concurrent(
//             channels_values,
//             patterns_values,
//             continuation_value,
//         );

//         Box::into_raw(Box::new(result))
//     }
// }

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
