use std::error::Error;

use crate::diskseq::DiskSeqDB;
use crate::memseq::MemSeqDB;
use crate::shared::{MyTrait, OptionResult};
use example::{Address, Entry, Name, Printer};

mod diskseq;
mod example;
mod memseq;
mod shared;

fn run_k(ks: Vec<OptionResult<Entry, Printer>>) {
    for k in ks {
        println!("\nRunning continuation for {}...", k.data.name.first);

        let r#struct = k.continuation;
        r#struct.print_entry(&k.data);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut memseq: MemSeqDB<Entry, Printer> = MemSeqDB::create().unwrap();
    // let mut rspace: RSpace<Entry, Printer> = RSpace::create().unwrap();

    domemseq(&mut memseq);

    // do_the_thing(&mut memseq);
    // do_the_thing(&mut rspace);

    Ok(())
}

// fn do_the_thing<T>(somedb: &mut T) where T: MyTrait {

//     let alice = Entry {
//         name: Name {
//             first: "Alice".to_string(),
//             last: "Lincoln".to_string(),
//         },
//         address: Address {
//             street: "777 Ford St".to_string(),
//             city: "Crystal Lake".to_string(),
//             state: "Idaho".to_string(),
//             zip: "223322".to_string(),
//         },
//         email: "alicel@ringworld.net".to_string(),
//         phone: "787-555-1212".to_string(),
//     };

//     let bob = Entry {
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
//     };

//     let dan = Entry {
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
//     };

//     fn city_match(entry: Entry) -> bool {
//         entry.address.city == "Crystal Lake"
//     }

//     fn name_match(entry: Entry) -> bool {
//         entry.name.last == "Lahblah"
//     }

//     fn state_match(entry: Entry) -> bool {
//         entry.address.state == "Idaho"
//     }

//     // call methods/functions on T
//     println!("\n**** Example 1 ****");
//     let _cres1 = somedb.consume(vec!["friends"], vec![city_match], Printer, false);
//     let _ = somedb.print_channel("friends");
//     let pres1 = somedb.produce("friends", alice.clone(), false);
//     if pres1.is_some() {
//         run_k(vec![pres1.unwrap()]);
//     }
//     let _ = somedb.print_channel("friends");

//     println!("\n**** Example 2 ****");
//     let _pres2 = somedb.produce("friends", bob, false);
//     let _ = somedb.print_channel("friends");
//     let cres2 = somedb.consume(vec!["friends"], vec![name_match], Printer, false);
//     if cres2.is_some() {
//         run_k(cres2.unwrap());
//     }
//     let _ = somedb.print_channel("friends");

//     println!("\n**** Example 3 ****");
//     let _pres3 = somedb.produce("colleagues", dan, false);
//     let _pres4 = somedb.produce("friends", alice.clone(), false);
//     let _ = somedb.print_channel("friends");
//     let cres3 = somedb.consume(
//         vec!["friends", "colleagues"],
//         vec![state_match, state_match],
//         Printer,
//         true,
//     );
//     if cres3.is_some() {
//         run_k(cres3.unwrap());
//         let _ = somedb.print_channel("friends");
//     }

//     let _ = somedb.clear();
//     assert!(somedb.is_empty());

//     somedb.my_method();
// }

fn domemseq(somedb: &mut MemSeqDB<Entry, Printer>) {
    let alice = Entry {
        name: Name {
            first: "Alice".to_string(),
            last: "Lincoln".to_string(),
        },
        address: Address {
            street: "777 Ford St".to_string(),
            city: "Crystal Lake".to_string(),
            state: "Idaho".to_string(),
            zip: "223322".to_string(),
        },
        email: "alicel@ringworld.net".to_string(),
        phone: "787-555-1212".to_string(),
    };

    let bob = Entry {
        name: Name {
            first: "Bob".to_string(),
            last: "Lahblah".to_string(),
        },
        address: Address {
            street: "1000 Main St".to_string(),
            city: "Crystal Lake".to_string(),
            state: "Idaho".to_string(),
            zip: "223322".to_string(),
        },
        email: "blablah@tenex.net".to_string(),
        phone: "232-555-1212".to_string(),
    };

    let dan = Entry {
        name: Name {
            first: "Dan".to_string(),
            last: "Walters".to_string(),
        },
        address: Address {
            street: "40 Shady Lane".to_string(),
            city: "Crystal Lake".to_string(),
            state: "Idaho".to_string(),
            zip: "223322".to_string(),
        },
        email: "deejwalters@sdf.lonestar.org".to_string(),
        phone: "444-555-1212".to_string(),
    };

    fn city_match(entry: Entry) -> bool {
        entry.address.city == "Crystal Lake"
    }

    fn name_match(entry: Entry) -> bool {
        entry.name.last == "Lahblah"
    }

    fn state_match(entry: Entry) -> bool {
        entry.address.state == "Idaho"
    }

    // call methods/functions on T
    println!("\n**** Example 1 ****");
    let _cres1 = somedb.consume(vec!["friends"], vec![city_match], Printer, false);
    let _ = somedb.print_channel("friends");
    let pres1 = somedb.produce("friends", alice.clone(), false);
    if pres1.is_some() {
        run_k(vec![pres1.unwrap()]);
    }
    let _ = somedb.print_channel("friends");

    println!("\n**** Example 2 ****");
    let _pres2 = somedb.produce("friends", bob, false);
    let _ = somedb.print_channel("friends");
    let cres2 = somedb.consume(vec!["friends"], vec![name_match], Printer, false);
    if cres2.is_some() {
        run_k(cres2.unwrap());
    }
    let _ = somedb.print_channel("friends");

    println!("\n**** Example 3 ****");
    let _pres3 = somedb.produce("colleagues", dan, false);
    let _pres4 = somedb.produce("friends", alice.clone(), false);
    let _ = somedb.print_channel("friends");
    let cres3 = somedb.consume(
        vec!["friends", "colleagues"],
        vec![state_match, state_match],
        Printer,
        true,
    );
    if cres3.is_some() {
        run_k(cres3.unwrap());
        let _ = somedb.print_channel("friends");
    }

    let _ = somedb.clear();
    assert!(somedb.is_empty());

    //somedb.my_method();
}

// fn repl() {
//     loop {
//         print!("> ");
//         io::stdout().flush().unwrap();

//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();

//         // TODO: parse and evaluate the input using the BNF grammar

//         println!("{}", input.trim());
//     }
// }
