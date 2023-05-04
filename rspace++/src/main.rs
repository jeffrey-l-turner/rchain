use crate::rtypes::rtypes::{Address, Entry, Name, OptionResult};
use std::error::Error;

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
        println!(
            "\nRunning continuation for {:?}...",
            k.data.unwrap().name.unwrap()
        );

        println!("\n{:?}", k.continuation);
    }
}

fn create_send(
    _channel: String,
    _data: Entry,
    _match_case: String,
    _persistent: bool,
) -> rtypes::rtypes::Send {
    let mut send = rtypes::rtypes::Send::default();
    send.chan = _channel;
    send.data = Some(_data);
    send.match_case = _match_case;
    send.persistent = _persistent;
    send
}

fn create_receive(
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
    println!("\nNeed to add example using rspace, not individual dbs");

    Ok(())
}
