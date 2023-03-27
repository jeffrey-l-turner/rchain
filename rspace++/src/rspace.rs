use heed::{types::*, Env};
use heed::{Database, EnvOpenOptions};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

use crate::example::{Channel, Entry, Printer};

pub struct OptionResult {
    pub continuation: Printer,
    pub data: Entry,
}

#[derive(Debug, Serialize, Deserialize, Hash, Clone, Copy)]
pub struct KData<P, F> {
    pattern: P,
    function: F,
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

    pub fn consume<
        P: std::hash::Hash + serde::Serialize + 'static,
        F: std::hash::Hash + serde::Serialize + 'static,
    >(
        &self,
        channel: &Channel,
        pattern: P,
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

    pub fn produce(&self, channel: &Channel, entry: Entry) -> Option<OptionResult> {
        let mut continuation = Printer;
        let mut matched = false;

        let rtxn = self.env.read_txn().unwrap();
        let mut iter = self.db.iter(&rtxn).unwrap();
        let mut iter_decode = iter.next().transpose().unwrap();

        while iter_decode.is_some() {
            let option = iter_decode.unwrap();
            let k_data = option.1;
            let pattern = k_data.pattern;

            if pattern.city_match(&entry) {
                let mut wtxn = self.env.write_txn().unwrap();
                let _ = self.db.delete(&mut wtxn, option.0);
                wtxn.commit().unwrap();

                continuation = k_data.function;
                matched = true;
                break;
            }
            iter_decode = iter.next().transpose().unwrap();
        }
        drop(iter);
        rtxn.commit().unwrap();

        if matched {
            Some(OptionResult {
                continuation,
                data: entry,
            })
        } else {
            println!("\nNo matching data for {}...", entry.name.first);
            None
        }
    }

    pub fn print<
        P: for<'a> serde::Deserialize<'a> + std::fmt::Debug,
        F: for<'a> serde::Deserialize<'a> + std::fmt::Debug,
    >(
        &self,
    ) -> Result<(), Box<dyn Error>> {
        let rtxn = self.env.write_txn()?;
        let mut iter = self.db.iter(&rtxn)?;

        if !self.db.is_empty(&rtxn)? {
            println!("\nCurrent store state:");
            let mut iter_option = iter.next().transpose()?;
            while iter_option.is_some() {
                let k_data: KData<P, F> =
                    bincode::deserialize::<KData<P, F>>(&iter_option.as_ref().unwrap().1).unwrap();
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
