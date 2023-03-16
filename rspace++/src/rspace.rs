use std::error::Error;

use crate::example::{Channel, Entry, Pattern};

pub struct Option {
    function: fn(entry: &Entry),
    data: Entry,
}

/*
See RSpace.scala and Tuplespace.scala in rspace/
*/
pub trait RSpace {
    fn create(
        &self,
        channel: &Channel,
        pattern: &Pattern,
        entry: Entry,
        function: fn(entry: &Entry),
    ) -> Result<(), Box<dyn Error>>;

    fn consume(
        &self,
        channel: &Channel,
        r#match: &Pattern,
        function: fn(entry: &Entry),
        persist: bool,
    ) -> Result<(), Box<dyn Error>>;

    fn produce(
        &self,
        channel: &Channel,
        entry: Entry,
        persist: bool,
    ) -> Result<Option, Box<dyn Error>>;
}
