use heed::{types::*, Env};
use heed::{Database, EnvOpenOptions};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

use crate::example::{Channel, CityMatch, Entry, Printer};

pub struct OptionResult {
    pub continuation: Printer,
    pub data: Entry,
}

#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct KData {
    pattern: CityMatch,
    function: Printer,
}

/*
See RSpace.scala and Tuplespace.scala in rspace/
*/
pub struct RSpace {
    env: Env,
    db: Database<Str, SerdeBincode<KData>>,
}

impl RSpace {
    pub fn create() -> Result<RSpace, Box<dyn Error>> {
        fs::create_dir_all(Path::new("target").join("rspace"))?;
        let env = EnvOpenOptions::new().open(Path::new("target").join("rspace"))?;

        // open the default unamed database
        let db = env.create_database(None)?;

        Ok(RSpace { env, db })
    }

    pub fn consume(
        &self,
        channel: &Channel,
        pattern: CityMatch,
        function: Printer,
    ) -> Result<(), Box<dyn Error>> {
        let k_data = KData { pattern, function };

        // opening a write transaction
        let mut wtxn = self.env.write_txn()?;

        let kdata_hash = self.calculate_hash(&k_data);
        let key = format!("{}-{}", &channel.name, &kdata_hash);

        let _ = self.db.put(&mut wtxn, &key, &k_data);
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

    pub fn print(&self) -> Result<(), Box<dyn Error>> {
        let wtxn = self.env.write_txn()?;
        let mut iter = self.db.iter(&wtxn)?;

        if !self.db.is_empty(&wtxn)? {
            println!("\nCurrent store state:");
            let mut _iter = iter.next().transpose()?;
            while _iter.is_some() {
                println!("{:?}", _iter);
                _iter = iter.next().transpose()?;
            }
        } else {
            println!("Database is empty")
        }

        drop(iter);
        wtxn.commit()?;

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
