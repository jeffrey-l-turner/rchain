mod datastore;
fn main() {
    println!("hello world!");
    //let mut db = datastore::makeNew();//(String::from("abc"),123);


    let db = datastore::makeNew();
    println!("{}", db.name);
    println!("{}", db.age);
    datastore::add(db.db,String::from("key"), String::from("val"));
    datastore::add(db.db,String::from("key1"), String::from("val1"));
    datastore::add(db.db,String::from("key2"), String::from("val2"));

    // let res = db.add(String::from("key"), String::from("val")).unwrap();
    // //let res2 = db.add(String::from(db.name), String::from(db.age.to_string())).unwrap();
    // let res3 = db.add( String::from("key2"), String::from("val2")).unwrap();
}
