use heed::{types::*, Env};
use heed::{Database, EnvOpenOptions};
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::example::{Channel, Entry, Printer};

pub struct Option {
    pub continuation: Printer,
    pub data: Entry,
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

        // open the default unamed database
        let db = env.create_database(None)?;

        println!("\nCreated new database: \"rspace\" in \"target\" directory\n");

        Ok(RSpace { env, db })
    }

    pub fn consume(&self, channel: &Channel, printer: Printer) -> Result<(), Box<dyn Error>> {
        // opening a write transaction
        let mut wtxn = self.env.write_txn()?;
        let _ = self.db.put(&mut wtxn, &channel.name, &printer);
        wtxn.commit()?;

        println!("Installed continuation at channel: \"friends\" with struct: \"Printer\"");

        Ok(())
    }

    pub fn produce(&self, channel: &Channel, entry: Entry) -> Result<Option, Box<dyn Error>> {
        // opening a read transaction
        let rtxn = self.env.read_txn()?;
        let ret = self.db.get(&rtxn, &channel.name)?;

        println!("\nReceived value: \"Printer\" for channel: \"friends\"\n");

        Ok(Option {
            continuation: ret.unwrap(),
            data: entry,
        })
    }
}
