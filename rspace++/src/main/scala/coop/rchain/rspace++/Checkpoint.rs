use crate::coop::rchain::rspace::hashing::Blake2b256Hash;
use crate::coop::rchain::rspace::trace::Produce;
use std::collections::HashMap;

#[derive(Debug)]
struct SoftCheckpoint<C, P, A, K> {
    cache_snapshot: HotStoreState<C, P, A, K>,
    log: trace::Log,
    produce_counter: HashMap<Produce, usize>,
}

#[derive(Debug)]
struct Checkpoint {
    root: Blake2b256Hash,
    log: trace::Log,
}
