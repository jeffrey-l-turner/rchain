use heed::{types::*, Env};
use heed::{Database, EnvOpenOptions};
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::example::{Channel, CityMatch, Entry};

pub struct Option {
    function: fn(entry: &Entry),
    data: Entry,
}

/*
See RSpace.scala and Tuplespace.scala in rspace/
*/
pub struct RSpace {
    env: Env,
    db: Database<CityMatch, OwnedType<()>>,
}

impl RSpace {
    pub fn new(channel: &Channel) -> Result<RSpace, Box<dyn Error>> {
        let mdb_name = format!("rspace_{}.mdb", channel.name);
        fs::create_dir_all(Path::new("target").join(&mdb_name))?;
        let env = EnvOpenOptions::new().open(Path::new("target").join(&mdb_name))?;

        println!("\n");
        println!(
            "Created new database: \"{}\" in \"target\" directory",
            mdb_name
        );
        println!("\n");

        let db = env.create_database(None)?;

        Ok(RSpace { env, db })
    }

    pub fn consume(
        &self,
        r#match: &CityMatch,
        function: &(),
        persist: bool,
    ) -> Result<(), Box<dyn Error>> {
        let mut wtxn = self.env.write_txn()?;

        self.db.put(&mut wtxn, r#match, function)?;
        wtxn.commit()?;

        Ok(())
    }

    pub fn produce(&self, entry: Entry, persist: bool) -> Result<Option, Box<dyn Error>> {
        let rtxn = self.env.read_txn()?;
        let ret = self.db.get(
            &rtxn,
            &CityMatch {
                city: String::from("Crystal Lake"),
            },
        )?;

        Ok(Option {
            function: ret,
            data: entry,
        })
    }
}
