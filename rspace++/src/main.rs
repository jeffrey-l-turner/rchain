use std::error::Error;
use std::io::{self, Write};

// use crate::rspace::{Option, RSpace};

use example::{Address, Channel, Entry, Name, Printer};
use rspace::{Option, RSpace};

mod example;
mod rspace;

fn run_k(k: Option) {
    let r#struct = k.continuation;
    r#struct.print_entry(&k.data);
}

fn main() -> Result<(), Box<dyn Error>> {
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
