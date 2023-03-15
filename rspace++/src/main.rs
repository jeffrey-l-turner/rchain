use std::error::Error;

use std::io::{self, Write};
use heed::types::*;
use heed::{Database, EnvOpenOptions};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(Path::new("target").join("test_db.mdb"))?;
    let env = EnvOpenOptions::new().open(Path::new("target").join("test_db.mdb"))?;

    println!("\n");
    println!("Created new database: \"test_db\"");
    println!("\n");

    // we will open the default unamed database
    let db: Database<Str, OwnedType<i32>> = env.create_database(None)?;

    // opening a write transaction
    let mut wtxn = env.write_txn()?;
    db.put(&mut wtxn, "seven", &7)?;
    db.put(&mut wtxn, "zero", &0)?;
    db.put(&mut wtxn, "five", &5)?;
    db.put(&mut wtxn, "three", &3)?;
    wtxn.commit()?;

    println!("Added key: \"seven\" with value: 7");
    println!("Added key: \"zero\" with value: 0");
    println!("Added key: \"five\" with value: 5");
    println!("Added key: \"three\" with value: 3");
    println!("\n");

    // opening a read transaction
    // to check if those values are now available
    let rtxn = env.read_txn()?;

    let ret = db.get(&rtxn, "zero")?;
    println!("Value for key \"zero\": {:?}", ret);
    assert_eq!(ret, Some(0));

    let ret = db.get(&rtxn, "five")?;
    println!("Value for key \"five\": {:?}", ret);
    assert_eq!(ret, Some(5));

    Ok(())
}


fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // TODO: parse and evaluate the input using the BNF grammar

        println!("{}", input.trim());
    }
}
