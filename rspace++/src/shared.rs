use crate::rtypes::rtypes;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::hash::Hash;

#[derive(Debug, Hash, Clone)]
pub struct OptionResult {
    pub continuation: String,
    pub data: rtypes::Entry,
}

pub type Pattern<D> = fn(D) -> bool;

#[derive(Debug, Hash, Clone, Copy)]
pub struct KData<Pattern, K> {
    pub pattern: Pattern,
    pub continuation: K,
    pub persist: bool,
}

impl<T, F> Serialize for KData<Pattern<T>, F>
where
    F: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("KData", 3)?;
        // Serialize the pattern field as a string representation of the function pointer.
        let pattern_string = format!("{:p}", self.pattern);
        state.serialize_field("pattern", &pattern_string)?;
        state.serialize_field("continuation", &self.continuation)?;
        state.serialize_field("persist", &self.persist)?;
        state.end()
    }
}

impl<'de, T, F> Deserialize<'de> for KData<Pattern<T>, F>
where
    F: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct KDataHelper<F> {
            pattern: String,
            continuation: F,
            persist: bool,
        }

        let helper = KDataHelper::<F>::deserialize(deserializer)?;
        let pattern_ptr = usize::from_str_radix(&helper.pattern[2..], 16)
            .map_err(|err| serde::de::Error::custom(format!("Invalid pattern: {}", err)))?;
        let persist = helper.persist;

        Ok(KData {
            pattern: unsafe { std::mem::transmute(pattern_ptr) },
            continuation: helper.continuation,
            persist,
        })
    }
}

#[derive(Debug, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct ProduceData<D> {
    pub data: D,
    pub persist: bool,
}
