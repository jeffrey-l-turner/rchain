pub mod diskconc;
pub mod diskseq;
pub mod example;
pub mod memconc;
pub mod memseq;
pub mod rspace;
pub mod shared;

use rspace::RSpace;

// #[no_mangle]
pub extern "C" fn get_rspace<
    D: Clone + std::hash::Hash + std::fmt::Debug + serde::Serialize + for<'a> serde::Deserialize<'a>,
    K: Clone
        + std::hash::Hash
        + std::fmt::Debug
        + serde::Serialize
        + for<'a> serde::Deserialize<'a>
        + 'static,
>() -> RSpace<D, K> {
    return RSpace::create().unwrap();
}
