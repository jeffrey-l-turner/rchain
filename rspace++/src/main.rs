use std::error::Error;
use std::ops::Add;

// use rspace_plus_plus::example::Address;
// use rspace_plus_plus::rtypes::rtypes::{Address, Name};

use crate::diskconc::DiskConcDB;
use crate::diskseq::DiskSeqDB;
use crate::memconc::MemConcDB;
use crate::rtypes::rtypes::{Address, Entry, Name};
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

struct Setup {
    alice: Entry,
    bob: Entry,
    dan: Entry,
}

impl Setup {
    fn new() -> Self {
        // Alice
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

        // Bob
        let mut bob_name = Name::default();
        bob_name.first = "Bob".to_string();
        bob_name.last = "Lahblah".to_string();

        let mut bob_address = Address::default();
        bob_address.street = "1000 Main St".to_string();
        bob_address.city = "Crystal Lake".to_string();
        bob_address.state = "Idaho".to_string();
        bob_address.zip = "223322".to_string();

        let mut bob = Entry::default();
        bob.name = Some(bob_name);
        bob.address = Some(bob_address);
        bob.email = "blablah@tenex.net".to_string();
        bob.phone = "232-555-1212".to_string();

        // Dan
        let mut dan_name = Name::default();
        dan_name.first = "Dan".to_string();
        dan_name.last = "Walters".to_string();

        let mut dan_address = Address::default();
        dan_address.street = "40 Shady Lane".to_string();
        dan_address.city = "Crystal Lake".to_string();
        dan_address.state = "Idaho".to_string();
        dan_address.zip = "223322".to_string();

        let mut dan = Entry::default();
        dan.name = Some(dan_name);
        dan.address = Some(dan_address);
        dan.email = "deejwalters@sdf.lonestar.org".to_string();
        dan.phone = "444-555-1212".to_string();

        Setup { alice, bob, dan }
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

fn run_k(ks: Vec<OptionResult>) {
    for k in ks {
        println!("\nRunning continuation for {:?}...", k.data.name.unwrap());

        println!("\n{:?}", k.continuation);
    }
}

fn createSend(_channel: String, _data: Entry, _persistent: bool) -> rtypes::rtypes::Send {
    let mut send = rtypes::rtypes::Send::default();
    send.chan = _channel;
    send.data = Some(_data);
    send.persistent = _persistent;
    send
}

fn createReceive(
    _channels: Vec<String>,
    _patterns: Vec<String>,
    _continutation: String,
    _persistent: bool,
) -> rtypes::rtypes::Receive {
    let mut receive = rtypes::rtypes::Receive::default();
    receive.channels = _channels;
    receive.patterns = _patterns;
    receive.continuation = _continutation;
    receive.persistent = _persistent;
    receive
}

fn main() -> Result<(), Box<dyn Error>> {
    // println!("\n*********** IN-MEMORY SEQUENTIAL ***********");
    // do_mem_seq();

    println!("\n*********** IN-MEMORY CONCURRENT ***********");
    do_mem_conc();

    println!("\n*********** ON-DISK SEQUENTIAL ***********");
    do_disk_seq();

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

fn do_disk_seq() {
    let setup = Setup::new();
    let diskseq: DiskSeqDB<Entry, String> = DiskSeqDB::create().unwrap();

    println!("\n**** Example 1 ****");

    let rec1 = createReceive(
        vec![String::from("friends")],
        vec![String::from("Lincoln")],
        String::from("I am the continuation, for now..."),
        false,
    );
    let _cres1 = diskseq.consume(rec1);

    let _ = diskseq.print_channel("friends");

    let send1 = createSend(String::from("friends"), setup.alice.clone(), false);
    let pres1 = diskseq.produce(send1);
    if pres1.is_some() {
        run_k(vec![pres1.unwrap()]);
    }
    let _ = diskseq.print_channel("friends");

    println!("\n**** Example 2 ****");

    let send2 = createSend(String::from("friends"), setup.bob, false);
    let _pres2 = diskseq.produce(send2);

    let _ = diskseq.print_channel("friends");

    let rec2 = createReceive(
        vec![String::from("friends")],
        vec![String::from("Lahblah")],
        String::from("I am the continuation, for now..."),
        false,
    );

    let cres2 = diskseq.consume(rec2);
    if cres2.is_some() {
        run_k(cres2.unwrap());
    }
    let _ = diskseq.print_channel("friends");

    println!("\n**** Example 3 ****");

    let send3 = createSend(String::from("colleagues"), setup.dan, false);
    let _pres3 = diskseq.produce(send3);

    let send4 = createSend(String::from("friends"), setup.alice.clone(), false);
    let _pres4 = diskseq.produce(send4);

    let _ = diskseq.print_channel("friends");

    let rec3 = createReceive(
        vec![String::from("friends"), String::from("colleagues")],
        vec![String::from("Lincoln"), String::from("Walters")],
        String::from("I am the continuation, for now..."),
        true,
    );
    let cres3 = diskseq.consume(rec3);
    if cres3.is_some() {
        run_k(cres3.unwrap());
    }
    let _ = diskseq.print_channel("friends");

    let _ = diskseq.clear();
    assert!(diskseq.is_empty());

    // my_function(&mut diskconc);
}

fn do_disk_conc() {
    let setup = Setup::new();
    let diskconc: DiskConcDB<Entry, String> = DiskConcDB::create().unwrap();

    println!("\n**** Example 1 ****");

    let rec1 = createReceive(
        vec![String::from("friends")],
        vec![String::from("Lincoln")],
        String::from("I am the continuation, for now..."),
        false,
    );
    let _cres1 = diskconc.consume(rec1);

    let _ = diskconc.print_channel("friends");

    let send1 = createSend(String::from("friends"), setup.alice.clone(), false);
    let pres1 = diskconc.produce(send1);
    if pres1.is_some() {
        run_k(vec![pres1.unwrap()]);
    }
    let _ = diskconc.print_channel("friends");

    println!("\n**** Example 2 ****");

    let send2 = createSend(String::from("friends"), setup.bob, false);
    let _pres2 = diskconc.produce(send2);

    let _ = diskconc.print_channel("friends");

    let rec2 = createReceive(
        vec![String::from("friends")],
        vec![String::from("Lahblah")],
        String::from("I am the continuation, for now..."),
        false,
    );

    let cres2 = diskconc.consume(rec2);
    if cres2.is_some() {
        run_k(cres2.unwrap());
    }
    let _ = diskconc.print_channel("friends");

    println!("\n**** Example 3 ****");

    let send3 = createSend(String::from("colleagues"), setup.dan, false);
    let _pres3 = diskconc.produce(send3);

    let send4 = createSend(String::from("friends"), setup.alice.clone(), false);
    let _pres4 = diskconc.produce(send4);

    let _ = diskconc.print_channel("friends");

    let rec3 = createReceive(
        vec![String::from("friends"), String::from("colleagues")],
        vec![String::from("Lincoln"), String::from("Walters")],
        String::from("I am the continuation, for now..."),
        true,
    );
    let cres3 = diskconc.consume(rec3);
    if cres3.is_some() {
        run_k(cres3.unwrap());
    }
    let _ = diskconc.print_channel("friends");

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

fn do_mem_conc() {
    let setup = Setup::new();
    let memconc: MemConcDB<Entry, String> = MemConcDB::create().unwrap();

    println!("\n**** Example 1 ****");

    let rec1 = createReceive(
        vec![String::from("friends")],
        vec![String::from("Lincoln")],
        String::from("I am the continuation, for now..."),
        false,
    );
    let _cres1 = memconc.consume(rec1);

    let _ = memconc.print_channel("friends");

    let send1 = createSend(String::from("friends"), setup.alice.clone(), false);
    let pres1 = memconc.produce(send1);
    if pres1.is_some() {
        run_k(vec![pres1.unwrap()]);
    }
    let _ = memconc.print_channel("friends");

    println!("\n**** Example 2 ****");

    let send2 = createSend(String::from("friends"), setup.bob, false);
    let _pres2 = memconc.produce(send2);

    let _ = memconc.print_channel("friends");

    let rec2 = createReceive(
        vec![String::from("friends")],
        vec![String::from("Lahblah")],
        String::from("I am the continuation, for now..."),
        false,
    );

    let cres2 = memconc.consume(rec2);
    if cres2.is_some() {
        run_k(cres2.unwrap());
    }
    let _ = memconc.print_channel("friends");

    println!("\n**** Example 3 ****");

    let send3 = createSend(String::from("colleagues"), setup.dan, false);
    let _pres3 = memconc.produce(send3);

    let send4 = createSend(String::from("friends"), setup.alice.clone(), false);
    let _pres4 = memconc.produce(send4);

    let _ = memconc.print_channel("friends");

    let rec3 = createReceive(
        vec![String::from("friends"), String::from("colleagues")],
        vec![String::from("Lincoln"), String::from("Walters")],
        String::from("I am the continuation, for now..."),
        true,
    );
    let cres3 = memconc.consume(rec3);
    if cres3.is_some() {
        run_k(cres3.unwrap());
    }
    let _ = memconc.print_channel("friends");

    let _ = memconc.clear();
    assert!(memconc.is_empty());
}

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
