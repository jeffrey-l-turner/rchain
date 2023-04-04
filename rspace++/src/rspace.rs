use heed::{types::*, Env};
use heed::{Database, EnvOpenOptions};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fs;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::path::Path;

pub struct OptionResult<D, K> {
    pub continuation: K,
    pub data: D,
}

type Pattern<D> = fn(D) -> bool;

#[derive(Debug, Hash, Clone, Copy)]
pub struct KData<Pattern, K> {
    pattern: Pattern,
    continuation: K,
}

impl<T, F> Serialize for KData<Pattern<T>, F>
where
    F: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("KData", 2)?;
        // Serialize the pattern field as a string representation of the function pointer.
        let pattern_string = format!("{:p}", self.pattern);
        state.serialize_field("pattern", &pattern_string)?;
        state.serialize_field("continuation", &self.continuation)?;
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
        }

        let helper = KDataHelper::<F>::deserialize(deserializer)?;
        let pattern_ptr = usize::from_str_radix(&helper.pattern[2..], 16)
            .map_err(|err| serde::de::Error::custom(format!("Invalid pattern: {}", err)))?;

        Ok(KData {
            pattern: unsafe { std::mem::transmute(pattern_ptr) },
            continuation: helper.continuation,
        })
    }
}

/*
See RSpace.scala and Tuplespace.scala in rspace/
*/
pub struct RSpace<D, K> {
    env: Env,
    db: Database<Str, SerdeBincode<Vec<u8>>>,
    phantom: PhantomData<(D, K)>,
}

impl<
        D: Clone
            + std::hash::Hash
            + std::fmt::Debug
            + serde::Serialize
            + for<'a> serde::Deserialize<'a>,
        K: std::hash::Hash
            + std::fmt::Debug
            + serde::Serialize
            + for<'a> serde::Deserialize<'a>
            + 'static,
    > RSpace<D, K>
{
    pub fn create() -> Result<RSpace<D, K>, Box<dyn Error>> {
        fs::create_dir_all(Path::new("target").join("rspace"))?;
        let env = EnvOpenOptions::new().open(Path::new("target").join("rspace"))?;

        // open the default unamed database
        let db = env.create_database(None)?;

        Ok(RSpace {
            env,
            db,
            phantom: PhantomData,
        })
    }

    pub fn consume(
        &self,
        channel: &str,
        pattern: Pattern<D>,
        continuation: K,
    ) -> Option<OptionResult<D, K>> {
        let rtxn = self.env.read_txn().unwrap();

        let data_prefix = format!("channel-{}-data", channel);
        let mut iter_data = self.db.prefix_iter(&rtxn, &data_prefix).unwrap();
        let mut iter_data_option = iter_data.next().transpose().unwrap();

        while iter_data_option.is_some() {
            let iter_data_unwrap = iter_data_option.unwrap();
            let data_bytes = iter_data_unwrap.1;
            let data: D = bincode::deserialize::<D>(&data_bytes).unwrap();

            if pattern(data.clone()) {
                let mut wtxn = self.env.write_txn().unwrap();
                let _ = self.db.delete(&mut wtxn, iter_data_unwrap.0);
                wtxn.commit().unwrap();

                return Some(OptionResult { continuation, data });
            }
            iter_data_option = iter_data.next().transpose().unwrap();
        }
        drop(iter_data);
        rtxn.commit().unwrap();

        let k_data = KData {
            pattern,
            continuation,
        };

        println!("\nNo matching data for {:?}", k_data);

        let k_data_bytes = bincode::serialize(&k_data).unwrap();

        // opening a write transaction
        let mut wtxn = self.env.write_txn().unwrap();

        let kdata_hash = self.calculate_hash(&k_data);
        let key = format!("channel-{}-continuation-{}", &channel, &kdata_hash);

        let _ = self.db.put(&mut wtxn, &key, &k_data_bytes);
        wtxn.commit().unwrap();

        None
    }

    pub fn produce(&self, channel: &str, entry: D) -> Option<OptionResult<D, K>> {
        let rtxn = self.env.read_txn().unwrap();

        let continuation_prefix = format!("channel-{}-continuation", channel);
        let mut iter_continuation = self.db.prefix_iter(&rtxn, &continuation_prefix).unwrap();
        let mut iter_continuation_option = iter_continuation.next().transpose().unwrap();

        while iter_continuation_option.is_some() {
            let iter_data = iter_continuation_option.unwrap();
            let k_data_bytes = iter_data.1;
            let k_data: KData<Pattern<D>, K> =
                bincode::deserialize::<KData<Pattern<D>, K>>(&k_data_bytes).unwrap();
            let pattern = k_data.pattern;

            if pattern(entry.clone()) {
                let mut wtxn = self.env.write_txn().unwrap();
                let _ = self.db.delete(&mut wtxn, iter_data.0);
                wtxn.commit().unwrap();

                return Some(OptionResult {
                    continuation: k_data.continuation,
                    data: entry.clone(),
                });
            }
            iter_continuation_option = iter_continuation.next().transpose().unwrap();
        }
        drop(iter_continuation);
        rtxn.commit().unwrap();

        println!("\nNo matching continuation for {:?}", entry.clone());

        let mut wtxn = self.env.write_txn().unwrap();

        let data_hash = self.calculate_hash(&entry);
        let key = format!("channel-{}-data-{}", &channel, &data_hash);
        let data_bytes = bincode::serialize(&entry).unwrap();

        let _ = self.db.put(&mut wtxn, &key, &data_bytes);
        wtxn.commit().unwrap();

        None
    }

    pub fn print_channel(&self, channel: &str) -> Result<(), Box<dyn Error>> {
        let rtxn = self.env.write_txn()?;

        let continuation_prefix = format!("channel-{}-continuation", channel);
        let mut iter_continuation = self.db.prefix_iter(&rtxn, &continuation_prefix)?;

        let data_prefix = format!("channel-{}-data", channel);
        let mut iter_data = self.db.prefix_iter(&rtxn, &data_prefix)?;

        if !self.db.is_empty(&rtxn)? {
            println!("\nCurrent channel state for \"{}\":", channel);

            let mut iter_continuation_option = iter_continuation.next().transpose()?;
            while iter_continuation_option.is_some() {
                let k_data_bytes = &iter_continuation_option.as_ref().unwrap().1;
                let key = iter_continuation_option.as_ref().unwrap().0;
                let k_data = bincode::deserialize::<KData<Pattern<D>, K>>(&k_data_bytes).unwrap();
                println!("KEY: {:?} VALUE: {:?}", key, k_data);
                iter_continuation_option = iter_continuation.next().transpose()?;
            }

            let mut iter_data_option = iter_data.next().transpose()?;
            while iter_data_option.is_some() {
                let data_bytes = &iter_data_option.as_ref().unwrap().1;
                let key = iter_data_option.as_ref().unwrap().0;
                let data = bincode::deserialize::<D>(&data_bytes).unwrap();
                println!("KEY: {:?} VALUE: {:?}", key, data);
                iter_data_option = iter_data.next().transpose()?;
            }
        } else {
            println!("\nDatabase is empty")
        }

        drop(iter_continuation);
        drop(iter_data);
        rtxn.commit()?;

        Ok(())
    }

    pub fn clear(&self) -> Result<(), Box<dyn Error>> {
        let mut wtxn = self.env.write_txn()?;
        let _ = self.db.clear(&mut wtxn)?;
        wtxn.commit()?;

        Ok(())
    }

    fn calculate_hash<T: Hash>(&self, t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}
