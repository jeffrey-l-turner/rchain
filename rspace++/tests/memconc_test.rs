#[cfg(test)]
mod tests {
    use rspace_plus_plus::example::{Address, Entry, Name, Printer};
    use rspace_plus_plus::memconc::MemConcDB;

    struct Setup {
        memconc: MemConcDB<Entry, Printer>,
        alice: Entry,
        bob: Entry,
        carol: Entry,
        dan: Entry,
        erin: Entry,
    }

    impl Setup {
        fn new() -> Self {
            Self {
                memconc: MemConcDB::create().unwrap(),
                alice: Entry {
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
                    pos: 1,
                    posStr: "1".to_string(),
                },
                bob: Entry {
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
                    pos: 1,
                    posStr: "1".to_string(),
                },
                carol: Entry {
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
                    pos: 1,
                    posStr: "1".to_string(),
                },
                dan: Entry {
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
                    pos: 1,
                    posStr: "1".to_string(),
                },
                erin: Entry {
                    name: Name {
                        first: "Erin".to_string(),
                        last: "Rush".to_string(),
                    },
                    address: Address {
                        street: "23 Market St.".to_string(),
                        city: "Peony".to_string(),
                        state: "Idaho".to_string(),
                        zip: "224422".to_string(),
                    },
                    email: "erush@lasttraintogoa.net".to_string(),
                    phone: "333-555-1212".to_string(),
                    pos: 1,
                    posStr: "1".to_string(),
                },
            }
        }
    }

    fn city_match(entry: Entry) -> bool {
        entry.address.city == "Crystal Lake"
    }

    fn name_match(entry: Entry) -> bool {
        entry.name.last == "Lahblah"
    }

    fn state_match(entry: Entry) -> bool {
        entry.address.state == "Idaho"
    }

    #[test]
    fn test_produce_match() {
        let setup = Setup::new();
        let memconc = setup.memconc;

        let cres = memconc.consume(vec!["friends"], vec![city_match], Printer, false);
        let pres = memconc.produce("friends", setup.alice, false);

        assert!(cres.is_none());
        assert!(pres.is_some());
        assert!(memconc.is_empty());

        let _ = memconc.clear();
    }

    #[test]
    fn test_produce_no_match() {
        let setup = Setup::new();
        let memconc = setup.memconc;

        let cres = memconc.consume(vec!["friends"], vec![city_match], Printer, false);
        let pres = memconc.produce("friends", setup.carol, false);

        assert!(cres.is_none());
        assert!(pres.is_none());
        assert!(!memconc.is_empty());

        let _ = memconc.clear();
    }

    #[test]
    fn test_consume_match() {
        let setup = Setup::new();
        let memconc = setup.memconc;

        let pres = memconc.produce("friends", setup.bob, false);
        let cres = memconc.consume(vec!["friends"], vec![name_match], Printer, false);

        assert!(pres.is_none());
        assert!(cres.is_some());
        assert!(memconc.is_empty());

        let _ = memconc.clear();
    }

    #[test]
    fn test_multiple_channels_consume_match() {
        let setup = Setup::new();
        let memconc = setup.memconc;

        let pres1 = memconc.produce("colleagues", setup.dan, false);
        let pres2 = memconc.produce("friends", setup.erin, false);

        let cres = memconc.consume(
            vec!["friends", "colleagues"],
            vec![state_match, state_match],
            Printer,
            false,
        );

        assert!(pres1.is_none());
        assert!(pres2.is_none());
        assert!(cres.is_some());
        assert_eq!(cres.unwrap().len(), 2);
        assert!(memconc.is_empty());

        let _ = memconc.clear();
    }

    #[test]
    fn test_consume_persist() {
        let setup = Setup::new();
        let memconc = setup.memconc;

        let cres = memconc.consume(vec!["friends"], vec![city_match], Printer, true);

        assert!(cres.is_none());
        assert!(!memconc.is_empty());

        let pres = memconc.produce("friends", setup.alice.clone(), false);

        assert!(pres.is_some());
        assert!(!memconc.is_empty());

        let _ = memconc.clear();
    }

    #[test]
    fn test_consume_persist_existing_matches() {
        let setup = Setup::new();
        let memconc = setup.memconc;

        let _pres1 = memconc.produce("friends", setup.alice.clone(), false);
        let _pres2 = memconc.produce("friends", setup.bob, false);
        let cres1 = memconc.consume(vec!["friends"], vec![city_match], Printer, true);

        assert_eq!(cres1.unwrap().len(), 1);
        assert!(!memconc.is_empty());

        let cres2 = memconc.consume(vec!["friends"], vec![city_match], Printer, true);

        assert_eq!(cres2.unwrap().len(), 1);
        assert!(memconc.is_empty());

        let cres3 = memconc.consume(vec!["friends"], vec![city_match], Printer, true);

        assert!(cres3.is_none());
        assert!(!memconc.is_empty());

        let pres3 = memconc.produce("friends", setup.alice.clone(), false);

        assert!(pres3.is_some());
        assert!(!memconc.is_empty());

        let _ = memconc.clear();
    }

    #[test]
    fn test_produce_persist() {
        let setup = Setup::new();
        let memconc = setup.memconc;

        let pres = memconc.produce("friends", setup.alice, true);

        assert!(pres.is_none());
        assert!(!memconc.is_empty());

        let cres = memconc.consume(vec!["friends"], vec![city_match], Printer, false);

        assert!(cres.is_some());
        assert_eq!(cres.unwrap().len(), 1);
        assert!(!memconc.is_empty());

        let _ = memconc.clear();
    }

    #[test]
    fn test_produce_persist_existing_matches() {
        let setup = Setup::new();
        let memconc = setup.memconc;

        let cres1 = memconc.consume(vec!["friends"], vec![city_match], Printer, false);

        assert!(cres1.is_none());
        assert!(!memconc.is_empty());

        let pres1 = memconc.produce("friends", setup.alice.clone(), true);

        assert!(pres1.is_some());
        assert!((memconc.is_empty()));

        let pres2 = memconc.produce("friends", setup.alice.clone(), true);
        let _cres2 = memconc.consume(vec!["friends"], vec![city_match], Printer, false);

        assert!(pres2.is_none());
        assert!(!memconc.is_empty());

        let _ = memconc.clear();
    }
}
