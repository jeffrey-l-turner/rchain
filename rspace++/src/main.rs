use rspace_plus_plus::{
    rspace::RSpace,
    rtypes::rtypes::{Address, Entry, Name, OptionResult, Receive, Send},
};
use std::error::Error;

mod diskconc;
mod diskseq;
mod example;
mod memconc;
mod memseq;
mod rtypes;
mod shared;

struct Setup {
    rspace: RSpace<Send, Receive>,
    alice: Entry,
    bob: Entry,
    dan: Entry,
}

impl Setup {
    fn new() -> Self {
        let rspace = RSpace::<Send, Receive>::create().unwrap();

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

        Setup {
            rspace,
            alice,
            bob,
            dan,
        }
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

fn create_send(_channel: String, _data: Entry, _match_case: String, _persistent: bool) -> Send {
    let mut send = Send::default();
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
) -> Receive {
    let mut receive = Receive::default();
    receive.channels = _channels;
    receive.patterns = _patterns;
    receive.continuation = _continutation;
    receive.persistent = _persistent;
    receive
}

fn main() -> Result<(), Box<dyn Error>> {
    let setup = Setup::new();
    let rspace = setup.rspace;

    let rec1 = create_receive(
        vec![String::from("friends")],
        vec![String::from("Crystal Lake")],
        String::from("I am the continuation, for now..."),
        false,
    );
    let _cres1 = rspace.put_once_durable_sequential(rec1);

    let _ = rspace.print_store("friends");

    let send1 = create_send(
        String::from("friends"),
        setup.alice.clone(),
        String::from("Crystal Lake"),
        false,
    );
    let pres1 = rspace.get_once_durable_sequential(send1);
    if pres1.is_some() {
        run_k(vec![pres1.unwrap()]);
    }
    let _ = rspace.print_store("friends");

    Ok(())
}
