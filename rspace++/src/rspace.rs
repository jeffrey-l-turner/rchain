use heed::{types::*, Env};
use heed::{Database, EnvOpenOptions};
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::example::{Channel, CityMatch, Entry, Printer};

pub struct Option {
    function: fn(entry: &Entry),
    data: Entry,
}

/*
See RSpace.scala and Tuplespace.scala in rspace/
*/
pub struct RSpace {
    env: Env,
    db: Database<Str, SerdeBincode<Printer>>,
}

impl RSpace {
    pub fn create() -> Result<RSpace, Box<dyn Error>> {
        fs::create_dir_all(Path::new("target").join("rspace"))?;
        let env = EnvOpenOptions::new().open(Path::new("target").join("rspace"))?;

        let db = env.create_database(None)?;

        println!("\nCreated new database: \"rspace\" in \"target\" directory\n");

        Ok(RSpace { env, db })
    }

    pub fn consume(&self, channel: &Channel, printer: Printer) -> Result<(), Box<dyn Error>> {
        let mut wtxn = self.env.write_txn()?;
        let _pres = self.db.put(&mut wtxn, &channel.name, &printer);
        wtxn.commit()?;

        println!("Installed continuation at channel: \"friends\" with struct: \"Printer\"");

        Ok(())
    }

    // pub fn produce(&self, entry: Entry, persist: bool) -> Result<Option, Box<dyn Error>> {
    //     let rtxn = self.env.read_txn()?;
    //     let ret = self.db.get(
    //         &rtxn,
    //         &CityMatch {
    //             city: String::from("Crystal Lake"),
    //         },
    //     )?;

    //     Ok(Option {
    //         function: ret,
    //         data: entry,
    //     })
    // }
}
