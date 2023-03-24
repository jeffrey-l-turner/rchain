use heed::{types::*, Env};
use heed::{Database, EnvOpenOptions};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

use crate::example::{Channel, CityMatch, Entry, Printer};

pub struct Option {
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

        println!("\nCreated new database: \"rspace\" in \"target\" directory\n");

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

        println!("Installed continuation in channel: \"{}\" with function: \"print_entry\" and matching pattern: \"CityMatch\"", channel.name);

        Ok(())
    }

    pub fn produce(&self, channel: &Channel, entry: Entry) -> Result<Option, Box<dyn Error>> {
        let mut continuation = Printer;
        let mut matched = false;

        let rtxn = self.env.read_txn()?;
        let mut iter = self.db.iter(&rtxn)?;
        let mut iter_decode = iter.next().transpose()?;

        while iter_decode.is_some() {
            let option = iter_decode.unwrap();
            let k_data = option.1;
            let pattern = k_data.pattern;

            if pattern.city_match(&entry) {
                println!(
                    "\nFound matching data: \"{}\" in channel: \"{}\"\n",
                    entry.name.first, channel.name
                );

                let mut wtxn = self.env.write_txn()?;
                let _ = self.db.delete(&mut wtxn, option.0);
                wtxn.commit()?;

                continuation = k_data.function;
                matched = true;
                break;
            }
            iter_decode = iter.next().transpose()?;
        }
        drop(iter);
        rtxn.commit()?;

        if matched {
            Ok(Option {
                continuation,
                data: entry,
            })
        } else {
            println!("\nNo matching data for {}...\n", entry.name.first);
            Err("error: did not find match".into())
        }
    }

    pub fn print(&self) -> Result<(), Box<dyn Error>> {
        let wtxn = self.env.write_txn()?;
        let mut iter = self.db.iter(&wtxn)?;

        println!();

        if !self.db.is_empty(&wtxn)? {
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
