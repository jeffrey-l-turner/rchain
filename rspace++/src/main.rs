use std::error::Error;
use std::io::{self, Write};

// use crate::rspace::{Option, RSpace};

use example::{Address, Channel, CityMatch, Entry, Name, Printer};
use rspace::{OptionResult, RSpace};

mod example;
mod rspace;

// fn run_k(k: OptionResult<K) {
//     println!("\nRunning continuation for {}...", k.data.name.first);

//     let r#struct = k.continuation;
//     r#struct.print_entry(&k.data);
// }

fn main() -> Result<(), Box<dyn Error>> {
    let chan1 = Channel {
        name: String::from("friends"),
    };

    let chan2 = Channel {
        name: String::from("colleagues"),
    };

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

    let carol = Entry {
        name: Name {
            first: "Carol".to_string(),
            last: "Lahblah".to_string(),
        },
        address: Address {
            street: "22 Goldwater Way".to_string(),
            city: "Herbert".to_string(),
            state: "Nevada".to_string(),
            zip: "334433".to_string(),
        },
        email: "carol@blablah.org".to_string(),
        phone: "232-555-1212".to_string(),
    };

    fn city_match(entry: &Entry) -> bool {
        entry.address.city == "Crystal Lake"
    }

    fn city_match2(entry: &Entry) -> bool {
        entry.address.city == "Herbert"
    }

    let rspace = RSpace::create().unwrap();

    let _cres1 = rspace.consume(&chan1, city_match, Printer);
    let _cres2 = rspace.consume(&chan1, city_match2, Printer);

    let _ = rspace.print::<CityMatch, Printer>();

    let pres1 = rspace.produce::<Entry, Printer>(&chan1, alice);

    let _ = rspace.print::<CityMatch, Printer>();

    // if pres1.is_some() {
    //     run_k(pres1.unwrap());
    // }

    // let _ = rspace.print();

    let _ = rspace.clear();

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
