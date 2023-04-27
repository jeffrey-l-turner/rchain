use std::error::Error;
use std::ops::Add;

// use rspace_plus_plus::example::Address;
// use rspace_plus_plus::rtypes::rtypes::{Address, Name};

use crate::diskconc::DiskConcDB;
use crate::rtypes::rtypes::{Address, Entry, Name};
// use crate::diskseq::DiskSeqDB;
// use crate::memconc::MemConcDB;
// use crate::memseq::MemSeqDB;
use crate::shared::OptionResult;
// use example::{Address, Entry, Name, Printer};

mod diskconc;
mod diskseq;
mod example;
mod memconc;
mod memseq;
mod rtypes;
mod shared;

fn run_k(ks: Vec<OptionResult>) {
    for k in ks {
        println!("\nRunning continuation for {:?}...", k.data.name.unwrap());

        println!("\n{:?}", k.continuation);
    }
}

struct Setup {
    alice: Entry,
    // bob: Entry,
    // dan: Entry,
}

impl Setup {
    fn new() -> Self {
        let mut alice_name = Name::default();
        alice_name.first = "Alice".to_string();
        alice_name.last = "Lincoln".to_string();

        let mut alice_address = Address::default();
        alice_address.street = "777 Ford St".to_string();
        alice_address.city = "Crystal Lake".to_string();
        alice_address.state = "Idaho".to_string();
        alice_address.zip = "223322".to_string();

        let mut alice = Entry::default();
        alice.name = Some(alice_name);
        alice.address = Some(alice_address);
        alice.email = "alicel@ringworld.net".to_string();
        alice.phone = "787-555-1212".to_string();

        Setup { alice }

        //     bob: Entry {
        //         name: Name {
        //             first: "Bob".to_string(),
        //             last: "Lahblah".to_string(),
        //         },
        //         address: Address {
        //             street: "1000 Main St".to_string(),
        //             city: "Crystal Lake".to_string(),
        //             state: "Idaho".to_string(),
        //             zip: "223322".to_string(),
        //         },
        //         email: "blablah@tenex.net".to_string(),
        //         phone: "232-555-1212".to_string(),
        //         pos: 1,
        //         pos_str: "1".to_string(),
        //     },

        //     dan: Entry {
        //         name: Name {
        //             first: "Dan".to_string(),
        //             last: "Walters".to_string(),
        //         },
        //         address: Address {
        //             street: "40 Shady Lane".to_string(),
        //             city: "Crystal Lake".to_string(),
        //             state: "Idaho".to_string(),
        //             zip: "223322".to_string(),
        //         },
        //         email: "deejwalters@sdf.lonestar.org".to_string(),
        //         phone: "444-555-1212".to_string(),
        //         pos: 1,
        //         pos_str: "1".to_string(),
        //     },
    }
}

// fn city_match(entry: Entry) -> bool {
//     entry.address.city == "Crystal Lake"
// }

// fn name_match(entry: Entry) -> bool {
//     entry.name.last == "Lahblah"
// }

// fn state_match(entry: Entry) -> bool {
//     entry.address.state == "Idaho"
// }

fn main() -> Result<(), Box<dyn Error>> {
    // println!("\n*********** IN-MEMORY SEQUENTIAL ***********");
    // do_mem_seq();

    // println!("\n*********** IN-MEMORY CONCURRENT ***********");
    // do_mem_conc();

    // println!("\n*********** ON-DISK SEQUENTIAL ***********");
    // do_disk_seq();

    println!("\n*********** ON-DISK CONCURRENT ***********");
    do_disk_conc();

    // let mut diskseq: DiskSeqDB<Entry, Printer> = DiskSeqDB::create().unwrap();
    // let mut memseq: MemSeqDB<Entry, Printer> = MemSeqDB::create().unwrap();
    // let mut memconc: MemConcDB<Entry, Printer> = MemConcDB::create().unwrap();

    // let _ = diskseq.clear();

    // my_function(&mut diskseq);
    // my_function(&mut memseq);
    // my_function(&mut memconc);

    Ok(())
}

// fn do_disk_seq() {
//     let setup = Setup::new();
//     let diskseq: DiskSeqDB<Entry, Printer> = DiskSeqDB::create().unwrap();

//     println!("\n**** Example 1 ****");
//     let _cres1 = diskseq.consume(vec!["friends"], vec![city_match], Printer, false);
//     let _ = diskseq.print_channel("friends");
//     let pres1 = diskseq.produce("friends", setup.alice.clone(), false);
//     if pres1.is_some() {
//         run_k(vec![pres1.unwrap()]);
//     }
//     let _ = diskseq.print_channel("friends");

//     println!("\n**** Example 2 ****");
//     let _pres2 = diskseq.produce("friends", setup.bob, false);
//     let _ = diskseq.print_channel("friends");
//     let cres2 = diskseq.consume(vec!["friends"], vec![name_match], Printer, false);
//     if cres2.is_some() {
//         run_k(cres2.unwrap());
//     }
//     let _ = diskseq.print_channel("friends");

//     println!("\n**** Example 3 ****");
//     let _pres3 = diskseq.produce("colleagues", setup.dan, false);
//     let _pres4 = diskseq.produce("friends", setup.alice.clone(), false);
//     let _ = diskseq.print_channel("friends");
//     let cres3 = diskseq.consume(
//         vec!["friends", "colleagues"],
//         vec![state_match, state_match],
//         Printer,
//         true,
//     );
//     if cres3.is_some() {
//         run_k(cres3.unwrap());
//     }
//     let _ = diskseq.print_channel("friends");

//     let _ = diskseq.clear();
//     assert!(diskseq.is_empty());
//     // my_function(&mut diskseq);
// }

fn do_disk_conc() {
    let setup = Setup::new();
    let diskconc: DiskConcDB<Entry, String> = DiskConcDB::create().unwrap();

    println!("\n**** Example 1 ****");

    let mut rec1 = rtypes::rtypes::Receive::default();
    rec1.channels = vec![String::from("friends")];
    rec1.patterns = vec![String::from("Lincoln")];
    rec1.continuation = String::from("I am the continuation, for now...");
    rec1.persistent = false;
    let _cres1 = diskconc.consume(rec1);

    let _ = diskconc.print_channel("friends");

    let mut send1 = rtypes::rtypes::Send::default();
    send1.chan = String::from("friends");
    send1.data = Some(setup.alice);
    send1.persistent = false;
    let pres1 = diskconc.produce(send1);
    if pres1.is_some() {
        run_k(vec![pres1.unwrap()]);
    }
    let _ = diskconc.print_channel("friends");

    // println!("\n**** Example 2 ****");
    // let _pres2 = diskconc.produce("friends", setup.bob, false);
    // let _ = diskconc.print_channel("friends");
    // let cres2 = diskconc.consume(vec!["friends"], vec![name_match], Printer, false);
    // // if cres2.is_some() {
    // //     run_k(cres2.unwrap());
    // // }
    // let _ = diskconc.print_channel("friends");

    // println!("\n**** Example 3 ****");
    // let _pres3 = diskconc.produce("colleagues", setup.dan, false);
    // let _pres4 = diskconc.produce("friends", setup.alice.clone(), false);
    // let _ = diskconc.print_channel("friends");
    // let cres3 = diskconc.consume(
    //     vec!["friends", "colleagues"],
    //     vec![state_match, state_match],
    //     Printer,
    //     true,
    // );
    // if cres3.is_some() {
    //     run_k(cres3.unwrap());
    // }
    // let _ = diskconc.print_channel("friends");

    let _ = diskconc.clear();
    assert!(diskconc.is_empty());
    // my_function(&mut diskconc);
}

// fn do_mem_seq() {
//     let setup = Setup::new();
//     let memseq: MemSeqDB<Entry, Printer> = MemSeqDB::create().unwrap();

//     // call methods/functions on T
//     println!("\n**** Example 1 ****");
//     let _cres1 = memseq.consume(vec!["friends"], vec![city_match], Printer, false);
//     let _ = memseq.print_channel("friends");
//     let pres1 = memseq.produce("friends", setup.alice.clone(), false);
//     if pres1.is_some() {
//         run_k(vec![pres1.unwrap()]);
//     }
//     let _ = memseq.print_channel("friends");

//     println!("\n**** Example 2 ****");
//     let _pres2 = memseq.produce("friends", setup.bob, false);
//     let _ = memseq.print_channel("friends");
//     let cres2 = memseq.consume(vec!["friends"], vec![name_match], Printer, false);
//     if cres2.is_some() {
//         run_k(cres2.unwrap());
//     }
//     let _ = memseq.print_channel("friends");

//     println!("\n**** Example 3 ****");
//     let _pres3 = memseq.produce("colleagues", setup.dan, false);
//     let _pres4 = memseq.produce("friends", setup.alice.clone(), false);
//     let _ = memseq.print_channel("friends");
//     let cres3 = memseq.consume(
//         vec!["friends", "colleagues"],
//         vec![state_match, state_match],
//         Printer,
//         true,
//     );
//     if cres3.is_some() {
//         run_k(cres3.unwrap());
//     }
//     let _ = memseq.print_channel("friends");

//     let _ = memseq.clear();
//     assert!(memseq.is_empty());
//     // my_function(&mut memseq);
// }

// fn do_mem_conc() {
//     let setup = Setup::new();
//     let memconc: MemConcDB<Entry, Printer> = MemConcDB::create().unwrap();

//     println!("\n**** Example 1 ****");
//     let _cres1 = memconc.consume(vec!["friends"], vec![city_match], Printer, false);
//     let _ = memconc.print_channel("friends");
//     let pres1 = memconc.produce("friends", setup.alice.clone(), false);
//     if pres1.is_some() {
//         run_k(vec![pres1.unwrap()]);
//     }
//     let _ = memconc.print_channel("friends");

//     println!("\n**** Example 2 ****");
//     let _pres2 = memconc.produce("friends", setup.bob, false);
//     let _ = memconc.print_channel("friends");
//     let cres2 = memconc.consume(vec!["friends"], vec![name_match], Printer, false);
//     if cres2.is_some() {
//         run_k(cres2.unwrap());
//     }
//     let _ = memconc.print_channel("friends");

//     println!("\n**** Example 3 ****");
//     let _pres3 = memconc.produce("colleagues", setup.dan, false);
//     let _pres4 = memconc.produce("friends", setup.alice.clone(), false);
//     let _ = memconc.print_channel("friends");
//     let cres3 = memconc.consume(
//         vec!["friends", "colleagues"],
//         vec![state_match, state_match],
//         Printer,
//         true,
//     );
//     if cres3.is_some() {
//         run_k(cres3.unwrap());
//     }
//     let _ = memconc.print_channel("friends");

//     let _ = memconc.clear();
//     assert!(memconc.is_empty());
//     // my_function(&mut memconc);
// }

// fn do_some_db<D, K, T>(somedb: &mut T) where T: MyTrait<D, K> {
//     let setup = Setup::new();

//     // call methods/functions on T
//     println!("\n**** Example 1 ****");
//     let _cres1 = somedb.consume(vec!["friends"], vec![city_match], Printer, false);
//     let _ = somedb.print_channel("friends");
//     let pres1 = somedb.produce("friends", setup.alice.clone(), false);
//     if pres1.is_some() {
//         run_k(vec![pres1.unwrap()]);
//     }
//     let _ = somedb.print_channel("friends");

//     println!("\n**** Example 2 ****");
//     let _pres2 = somedb.produce("friends", setup.bob, false);
//     let _ = somedb.print_channel("friends");
//     let cres2 = somedb.consume(vec!["friends"], vec![name_match], Printer, false);
//     if cres2.is_some() {
//         run_k(cres2.unwrap());
//     }
//     let _ = somedb.print_channel("friends");

//     println!("\n**** Example 3 ****");
//     let _pres3 = somedb.produce("colleagues", setup.dan, false);
//     let _pres4 = somedb.produce("friends", setup.alice.clone(), false);
//     let _ = somedb.print_channel("friends");
//     let cres3 = somedb.consume(
//         vec!["friends", "colleagues"],
//         vec![state_match, state_match],
//         Printer,
//         true,
//     );
//     if cres3.is_some() {
//         run_k(cres3.unwrap());
//     }
//     let _ = somedb.print_channel("friends");

//     let _ = somedb.clear();
//     assert!(somedb.is_empty());
// }

// fn my_function<D, K, T>(data: &mut T)
// where
//     T: MyTrait<D, K>,
// {
//     data.my_method();
//     let _ = data.print_channel("friends");
//     let _ = data.clear();
//     assert!(data.is_empty());
// }
