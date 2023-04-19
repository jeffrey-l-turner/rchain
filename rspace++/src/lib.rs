pub mod diskconc;
pub mod diskseq;
pub mod example;
pub mod memconc;
pub mod memseq;
pub mod rspace;
pub mod shared;

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
