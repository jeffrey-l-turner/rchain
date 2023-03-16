use std::{error::Error, str::Bytes};

pub trait Serialize<A> {
    fn encode(&self, a: &A) -> Bytes;
    fn decode(&self, bytes: Bytes) -> Result<A, Box<dyn Error>>;
}
