#[cfg(test)]
mod tests {
    use rspace_plus_plus::diskseq::DiskSeqDB;
    use rspace_plus_plus::rtypes::rtypes::{Address, Entry, Name, OptionResult, Receive, Send};

    struct Setup {
        diskseq: DiskSeqDB<Send, Receive>,
        alice: Entry,
        bob: Entry,
        carol: Entry,
        dan: Entry,
        erin: Entry,
    }

    impl Setup {
        fn new() -> Self {
            let diskseq = DiskSeqDB::<Send, Receive>::create().unwrap();

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
            bob.phone = "698-555-1212".to_string();

            // Carol
            let mut carol_name = Name::default();
            carol_name.first = "Carol".to_string();
            carol_name.last = "Lahblah".to_string();

            let mut carol_address = Address::default();
            carol_address.street = "22 Goldwater Way".to_string();
            carol_address.city = "Herbert".to_string();
            carol_address.state = "Nevada".to_string();
            carol_address.zip = "334433".to_string();

            let mut carol = Entry::default();
            carol.name = Some(carol_name);
            carol.address = Some(carol_address);
            carol.email = "carol@blablah.org".to_string();
            carol.phone = "232-555-1212".to_string();

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

            // Erin
            let mut erin_name = Name::default();
            erin_name.first = "Erin".to_string();
            erin_name.last = "Rush".to_string();

            let mut erin_address = Address::default();
            erin_address.street = "23 Market St.".to_string();
            erin_address.city = "Peony".to_string();
            erin_address.state = "Idaho".to_string();
            erin_address.zip = "224422".to_string();

            let mut erin = Entry::default();
            erin.name = Some(erin_name);
            erin.address = Some(erin_address);
            erin.email = "erush@lasttraintogoa.net".to_string();
            erin.phone = "333-555-1212".to_string();

            Setup {
                diskseq,
                alice,
                bob,
                carol,
                dan,
                erin,
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

    #[test]
    fn diskseq_test_produce_match() {
        let setup = Setup::new();
        let diskseq = setup.diskseq;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![String::from("Crystal Lake")],
            String::from("I am the continuation, for now..."),
            false,
        );
        let cres = diskseq.consume(receive);

        let send = create_send(
            String::from("friends"),
            setup.alice,
            String::from("Crystal Lake"),
            false,
        );
        let pres = diskseq.produce(send);

        assert!(cres.is_none());
        assert!(pres.is_some());
        assert!(diskseq.is_empty());

        let _ = diskseq.clear();
    }

    // #[test]
    // fn diskseq_test_produce_no_match() {
    //     let setup = Setup::new();
    //     let diskseq = setup.diskseq;

    //     let cres = diskseq.consume(vec!["friends"], vec![city_match], Printer, false);
    //     let pres = diskseq.produce("friends", setup.carol, false);

    //     assert!(cres.is_none());
    //     assert!(pres.is_none());
    //     assert!(!diskseq.is_empty());

    //     let _ = diskseq.clear();
    // }

    // #[test]
    // fn diskseq_test_consume_match() {
    //     let setup = Setup::new();
    //     let diskseq = setup.diskseq;

    //     let pres = diskseq.produce("friends", setup.bob, false);
    //     let cres = diskseq.consume(vec!["friends"], vec![name_match], Printer, false);

    //     assert!(pres.is_none());
    //     assert!(cres.is_some());
    //     assert!(diskseq.is_empty());

    //     let _ = diskseq.clear();
    // }

    // #[test]
    // fn diskseq_test_multiple_channels_consume_match() {
    //     let setup = Setup::new();
    //     let diskseq = setup.diskseq;

    //     let pres1 = diskseq.produce("colleagues", setup.dan, false);
    //     let pres2 = diskseq.produce("friends", setup.erin, false);

    //     let cres = diskseq.consume(
    //         vec!["friends", "colleagues"],
    //         vec![state_match, state_match],
    //         Printer,
    //         false,
    //     );

    //     assert!(pres1.is_none());
    //     assert!(pres2.is_none());
    //     assert!(cres.is_some());
    //     assert_eq!(cres.unwrap().len(), 2);
    //     assert!(diskseq.is_empty());

    //     let _ = diskseq.clear();
    // }

    // #[test]
    // fn diskseq_test_consume_persist() {
    //     let setup = Setup::new();
    //     let diskseq = setup.diskseq;

    //     let cres = diskseq.consume(vec!["friends"], vec![city_match], Printer, true);

    //     assert!(cres.is_none());
    //     assert!(!diskseq.is_empty());

    //     let pres = diskseq.produce("friends", setup.alice.clone(), false);

    //     assert!(pres.is_some());
    //     assert!(!diskseq.is_empty());

    //     let _ = diskseq.clear();
    // }

    // #[test]
    // fn diskseq_test_consume_persist_existing_matches() {
    //     let setup = Setup::new();
    //     let diskseq = setup.diskseq;

    //     let _pres1 = diskseq.produce("friends", setup.alice.clone(), false);
    //     let _pres2 = diskseq.produce("friends", setup.bob, false);
    //     let cres1 = diskseq.consume(vec!["friends"], vec![city_match], Printer, true);

    //     assert_eq!(cres1.unwrap().len(), 1);
    //     assert!(!diskseq.is_empty());

    //     let cres2 = diskseq.consume(vec!["friends"], vec![city_match], Printer, true);

    //     assert_eq!(cres2.unwrap().len(), 1);
    //     assert!(diskseq.is_empty());

    //     let cres3 = diskseq.consume(vec!["friends"], vec![city_match], Printer, true);

    //     assert!(cres3.is_none());
    //     assert!(!diskseq.is_empty());

    //     let pres3 = diskseq.produce("friends", setup.alice.clone(), false);

    //     assert!(pres3.is_some());
    //     assert!(!diskseq.is_empty());

    //     let _ = diskseq.clear();
    // }

    // #[test]
    // fn diskseq_test_produce_persist() {
    //     let setup = Setup::new();
    //     let diskseq = setup.diskseq;

    //     let pres = diskseq.produce("friends", setup.alice, true);

    //     assert!(pres.is_none());
    //     assert!(!diskseq.is_empty());

    //     let cres = diskseq.consume(vec!["friends"], vec![city_match], Printer, false);

    //     assert!(cres.is_some());
    //     assert_eq!(cres.unwrap().len(), 1);
    //     assert!(!diskseq.is_empty());

    //     let _ = diskseq.clear();
    // }

    // #[test]
    // fn diskseq_test_produce_persist_existing_matches() {
    //     let setup = Setup::new();
    //     let diskseq = setup.diskseq;

    //     let cres1 = diskseq.consume(vec!["friends"], vec![city_match], Printer, false);

    //     assert!(cres1.is_none());
    //     assert!(!diskseq.is_empty());

    //     let pres1 = diskseq.produce("friends", setup.alice.clone(), true);

    //     assert!(pres1.is_some());
    //     assert!((diskseq.is_empty()));

    //     let pres2 = diskseq.produce("friends", setup.alice.clone(), true);
    //     let _cres2 = diskseq.consume(vec!["friends"], vec![city_match], Printer, false);

    //     assert!(pres2.is_none());
    //     assert!(!diskseq.is_empty());

    //     let _ = diskseq.clear();
    // }
}
