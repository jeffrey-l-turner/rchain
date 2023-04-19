pub mod diskconc;
pub mod diskseq;
pub mod example;
pub mod memconc;
pub mod memseq;
pub mod rspace;
pub mod shared;

use std::error::Error;

use example::{Address, Entry, Name, Printer};
use rspace::RSpace;
// use std::error::Error;

// #[no_mangle]
// #[link(name = "my_library_name")]
// pub fn create<
//     D: Clone + std::hash::Hash + std::fmt::Debug + serde::Serialize + for<'a> serde::Deserialize<'a>,
//     K: Clone
//         + std::hash::Hash
//         + std::fmt::Debug
//         + serde::Serialize
//         + for<'a> serde::Deserialize<'a>
//         + 'static,
// >() -> Result<RSpace<D, K>, Box<dyn Error>> {
//     return RSpace::create();
// }

// #[no_mangle]
// pub extern "C" fn add(x: i32, y: i32) -> i32 {
//     x + y
// }

// #[no_mangle]
// pub extern "C" fn create() -> RSpace<i32, i32> {
//     let rspace: RSpace<i32, i32> = RSpace::create().unwrap();
//     return rspace;
// }

// Example 1
// #[repr(C)]
// pub struct MyStruct {
//     pub a: i32,
//     pub b: i32,
// }

// impl MyStruct {
//     pub fn add(&self) -> i32 {
//         self.a + self.b
//     }
// }

// #[no_mangle]
// pub extern "C" fn create_my_struct(a: i32, b: i32) -> *mut MyStruct {
//     Box::into_raw(Box::new(MyStruct { a, b }))
// }

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
