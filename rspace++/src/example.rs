// use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::rspace::{Option, RSpace};

pub struct Channel {
    pub name: String,
}

struct Name {
    first: String,
    last: String,
}

struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
}

pub struct Entry {
    name: Name,
    address: Address,
    email: String,
    phone: String,
}

// pub enum Pattern {
//     NameMatch { last: String },
//     CityMatch { city: String },
//     StateMatch { state: String },
// }

pub struct CityMatch {
    city: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Printer;

impl Printer {
    pub fn print_entry(&self, entry: &Entry) -> () {
        let name_str = format!("{}, {}", entry.name.last, entry.name.first);
        let addr_str = format!(
            "{}, {}, {}, {}",
            entry.address.street, entry.address.city, entry.address.state, entry.address.zip
        );

        println!(
            r#"
=== ENTRY ===
name:    {}
address: {}
email:   {}
phone:   {}
"#,
            name_str, addr_str, entry.email, entry.phone
        );
    }
}

fn run_k(k: Option) {
    let r#struct = k.continuation;
    r#struct.print_entry(&k.data);
}

pub fn example_main() {
    let chan = Channel {
        name: String::from("friends"),
    };

    let printer = Printer;

    let alice = Entry {
        name: Name {
            first: "Alice".to_string(),
            last: "Lincoln".to_string(),
        },
        address: Address {
            street: "777 Ford St.".to_string(),
            city: "Crystal Lake".to_string(),
            state: "Idaho".to_string(),
            zip: "223322".to_string(),
        },
        email: "alicel@ringworld.net".to_string(),
        phone: "787-555-1212".to_string(),
    };

    let rspace = RSpace::create().unwrap();

    let _cres = rspace.consume(&chan, printer);

    let pres = rspace.produce(&chan, alice);

    print!("Running continuation...\n");
    run_k(pres.unwrap());
}
