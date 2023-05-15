#[cfg(test)]
mod tests {
    use rspace_plus_plus::diskconc::DiskConcDB;
    use rspace_plus_plus::rtypes::rtypes::{Address, Entry, Name, Receive, Send};

    struct Setup {
        diskconc: DiskConcDB<Send, Receive>,
        city_pattern: String,
        name_pattern: String,
        state_pattern: String,
        alice: Entry,
        bob: Entry,
        carol: Entry,
        dan: Entry,
        erin: Entry,
    }

    impl Setup {
        fn new() -> Self {
            let diskconc = DiskConcDB::<Send, Receive>::create().unwrap();

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
                diskconc,
                city_pattern: String::from("Crystal Lake"),
                name_pattern: String::from("Lahblah"),
                state_pattern: String::from("Idaho"),
                alice,
                bob,
                carol,
                dan,
                erin,
            }
        }
    }

    fn city_match_case(entry: Entry) -> String {
        entry.address.unwrap().city
    }

    fn name_match_case(entry: Entry) -> String {
        entry.name.unwrap().last
    }

    fn state_match_case(entry: Entry) -> String {
        entry.address.unwrap().state
    }

    fn create_send(_channel: String, _data: Entry, _match_case: String) -> Send {
        let mut send = Send::default();
        send.chan = _channel;
        send.data = Some(_data);
        send.match_case = _match_case;
        send
    }

    fn create_receive(
        _channels: Vec<String>,
        _patterns: Vec<String>,
        _continutation: String,
    ) -> Receive {
        let mut receive = Receive::default();
        receive.channels = _channels;
        receive.patterns = _patterns;
        receive.continuation = _continutation;
        receive
    }

    #[test]
    fn diskconc_test_produce_match() {
        let setup = Setup::new();
        let diskconc = setup.diskconc;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let cres = diskconc.consume(receive, false);

        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let pres = diskconc.produce(send, false);

        assert!(cres.is_none());
        assert!(pres.is_some());
        assert!(diskconc.is_empty());

        let _ = diskconc.clear();
    }

    #[test]
    fn diskconc_test_produce_no_match() {
        let setup = Setup::new();
        let diskconc = setup.diskconc;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let cres = diskconc.consume(receive, false);

        let send = create_send(
            String::from("friends"),
            setup.carol.clone(),
            city_match_case(setup.carol),
        );
        let pres = diskconc.produce(send, false);

        assert!(cres.is_none());
        assert!(pres.is_none());
        assert!(!diskconc.is_empty());

        let _ = diskconc.clear();
    }

    #[test]
    fn diskconc_test_consume_match() {
        let setup = Setup::new();
        let diskconc = setup.diskconc;

        let send = create_send(
            String::from("friends"),
            setup.bob.clone(),
            name_match_case(setup.bob),
        );
        let pres = diskconc.produce(send, false);

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.name_pattern],
            String::from("I am the continuation, for now..."),
        );
        let cres = diskconc.consume(receive, false);

        assert!(pres.is_none());
        assert!(cres.is_some());
        assert!(diskconc.is_empty());

        let _ = diskconc.clear();
    }

    #[test]
    fn diskconc_test_multiple_channels_consume_match() {
        let setup = Setup::new();
        let diskconc = setup.diskconc;

        let send1 = create_send(
            String::from("colleagues"),
            setup.dan.clone(),
            state_match_case(setup.dan),
        );
        let pres1 = diskconc.produce(send1, false);

        let send2 = create_send(
            String::from("friends"),
            setup.erin.clone(),
            state_match_case(setup.erin),
        );
        let pres2 = diskconc.produce(send2, false);

        let receive = create_receive(
            vec![String::from("friends"), String::from("colleagues")],
            vec![setup.state_pattern.clone(), setup.state_pattern],
            String::from("I am the continuation, for now..."),
        );
        let cres = diskconc.consume(receive, false);

        assert!(pres1.is_none());
        assert!(pres2.is_none());
        assert!(cres.is_some());
        assert_eq!(cres.unwrap().len(), 2);
        assert!(diskconc.is_empty());

        let _ = diskconc.clear();
    }

    #[test]
    fn diskconc_test_consume_persist() {
        let setup = Setup::new();
        let diskconc = setup.diskconc;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let cres = diskconc.consume(receive, true);

        assert!(cres.is_none());
        assert!(!diskconc.is_empty());

        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let pres = diskconc.produce(send, false);

        assert!(pres.is_some());
        assert!(!diskconc.is_empty());

        let _ = diskconc.clear();
    }

    #[test]
    fn diskconc_test_consume_persist_existing_matches() {
        let setup = Setup::new();
        let diskconc = setup.diskconc;

        let send1 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice.clone()),
        );
        let _pres1 = diskconc.produce(send1, false);

        let send2 = create_send(
            String::from("friends"),
            setup.bob.clone(),
            city_match_case(setup.bob),
        );
        let _pres2 = diskconc.produce(send2, false);

        let receive1 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let cres1 = diskconc.consume(receive1, true);

        assert_eq!(cres1.unwrap().len(), 1);
        assert!(!diskconc.is_empty());

        let receive2 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let cres2 = diskconc.consume(receive2, true);

        assert_eq!(cres2.unwrap().len(), 1);
        assert!(diskconc.is_empty());

        let receive3 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let cres3 = diskconc.consume(receive3, true);

        assert!(cres3.is_none());
        assert!(!diskconc.is_empty());

        let send3 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let pres3 = diskconc.produce(send3, false);

        assert!(pres3.is_some());
        assert!(!diskconc.is_empty());

        let _ = diskconc.clear();
    }

    #[test]
    fn diskconc_test_produce_persist() {
        let setup = Setup::new();
        let diskconc = setup.diskconc;

        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let pres = diskconc.produce(send, true);

        assert!(pres.is_none());
        assert!(!diskconc.is_empty());

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let cres = diskconc.consume(receive, false);

        assert!(cres.is_some());
        assert_eq!(cres.unwrap().len(), 1);
        assert!(!diskconc.is_empty());

        let _ = diskconc.clear();
    }

    #[test]
    fn diskconc_test_produce_persist_existing_matches() {
        let setup = Setup::new();
        let diskconc = setup.diskconc;

        let receive1 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let cres1 = diskconc.consume(receive1, false);

        assert!(cres1.is_none());
        assert!(!diskconc.is_empty());

        let send1 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice.clone()),
        );
        let pres1 = diskconc.produce(send1, true);

        assert!(pres1.is_some());
        assert!((diskconc.is_empty()));

        let send2 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let pres2 = diskconc.produce(send2, true);

        let receive2 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let _cres2 = diskconc.consume(receive2, false);

        assert!(pres2.is_none());
        assert!(!diskconc.is_empty());

        let _ = diskconc.clear();
    }
}
