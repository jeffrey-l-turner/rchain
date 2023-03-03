use std::error::Error;
use std::fs;
use std::path::Path;

use heed::types::*;
use heed::{Database, EnvOpenOptions};
use heed::Env;

pub struct FileDatabase {
    pub name: String,
    pub age: u8,
    pub environment: Env,
    pub db: Database<Str, Str>,
}

fn duplicate<T>(x: T) -> T { x } 

impl FileDatabase {
    pub fn new(name: String, age: u8) -> FileDatabase {

    let env_path = Path::new("target").join(name.to_owned()+".mdb");
    let _ = fs::remove_dir_all(&env_path);
    fs::create_dir_all(&env_path);
    let env= EnvOpenOptions::new()
        .map_size(10 * 1024 * 1024) // 10MB
        .max_dbs(3)
        .open(env_path).unwrap();

    let db = makeDB(&env).unwrap();

        FileDatabase {
            name,
            age,
            environment: env,
            db: db,
            //env: envStored,
        }
    }

    fn makeDB(env: Env) -> Result<Database<Str, Str>, Box<dyn Error>> {
    
        
        println!("called `makeDB`");
        let first: Database<Str, Str> = env.create_database( Some("first"))?;
        println!("called `makeDB`");
    
    
        Ok(first)
    }

    pub fn add(&mut self, key : String, val: String) -> Result<(), Box<dyn Error>> {


        println!("called `db.add()`");
        let mut wtxn = self.environment.write_txn()?;
        self.db.put(&mut wtxn, &key, &val);
        wtxn.commit()?;

        Ok(())
    }

    pub fn get(&mut self, key : String) -> Result<Option<&str>, Box<dyn Error>> {
        //let mut finalVal:Option<&str> = Some("xyz");
        let finalVal:Option<&str> = Some("xyz");
        let mut wtxn = self.environment.write_txn()?;
        {

            let ret = self.db.get::<String, String>(&wtxn, "hello")?;

            let get_result = self.db.get(&mut wtxn, &key)?;
            println!("get_result:\t{:?}", get_result);
            //finalVal = get_result.clone();
            // if get_result.is_none() {
            //     finalVal = None
            // } else {
            //     finalVal = Some(get_result.unwrap());
            // }

            // let x = match get_result {
            //     Ok(val) => Ok(val),
            //     Err(e) => Err(e.into()),
            // }

            let result = match get_result {
                Some(val) => val,//finalVal = Some(val),
                None => "not found!",//finalVal = Some("not found!"),
            };

            wtxn.commit()?;
        }
        Ok(finalVal)

        // match finalVal {
        //     Some() =>
        //     None() =>
        // }

        
    }
}

// pub fn makeNew() -> FileDatabase {
//     println!("called `datastore::new()`");

//     let newDb: Database<Str, Str> = makeDB().unwrap();


//     return  FileDatabase{
//         name: String::from("abc"),
//         age: 123,
//         db: newDb,
//     };
// }


// pub fn add(db:Database<Str, Str>, key : String, val: String) -> Result<(), Box<dyn Error>> {
//     println!("called `datastore::add()`");
//     let env_path = Path::new("target").join("test2.mdb");

//     fs::create_dir_all(&env_path)?;
//     let env = EnvOpenOptions::new().open(env_path)?;
//     let mut wtxn = env.write_txn()?;
//     db.put(&mut wtxn, &key, &val);
//     wtxn.commit()?;
//     Ok(())
// }

// pub fn get(db:Database<Str, Str>, key : String) -> Result<String, Box<dyn Error>> {
//     println!("called `datastore::get()`");
//     let env_path = Path::new("target").join("test2.mdb");

//     fs::create_dir_all(&env_path)?;
//     let env = EnvOpenOptions::new().open(env_path)?;
//     let mut rtxn = env.read_txn()?;
//     let res = db.get(&rtxn, &key)?;
//     //let retVal = res.or_else("none");

//     Ok(String::from("abc"))
// }

fn makeDB(env:&Env) -> Result<Database<Str, Str>, Box<dyn Error>> {
    
    println!("called `makeDB`");
    let first: Database<Str, Str> = env.create_database( Some("first"))?;
    println!("called `makeDB`");


    Ok(first)
}
   