// pub fn hello_world() {
//   println!("\nhello world!");
// }

#[no_mangle]
pub extern "C" fn hello_world() {
    println!("Hello, world!");
}
