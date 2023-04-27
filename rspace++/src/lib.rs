pub mod diskconc;
pub mod diskseq;
pub mod example;
pub mod memconc;
pub mod memseq;
pub mod rspace;
pub mod shared;
pub mod rtypes;

// use rspace::RSpace;
// use shared::{OptionResult, Pattern};
// use std::ffi::{c_char, CStr};

// pub mod rtypes {
//     include!(concat!(env!("OUT_DIR"), "/firefly.rtypes.rs"));
// }

// #[no_mangle]
// pub extern "C" fn process_strings(input: *const *const c_char, length: usize) {
//     let input_strings: Vec<&str> = (0..length)
//         .map(|i| {
//             let cstr = unsafe { CStr::from_ptr(*input.offset(i as isize)) };
//             cstr.to_str().unwrap()
//         })
//         .collect();

//     // Process the vector of strings
//     for string in input_strings {
//         println!("{}", string);
//     }
// }

// #[repr(C)]
// pub struct Space {
//     rspace: RSpace<rtypes::Entry, String>,
// }

// #[no_mangle]
// pub extern "C" fn space_new() -> *mut Space {
//     Box::into_raw(Box::new(Space {
//         rspace: RSpace::create().unwrap(),
//     }))
// }

// // Verb Set 1
// #[no_mangle]
// pub extern "C" fn space_get_once_durable_concurrent(
//     rspace: *mut Space,
//     channel: *const c_char,
//     entry: *const c_char,
// ) -> *mut Option<OptionResult<String>> {
//     unsafe {
//         let channel_str = CStr::from_ptr(channel).to_str().unwrap();
//         let entry_str = { CStr::from_ptr(entry) }.to_string_lossy().into_owned();

//         let result = (*rspace)
//             .rspace
//             .get_once_durable_concurrent(channel_str, entry_str);

//         Box::into_raw(Box::new(result))
//     }
// }

// // Verb Set 3
// #[no_mangle]
// pub extern "C" fn space_put_once_durable_concurrent(
//     rspace: *mut Space,
//     channels: *const Vec<c_char>,
//     patterns: *const Vec<Pattern<String>>,
//     continuation: *const c_char,
// ) -> *mut Option<Vec<OptionResult<String>>> {
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

// #[no_mangle]
// pub extern "C" fn space_print(rspace: *mut Space, channel: *const c_char) -> () {
//     unsafe {
//         let channel_str = CStr::from_ptr(channel).to_str().unwrap();
//         (*rspace).rspace.print_store(channel_str)
//     }
// }

// #[no_mangle]
// pub extern "C" fn space_clear(rspace: *mut Space) -> () {
//     unsafe {
//         (*rspace).rspace.clear_store();
//     }
// }

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
