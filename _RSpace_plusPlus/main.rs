use crate::datastore::FileDatabase;

mod datastore;
fn main() {
    println!("hello world!");
    //let mut d0datastore::makeNew();//(String::from("abc"),123);

    let mut db :FileDatabase = FileDatabase::new(String::from("test db"), 123);

    db.add(String::from("abc"), String::from("123"));
    db.add(String::from("def"), String::from("321"));
    db.add(String::from("xyz"), String::from("333"));

    {
        let result1 = db.get(String::from("abc")).unwrap();
        println!("result1: {:?}", result1);
    }
    {
        let result2 = db.get(String::from("def")).unwrap();
        println!("result2: {:?}", result2);
    }
    {
        let result3 = db.get(String::from("xyz")).unwrap();
        println!("result3: {:?}", result3);
    }
    

    // let db = datastore::makeNew();
    // println!("{}", db.name);
    // println!("{}", db.age);
    // datastore::add(db.3,String::from("key"), String::from("val"));
    // datastore::add(db.3,String::from("key1"), String::from("val1"));
    // datastore::add(db.3,String::from("key2"), String::from("val2"));

    // let res = db.add(String::from("key"), String::from("val")).unwrap();
    // //let res2 = db.add(String::from(db.name), String::from(db.age.to_string())).unwrap();
    // let res3 = db.add( String::from("key2"), String::from("val2")).unwrap();
}
