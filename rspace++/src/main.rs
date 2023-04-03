use std::error::Error;
use std::io::{self, Write};

use example::{Address, Channel, Entry, Name, Printer};
use rspace::{OptionResult, RSpace};

mod example;
mod rspace;

fn run_k(k: OptionResult<Entry, Printer>) {
    println!("\nRunning continuation for {}...", k.data.name.first);

    let r#struct = k.continuation;
    r#struct.print_entry(&k.data);
}

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

    fn city_match(entry: Entry) -> bool {
        entry.address.city == "Crystal Lake"
    }

    fn name_match(entry: Entry) -> bool {
        entry.name.last == "Lahblah"
    }

    let rspace: RSpace<Entry, Printer> = RSpace::create().unwrap();

    let cres1 = rspace.consume(&chan1, city_match, Printer);
    let pres1 = rspace.produce(&chan1, carol);

    let _ = rspace.print_channel(&chan1);

    let cres2 = rspace.consume(&chan1, name_match, Printer);

    if cres2.is_some() {
        run_k(cres2.unwrap());
    }

    let _ = rspace.print_channel(&chan1);

    let pres2 = rspace.produce(&chan1, alice);

    if pres2.is_some() {
        run_k(pres2.unwrap());
    }

    let _ = rspace.print_channel(&chan1);

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
