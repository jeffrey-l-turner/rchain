use std::error::Error;

use crate::example::{Channel, Entry};

struct Option {
    function: fn(entry: &Entry),
    data: Entry,
}

pub trait RSpace {
    fn create(
        &self,
        channel: &Channel,
        pattern: &Pattern,
        Entry: Entry,
        function: fn(entry: &Entry),
    ) -> Result<(), Box<dyn Error>>;

    fn consume(
        &self,
        channel: &Channel,
        r#match: &Match,
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
