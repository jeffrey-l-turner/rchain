use std::error::Error;
use std::fs;
use std::path::Path;

use heed::types::*;
use heed::{Database, EnvOpenOptions};
use heed::Env;

pub struct FileDatabase {
    pub name: String,
    pub age: u8,
    pub db: Database<Str, Str>,
    //pub env: Env,
}

// impl FileDatabase {
//     pub fn new(name: String, age: u8) -> FileDatabase {

//     let env_path = Path::new("target").join("test.mdb");
//     let _ = fs::remove_dir_all(&env_path);
//     fs::create_dir_all(&env_path);
//     let env = EnvOpenOptions::new()
//         .map_size(10 * 1024 * 1024) // 10MB
//         .max_dbs(3)
//         .open(env_path).unwrap();

//     let x = env.clone();

//         FileDatabase {
//             name,
//             age,
//             db: makeDB().unwrap(),
//             //env: envStored,
//         }
//     }

//     pub fn add(&mut self, key : String, val: String) -> Result<(), Box<dyn Error>> {
//         println!("called `db.add()`");
//         let mut wtxn = self.env.write_txn()?;
//         self.db.put(&mut wtxn, &key, &val);
//         wtxn.commit()?;

//         Ok(())
//     }

//     // pub fn get(&mut self, key : String) -> Result<Option<&str>, Box<dyn Error>> {
//     //     let mut rtxn = self.env.read_txn()?;
//     //     let mut res = self.db.get(&mut rtxn, &key);
//     //     let retVal = res.unwrap()?;

//     //     Ok(retVal);
//     // }
// }

pub fn makeNew() -> FileDatabase {
    println!("called `datastore::new()`");

    let newDb: Database<Str, Str> = makeDB().unwrap();


    return  FileDatabase{
        name: String::from("abc"),
        age: 123,
        db: newDb,
    };
}


pub fn add(db:Database<Str, Str>, key : String, val: String) -> Result<(), Box<dyn Error>> {
    println!("called `datastore::add()`");
    let env_path = Path::new("target").join("test2.mdb");

    fs::create_dir_all(&env_path)?;
    let env = EnvOpenOptions::new().open(env_path)?;
    let mut wtxn = env.write_txn()?;
    db.put(&mut wtxn, &key, &val);
    wtxn.commit()?;
    Ok(())
}

pub fn get(db:Database<Str, Str>, key : String) -> Result<String, Box<dyn Error>> {
    println!("called `datastore::get()`");
    let env_path = Path::new("target").join("test2.mdb");

    fs::create_dir_all(&env_path)?;
    let env = EnvOpenOptions::new().open(env_path)?;
    let mut rtxn = env.read_txn()?;
    let res = db.get(&rtxn, &key)?;
    //let retVal = res.or_else("none");

    Ok(String::from("abc"))
}

pub fn makeDB() -> Result<Database<Str, Str>, Box<dyn Error>> {
    
    let env_path = Path::new("target").join("test2.mdb");
    let _ = fs::remove_dir_all(&env_path);
    fs::create_dir_all(&env_path)?;
    let env = EnvOpenOptions::new()
        .map_size(10 * 1024 * 1024) // 10MB
        .max_dbs(3)
        .open(env_path)?;
    println!("called `makeDB`");
    let first: Database<Str, Str> = env.create_database( Some("first"))?;
    println!("called `makeDB`");


    Ok(first)
}
   