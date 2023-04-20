pub mod diskconc;
pub mod diskseq;
pub mod example;
pub mod memconc;
pub mod memseq;
pub mod rspace;
pub mod shared;

// use example::{Entry, Printer};
use rspace::RSpace;
use shared::OptionResult;
use std::ffi::{c_char, CStr};
// use std::error::Error;

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

// pub fn space_get_once_durable_concurrent(
//     _rspace: Space<Entry, Printer>,
//     _channel: &str,
//     _entry: Entry,
// ) -> Option<OptionResult<Entry, Printer>> {
//     _rspace.rspace.get_once_durable_concurrent(_channel, _entry)
// }

// Example 1
#[repr(C)]
pub struct MyStruct {
    x: i32,
    y: i32,
}

#[no_mangle]
pub extern "C" fn MyStruct_new(x: i32, y: i32) -> *mut MyStruct {
    Box::into_raw(Box::new(MyStruct { x, y }))
}

#[no_mangle]
pub extern "C" fn MyStruct_add(my_struct: *const MyStruct) -> i32 {
    let my_struct = unsafe { &*my_struct };
    my_struct.x + my_struct.y
}

// Example 2
pub fn print_types_inner<T: std::fmt::Debug, U: std::fmt::Debug>(x: T, y: U) {
    println!("Type of x: {:?}", std::any::type_name::<T>());
    println!("Value of x: {:?}", x);
    println!("Type of y: {:?}", std::any::type_name::<U>());
    println!("Value of y: {:?}", y);
}

#[no_mangle]
pub extern "C" fn print_types(x: i32, y: f64) {
    print_types_inner(x, y);
}
