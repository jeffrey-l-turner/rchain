use heed::{types::*, Env};
use heed::{Database, EnvOpenOptions};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

use crate::example::{Channel, Entry};

pub struct OptionResult<F> {
    pub continuation: F,
    pub data: Entry,
}

type Pattern<T> = fn(T) -> bool;

#[derive(Debug, Hash, Clone, Copy)]
pub struct KData<Pattern, F> {
    pattern: Pattern,
    function: F,
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
        state.serialize_field("function", &self.function)?;
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
            function: F,
        }

        let helper = KDataHelper::<F>::deserialize(deserializer)?;
        let pattern_ptr = usize::from_str_radix(&helper.pattern[2..], 16)
            .map_err(|err| serde::de::Error::custom(format!("Invalid pattern: {}", err)))?;

        Ok(KData {
            pattern: unsafe { std::mem::transmute(pattern_ptr) },
            function: helper.function,
        })
    }
}

/*
See RSpace.scala and Tuplespace.scala in rspace/
*/
pub struct RSpace {
    env: Env,
    db: Database<Str, SerdeBincode<Vec<u8>>>,
}

impl RSpace {
    pub fn create() -> Result<RSpace, Box<dyn Error>> {
        fs::create_dir_all(Path::new("target").join("rspace"))?;
        let env = EnvOpenOptions::new().open(Path::new("target").join("rspace"))?;

        // open the default unamed database
        let db = env.create_database(None)?;

        Ok(RSpace { env, db })
    }

    pub fn consume<T, F: std::hash::Hash + serde::Serialize + 'static>(
        &self,
        channel: &Channel,
        pattern: Pattern<T>,
        function: F,
    ) -> Result<(), Box<dyn Error>> {
        let k_data = KData { pattern, function };
        let k_data_bytes = bincode::serialize(&k_data).unwrap();

        // opening a write transaction
        let mut wtxn = self.env.write_txn()?;

        let kdata_hash = self.calculate_hash(&k_data);
        let key = format!("{}-{}", &channel.name, &kdata_hash);

        let _ = self.db.put(&mut wtxn, &key, &k_data_bytes);
        wtxn.commit()?;

        Ok(())
    }

    // pub fn produce<T, F: for<'a> serde::Deserialize<'a>>(
    //     &self,
    //     channel: &Channel,
    //     entry: Entry,
    // ) -> Option<OptionResult<F>> {
    //     let mut continuation;
    //     let mut matched = false;

    //     let rtxn = self.env.read_txn().unwrap();
    //     let mut iter = self.db.iter(&rtxn).unwrap();
    //     let mut iter_option = iter.next().transpose().unwrap();

    //     while iter_option.is_some() {
    //         let iter_data = iter_option.unwrap();
    //         let k_data_bytes = iter_data.1;
    //         let k_data: KData<Pattern<T>, F> =
    //             bincode::deserialize::<KData<Pattern<T>, F>>(&k_data_bytes).unwrap();
    //         let pattern = k_data.pattern;

    //         if pattern.city_match(&entry) {
    //             let mut wtxn = self.env.write_txn().unwrap();
    //             let _ = self.db.delete(&mut wtxn, iter_data.0);
    //             wtxn.commit().unwrap();

    //             continuation = k_data.function;
    //             matched = true;
    //             break;
    //         }
    //         iter_option = iter.next().transpose().unwrap();
    //     }
    //     drop(iter);
    //     rtxn.commit().unwrap();

    //     if matched {
    //         Some(OptionResult {
    //             continuation,
    //             data: entry,
    //         })
    //     } else {
    //         println!("\nNo matching data for {}...", entry.name.first);
    //         None
    //     }
    // }

    pub fn print<T, F: for<'a> serde::Deserialize<'a> + std::fmt::Debug>(
        &self,
    ) -> Result<(), Box<dyn Error>> {
        let rtxn = self.env.write_txn()?;
        let mut iter = self.db.iter(&rtxn)?;

        if !self.db.is_empty(&rtxn)? {
            println!("\nCurrent store state:");
            let mut iter_option = iter.next().transpose()?;
            while iter_option.is_some() {
                let k_data_bytes = &iter_option.as_ref().unwrap().1;
                let k_data: KData<Pattern<T>, F> =
                    bincode::deserialize::<KData<Pattern<T>, F>>(&k_data_bytes).unwrap();
                println!(
                    "KEY: {:?} VALUE: {:?}",
                    iter_option.as_ref().unwrap().0,
                    k_data
                );
                iter_option = iter.next().transpose()?;
            }
        } else {
            println!("Database is empty")
        }

        drop(iter);
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
