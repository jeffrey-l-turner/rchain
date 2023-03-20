use heed::{types::*, Env};
use heed::{Database, EnvOpenOptions};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::example::{Channel, CityMatch, Entry, Printer};

pub struct Option {
    pub continuation: Printer,
    pub data: Entry,
}

#[derive(Debug, Serialize, Deserialize)]
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
        let _ = self.db.put(&mut wtxn, &channel.name, &k_data);
        wtxn.commit()?;

        println!("Installed continuation in channel: \"{}\" with function: \"print_entry\" and matching pattern: \"CityMatch\"", channel.name);

        Ok(())
    }

    pub fn produce(&self, channel: &Channel, entry: Entry) -> Result<Option, Box<dyn Error>> {
        // opening a read transaction
        let rtxn = self.env.read_txn()?;
        let ret = self.db.get(&rtxn, &channel.name)?;

        let k_data = ret.unwrap();
        let pattern = k_data.pattern;

        if pattern.city_match(&entry) {
            println!(
                "\nFound matching data: \"{}\" in channel: \"{}\"\n",
                entry.name.first, channel.name
            );
            Ok(Option {
                continuation: k_data.function,
                data: entry,
            })
        } else {
            println!("\nNo matching data for {}\n", entry.name.first);
            Err("error: did not find match".into())
        }
    }
}
