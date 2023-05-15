#[cfg(test)]
mod tests {
    use rspace_plus_plus::rspace::RSpace;
    use rspace_plus_plus::rtypes::rtypes::{Address, Entry, Name, Receive, Send};

    struct Setup {
        rspace: RSpace<Send, Receive>,
        city_pattern: String,
        name_pattern: String,
        state_pattern: String,
        email_pattern: String,
        phone_pattern: String,
        alice: Entry,
        bob: Entry,
        carol: Entry,
        dan: Entry,
        erin: Entry,
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
                rspace,
                city_pattern: String::from("Crystal Lake"),
                name_pattern: String::from("Lahblah"),
                state_pattern: String::from("Idaho"),
                email_pattern: String::from("deejwalters@sdf.lonestar.org"),
                phone_pattern: String::from("333-555-1212"),
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

    //memseq
    #[test]
    fn memseq_test_consume_persist_existing_matches() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send1 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice.clone()),
        );
        let send2 = create_send(
            String::from("friends"),
            setup.bob.clone(),
            city_match_case(setup.bob),
        );
        let send3 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let receive1 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let receive2 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let receive3 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );

        let _pres1 = rspace.get_once_non_durable_sequential(send1);
        let _pres2 = rspace.get_once_non_durable_sequential(send2);
        let cres1 =
            rspace.put_always_non_durable_sequential(receive1);
        assert_eq!(cres1.unwrap().len(), 1);
        assert!(!rspace.is_memseq_empty());

        let cres2 =
            rspace.put_always_non_durable_sequential(receive2);

        assert_eq!(cres2.unwrap().len(), 1);
        assert!(rspace.is_memseq_empty());

        let cres3 =
            rspace.put_always_non_durable_sequential(receive3);

        assert!(cres3.is_none());
        assert!(!rspace.is_memseq_empty());

        let pres3 = rspace.get_once_non_durable_sequential(send3);

        assert!(pres3.is_some());
        assert!(!rspace.is_memseq_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn memseq_test_multiple_channels_consume_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send1 = create_send(
            String::from("colleagues"),
            setup.dan.clone(),
            state_match_case(setup.dan),
        );
        let send2 = create_send(
            String::from("friends"),
            setup.erin.clone(),
            state_match_case(setup.erin),
        );
        let receive = create_receive(
            vec![String::from("friends"), String::from("colleagues")],
            vec![setup.state_pattern.clone(), setup.state_pattern],
            String::from("I am the continuation, for now..."),
        );

        let pres1 = rspace.get_once_non_durable_sequential(send1);
        let pres2 = rspace.get_once_non_durable_sequential(send2);

        let cres = rspace.put_once_non_durable_sequential(receive );

        assert!(pres1.is_none());
        assert!(pres2.is_none());
        assert!(cres.is_some());
        assert_eq!(cres.unwrap().len(), 2);
        assert!(rspace.is_memseq_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn memseq_test_consume_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send = create_send(
            String::from("friends"),
            setup.bob.clone(),
            name_match_case(setup.bob),
        );
        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.name_pattern],
            String::from("I am the continuation, for now..."),
        );

        let pres = rspace.get_once_non_durable_sequential(send);
        let cres = rspace.put_once_non_durable_sequential(receive);

        assert!(pres.is_none());
        assert!(cres.is_some());
        assert!(rspace.is_memseq_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn memseq_test_produce_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let cres = rspace.put_once_non_durable_sequential(receive);
        let pres = rspace.get_once_non_durable_sequential(send);

        assert!(cres.is_none());
        assert!(pres.is_some());
        assert!(rspace.is_memseq_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn memseq_test_produce_no_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send = create_send(
            String::from("friends"),
            setup.carol.clone(),
            city_match_case(setup.carol),
        );

        let cres = rspace.put_once_non_durable_sequential(receive);
        let pres = rspace.get_once_non_durable_sequential(send);

        assert!(cres.is_none());
        assert!(pres.is_none());
        assert!(!rspace.is_memseq_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn memseq_test_consume_persist() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );

        let cres = rspace.put_always_non_durable_sequential(receive);

        assert!(cres.is_none());
        assert!(!rspace.is_memseq_empty());

        let pres = rspace.get_once_non_durable_sequential(send);

        assert!(pres.is_some());
        assert!(!rspace.is_memseq_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn memseq_test_produce_persist() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );

        let pres = rspace.get_always_non_durable_sequential(send);

        assert!(pres.is_none());
        assert!(!rspace.is_memseq_empty());

        let cres = rspace.put_once_non_durable_sequential(receive);

        assert!(cres.is_some());
        assert_eq!(cres.unwrap().len(), 1);
        assert!(!rspace.is_memseq_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn memseq_test_produce_persist_existing_matches() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive1 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let send1 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice.clone()),
        );
        let send2 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let receive2 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );

        let cres1 = rspace.put_once_non_durable_sequential(receive1);

        assert!(cres1.is_none());
        assert!(!rspace.is_memseq_empty());

        let pres1 = rspace.get_always_non_durable_sequential(send1);

        assert!(pres1.is_some());
        assert!((rspace.is_memseq_empty()));

        let pres2 = rspace.get_always_non_durable_sequential(send2);
        let _cres2 = rspace.put_once_non_durable_sequential(receive2);

        assert!(pres2.is_none());
        assert!(!rspace.is_memseq_empty());

        let _ = rspace.clear_store();
    }

    //memconc
    #[test]
    fn memconc_test_consume_persist_existing_matches() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send1 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice.clone()),
        );
        let send2 = create_send(
            String::from("friends"),
            setup.bob.clone(),
            city_match_case(setup.bob),
        );
        let receive1 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let receive2 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let receive3 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send3 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let _pres1 = rspace.get_once_non_durable_concurrent(send1);
        let _pres2 = rspace.get_once_non_durable_concurrent(send2);
        let cres1 = rspace.put_always_non_durable_concurrent(receive1);

        assert_eq!(cres1.unwrap().len(), 1);
        assert!(!rspace.is_memconc_empty());

        let cres2 = rspace.put_always_non_durable_concurrent(receive2);

        assert_eq!(cres2.unwrap().len(), 1);
        assert!(rspace.is_memconc_empty());

        let cres3 = rspace.put_always_non_durable_concurrent(receive3);

        assert!(cres3.is_none());
        assert!(!rspace.is_memconc_empty());

        let pres3 = rspace.get_once_non_durable_concurrent(send3);

        assert!(pres3.is_some());
        assert!(!rspace.is_memconc_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn memconc_test_multiple_channels_consume_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send1 = create_send(
            String::from("colleagues"),
            setup.dan.clone(),
            state_match_case(setup.dan),
        );
        let send2 = create_send(
            String::from("friends"),
            setup.erin.clone(),
            state_match_case(setup.erin),
        );
        let receive = create_receive(
            vec![String::from("friends"), String::from("colleagues")],
            vec![setup.state_pattern.clone(), setup.state_pattern],
            String::from("I am the continuation, for now..."),
        );

        let pres1 = rspace.get_once_non_durable_concurrent(send1);
        let pres2 = rspace.get_once_non_durable_concurrent(send2);

        let cres = rspace.put_once_non_durable_concurrent(receive);

        assert!(pres1.is_none());
        assert!(pres2.is_none());
        assert!(cres.is_some());
        assert_eq!(cres.unwrap().len(), 2);
        assert!(rspace.is_memconc_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn memconc_test_consume_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send = create_send(
            String::from("friends"),
            setup.bob.clone(),
            name_match_case(setup.bob),
        );
        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.name_pattern],
            String::from("I am the continuation, for now..."),
        );
        let pres = rspace.get_once_non_durable_concurrent(send);
        let cres = rspace.put_once_non_durable_concurrent(receive);

        assert!(pres.is_none());
        assert!(cres.is_some());
        assert!(rspace.is_memconc_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn memconc_test_produce_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );

        let cres = rspace.put_once_non_durable_concurrent(receive);
        let pres = rspace.get_once_non_durable_concurrent(send);

        assert!(cres.is_none());
        assert!(pres.is_some());
        assert!(rspace.is_memconc_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn memconc_test_produce_no_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send = create_send(
            String::from("friends"),
            setup.carol.clone(),
            city_match_case(setup.carol),
        );

        let cres = rspace.put_once_non_durable_concurrent(receive);
        let pres = rspace.get_once_non_durable_concurrent(send);

        assert!(cres.is_none());
        assert!(pres.is_none());
        assert!(!rspace.is_memconc_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn memconc_test_consume_persist() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );

        let cres = rspace.put_always_non_durable_concurrent(receive);

        assert!(cres.is_none());
        assert!(!rspace.is_memconc_empty());

        let pres = rspace.get_once_non_durable_concurrent(send);

        assert!(pres.is_some());
        assert!(!rspace.is_memconc_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn memconc_test_produce_persist() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );

        let pres = rspace.get_always_non_durable_concurrent(send);

        assert!(pres.is_none());
        assert!(!rspace.is_memconc_empty());

        let cres = rspace.put_once_non_durable_concurrent(receive);

        assert!(cres.is_some());
        assert_eq!(cres.unwrap().len(), 1);
        assert!(!rspace.is_memconc_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn memconc_test_produce_persist_existing_matches() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive1 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let send1 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice.clone()),
        );
        let send2 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let receive2 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );

        let cres1 = rspace.put_once_non_durable_concurrent(receive1);

        assert!(cres1.is_none());
        assert!(!rspace.is_memconc_empty());

        let pres1 = rspace.get_always_non_durable_concurrent(send1);

        assert!(pres1.is_some());
        assert!((rspace.is_memconc_empty()));

        let pres2 = rspace.get_always_non_durable_concurrent(send2);
        let _cres2 = rspace.put_once_non_durable_concurrent(receive2);

        assert!(pres2.is_none());
        assert!(!rspace.is_memconc_empty());

        let _ = rspace.clear_store();
    }

    //diskconc
    #[test]
    fn diskconc_test_consume_persist_existing_matches() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send1 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice.clone()),
        );
        let send2 = create_send(
            String::from("friends"),
            setup.bob.clone(),
            city_match_case(setup.bob),
        );
        let receive1 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let receive2 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let receive3 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send3 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );

        let _pres1 = rspace.get_once_durable_concurrent(send1);
        let _pres2 = rspace.get_once_durable_concurrent(send2);
        rspace.print_data("friends");
        let cres1 = rspace.put_always_durable_concurrent(receive1);
        rspace.print_data("friends");
        assert_eq!(cres1.unwrap().len(), 1);
        assert!(!rspace.is_diskconc_empty());

        let cres2 = rspace.put_always_durable_concurrent(receive2);

        assert_eq!(cres2.unwrap().len(), 1);
        assert!(rspace.is_diskconc_empty());

        let cres3 =
            rspace.put_always_durable_concurrent(receive3);

        assert!(cres3.is_none());
        assert!(!rspace.is_diskconc_empty());

        let pres3 = rspace.get_once_durable_concurrent(send3);

        assert!(pres3.is_some());
        assert!(!rspace.is_diskconc_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn diskconc_test_multiple_channels_consume_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send1 = create_send(
            String::from("colleagues"),
            setup.dan.clone(),
            state_match_case(setup.dan),
        );
        let send2 = create_send(
            String::from("friends"),
            setup.erin.clone(),
            state_match_case(setup.erin),
        );
        let receive = create_receive(
            vec![String::from("friends"), String::from("colleagues")],
            vec![setup.state_pattern.clone(), setup.state_pattern],
            String::from("I am the continuation, for now..."),
        );

        let pres1 = rspace.get_once_durable_concurrent(send1);
        let pres2 = rspace.get_once_durable_concurrent(send2);

        let cres = rspace.put_once_durable_concurrent(receive);

        assert!(pres1.is_none());
        assert!(pres2.is_none());
        assert!(cres.is_some());
        assert_eq!(cres.unwrap().len(), 2);
        assert!(rspace.is_diskconc_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn diskconc_test_consume_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send = create_send(
            String::from("friends"),
            setup.bob.clone(),
            name_match_case(setup.bob),
        );
        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.name_pattern],
            String::from("I am the continuation, for now..."),
        );

        let pres = rspace.get_once_durable_concurrent(send);
        let cres = rspace.put_once_durable_concurrent(receive);

        assert!(pres.is_none());
        assert!(cres.is_some());
        assert!(rspace.is_diskconc_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn diskconc_test_produce_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );

        let cres = rspace.put_once_durable_concurrent(receive);
        let pres = rspace.get_once_durable_concurrent(send);

        assert!(cres.is_none());
        assert!(pres.is_some());
        assert!(rspace.is_diskconc_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn diskconc_test_produce_no_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send = create_send(
            String::from("friends"),
            setup.carol.clone(),
            city_match_case(setup.carol),
        );
        let cres = rspace.put_once_durable_concurrent(receive);
        let pres = rspace.get_once_durable_concurrent(send);

        assert!(cres.is_none());
        assert!(pres.is_none());
        assert!(!rspace.is_diskconc_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn diskconc_test_consume_persist() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );

        let cres = rspace.put_always_durable_concurrent(receive);

        assert!(cres.is_none());
        assert!(!rspace.is_diskconc_empty());

        let pres = rspace.get_once_durable_concurrent(send);

        assert!(pres.is_some());
        assert!(!rspace.is_diskconc_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn diskconc_test_produce_persist() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );

        let pres = rspace.get_always_durable_concurrent(send);

        assert!(pres.is_none());
        assert!(!rspace.is_diskconc_empty());

        let cres = rspace.put_once_durable_concurrent(receive);

        assert!(cres.is_some());
        assert_eq!(cres.unwrap().len(), 1);
        assert!(!rspace.is_diskconc_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn diskconc_test_produce_persist_existing_matches() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive1 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let send1 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice.clone()),
        );
        let send2 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let receive2 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );

        let cres1 = rspace.put_once_durable_concurrent(receive1);

        assert!(cres1.is_none());
        assert!(!rspace.is_diskconc_empty());

        let pres1 = rspace.get_always_durable_concurrent(send1);

        assert!(pres1.is_some());
        assert!((rspace.is_diskconc_empty()));

        let pres2 = rspace.get_always_durable_concurrent(send2);
        let _cres2 = rspace.put_once_durable_concurrent(receive2);

        assert!(pres2.is_none());
        assert!(!rspace.is_diskconc_empty());

        let _ = rspace.clear_store();
    }

    //diskseq
    #[test]
    fn diskseq_test_consume_persist_existing_matches() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send1 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice.clone()),
        );
        let send2 = create_send(
            String::from("friends"),
            setup.bob.clone(),
            city_match_case(setup.bob),
        );
        let receive1 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let receive2 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let receive3 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send3 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );

        let _pres1 = rspace.get_once_durable_sequential(send1);
        let _pres2 = rspace.get_once_durable_sequential(send2);
        let cres1 = rspace.put_always_durable_sequential(receive1);

        assert_eq!(cres1.unwrap().len(), 1);
        assert!(!rspace.is_diskseq_empty());

        let cres2 = rspace.put_always_durable_sequential(receive2);

        assert_eq!(cres2.unwrap().len(), 1);
        assert!(rspace.is_diskseq_empty());

        let cres3 =
            rspace.put_always_durable_sequential(receive3);

        assert!(cres3.is_none());
        assert!(!rspace.is_diskseq_empty());

        let pres3 = rspace.get_once_durable_sequential(send3);

        assert!(pres3.is_some());
        assert!(!rspace.is_diskseq_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn diskseq_test_multiple_channels_consume_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send1 = create_send(
            String::from("colleagues"),
            setup.dan.clone(),
            state_match_case(setup.dan),
        );
        let send2 = create_send(
            String::from("friends"),
            setup.erin.clone(),
            state_match_case(setup.erin),
        );
        let receive = create_receive(
            vec![String::from("friends"), String::from("colleagues")],
            vec![setup.state_pattern.clone(), setup.state_pattern],
            String::from("I am the continuation, for now..."),
        );
        let pres1 = rspace.get_once_durable_sequential(send1);
        let pres2 = rspace.get_once_durable_sequential(send2);

        let cres = rspace.put_once_durable_sequential(receive);

        assert!(pres1.is_none());
        assert!(pres2.is_none());
        assert!(cres.is_some());
        assert_eq!(cres.unwrap().len(), 2);
        assert!(rspace.is_diskseq_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn diskseq_test_consume_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send = create_send(
            String::from("friends"),
            setup.bob.clone(),
            name_match_case(setup.bob),
        );
        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.name_pattern],
            String::from("I am the continuation, for now..."),
        );
        let pres = rspace.get_once_durable_sequential(send);
        let cres = rspace.put_once_durable_sequential(receive);

        assert!(pres.is_none());
        assert!(cres.is_some());
        assert!(rspace.is_diskseq_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn diskseq_test_produce_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let cres = rspace.put_once_durable_sequential(receive);
        let pres = rspace.get_once_durable_sequential(send);

        assert!(cres.is_none());
        assert!(pres.is_some());
        assert!(rspace.is_diskseq_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn diskseq_test_produce_no_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send = create_send(
            String::from("friends"),
            setup.carol.clone(),
            city_match_case(setup.carol),
        );

        let cres = rspace.put_once_durable_sequential(receive);
        let pres = rspace.get_once_durable_sequential(send);

        assert!(cres.is_none());
        assert!(pres.is_none());
        assert!(!rspace.is_diskseq_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn diskseq_test_consume_persist() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let cres = rspace.put_always_durable_sequential(receive);

        assert!(cres.is_none());
        assert!(!rspace.is_diskseq_empty());

        let pres = rspace.get_once_durable_sequential(send);

        assert!(pres.is_some());
        assert!(!rspace.is_diskseq_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn diskseq_test_produce_persist() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let send = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let receive = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );

        let pres = rspace.get_always_durable_sequential(send);

        assert!(pres.is_none());
        assert!(!rspace.is_diskseq_empty());

        let cres = rspace.put_once_durable_sequential(receive);

        assert!(cres.is_some());
        assert_eq!(cres.unwrap().len(), 1);
        assert!(!rspace.is_diskseq_empty());

        let _ = rspace.clear_store();
    }

    #[test]
    fn diskseq_test_produce_persist_existing_matches() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let receive1 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern.clone()],
            String::from("I am the continuation, for now..."),
        );
        let send1 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice.clone()),
        );
        let send2 = create_send(
            String::from("friends"),
            setup.alice.clone(),
            city_match_case(setup.alice),
        );
        let receive2 = create_receive(
            vec![String::from("friends")],
            vec![setup.city_pattern],
            String::from("I am the continuation, for now..."),
        );
        let cres1 = rspace.put_once_durable_sequential(receive1);

        assert!(cres1.is_none());
        assert!(!rspace.is_diskseq_empty());

        let pres1 = rspace.get_always_durable_sequential(send1);

        assert!(pres1.is_some());
        assert!((rspace.is_diskseq_empty()));

        let pres2 = rspace.get_always_durable_sequential(send2);
        let _cres2 = rspace.put_once_durable_sequential(receive2);

        assert!(pres2.is_none());
        assert!(!rspace.is_diskseq_empty());

        let _ = rspace.clear_store();
    }
}


    // #[test]
    // fn rspace_test_duplicate_keys() {
    //     let mut setup = Setup::new();
    //     let rspace = setup.rspace;
    //     setup.entries = vec![setup.emptyEntry.clone(),setup.emptyEntry.clone(),setup.emptyEntry.clone()];
    //     let binding = "chan1".to_string();
    //     let channels: Vec<&str> = vec! [&binding,&binding,&binding];

    //     let cres = rspace.put_once_non_durable_sequential(channels, vec! [city_match,state_match,name_match], Printer);
    //     rspace.print_data(&binding);
    //     let pres = rspace.get_once_non_durable_sequential(&binding, setup.emptyEntry);
    //     rspace.print_data(&binding);

    //     println!("\npres {:?}", pres);

    //     assert!(cres.is_none());
    //     assert!(pres.is_some());

    //     // let _ = rspace.clear();
    // }

    // #[test]
    // fn rspace_test_memory_concurrent() {
    //     let mut setup = Setup::new();
    //     let rspace = Arc::new(setup.rspace);
    //     setup.entries = vec![setup.emptyEntry.clone(),setup.emptyEntry.clone(),setup.emptyEntry.clone()];

    //     println!("\n -= -= -= -= starting =- =- =- =- \n");
    //     // Create 10 threads that write to the hashmap
    //     let mut handles = Vec::new();
    //     for i in 0..10 {
    //         let binding = "chan1".to_string();
    //         let mut e: Entry = setup.emptyEntry.clone();
    //         // fn new_match(entry: Entry, i:u8) -> bool {
    //         //     entry.pos == i
    //         // }
    //         e.pos = i;
    //         e.posStr = i.to_string();
    //         let rspace_clone = Arc::clone(&rspace);
    //         let handle = thread::spawn(move || {
    //             //putting this one on allows the output to go through but its nonsense
    //             //let pres = rspace_clone.put_once_non_durable_concurrent(vec! [&binding], vec! [pos_match],Printer);

    //             //this one makes sense to put in the data by calling get knowing it puts the entry in the db
    //             //printouts during test show them going as threaded and out of order
    //             let pres = rspace_clone.get_once_non_durable_concurrent(&binding, e);
    //             //println!("pres value: {:?}", pres);

    //             //println!("value: {}", pres.unwrap().data.posStr);
    //             //let v= pres.unwrap().data;
    //             // println!("i: {}", i);
    //             // println!("v pos: {}", v.pos);
    //             // println!("v posStr: {}", v.posStr);
    //         });
    //         handles.push(handle);
    //     }

    //     // Wait for all the threads to complete
    //     println!("\ncompleting puts\n");
    //     for handle in handles {
    //         handle.join().unwrap();
    //     }
    //     println!("\nended puts\n");

    //     println!("\nbegin print state of db\n");
    //     rspace.print_data(&"chan1".to_string());
    //     println!("\nend print state of db\n");

    //     // Check that all the values were written correctly
    //     // for i in 0..10 {
    //     //     let binding = "chan1".to_string();
    //     //     let mut e: Entry = setup.emptyEntry.clone();
    //     //     e.pos = i;
    //     //     e.posStr = i.to_string();
    //     //     let pres = rspace.get_once_non_durable_sequential(&binding, e);
    //     //     // let v= pres.data;
    //     //     // println!("i: {}", i);
    //     //     // println!("v pos: {}", v.pos);
    //     //     // println!("v posStr: {}", v.posStr);
    //     //     println!("pres {:?}", pres);
    //     //     //assert_eq!(v.pos, i);
    //     // }

    //     // // Create 10 threads that read from the hashmap
    //     let mut handles = Vec::new();
    //     for i in 0..10 {
    //         let binding = "chan1".to_string();
    //         let mut e: Entry = setup.emptyEntry.clone();
    //         e.pos = i;
    //         e.posStr = i.to_string();
    //         let rspace_clone = Arc::clone(&rspace);
    //         let handle = thread::spawn(move || {
    //             //putting this one on allows the output to go through but its nonsense
    //             let pres = rspace_clone.put_once_non_durable_concurrent(vec! [&binding], vec! [pos_match],Printer);

    //             //this one makes sense to put in the data by calling get knowing it puts the entry in the db
    //             //printouts during test show them going as threaded and out of order
    //             //let pres = rspace_clone.get_once_non_durable_concurrent(&binding, e);

    //             //println!("pres value: {:?}", pres);
    //             // let v= pres.unwrap().data;
    //             // println!("i: {}", i);
    //             // println!("v pos: {}", v.pos);
    //             // println!("v posStr: {}", v.posStr);
    //         });
    //         handles.push(handle);
    //     }

    //     // Wait for all the threads to complete
    //     println!("\ncompleting gets\n");
    //     for handle in handles {
    //         handle.join().unwrap();
    //     }
    //     println!("\nended gets\n");

    //     //     let cres = rspace.put_once_non_durable_concurrent(channels, vec! [city_match], Printer);
    //     //     rspace.print_data(&binding);
    //     //     let pres = rspace.get_once_non_durable_sequential(&binding, setup.emptyEntry);

    //     println!("\nbegin print state of db\n");
    //     rspace.print_data(&"chan1".to_string());
    //     println!("\nend print state of db\n");

    // }
    // // //cargo test --test rspace_test -- --test-threads=1

    // #[test]
    // fn rspace_test_default_memory_sequential() {
    //     memseq_test_consume_persist_existing_matches();
    //     // memseq_test_multiple_channels_consume_match();
    //     // memseq_test_consume_match();
    //     // memseq_test_produce_match();
    //     // memseq_test_produce_no_match();
    //     // memseq_test_consume_persist();
    //     // memseq_test_produce_persist();
    //     // memseq_test_produce_persist_existing_matches();

    // }

