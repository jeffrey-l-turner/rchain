#[cfg(test)]
mod tests {
    use rspace_plus_plus::example::{Address, Entry, Name, Printer};
    use rspace_plus_plus::rspace::RSpace;

    struct Setup {
        rspace: RSpace<Entry, Printer>,
        alice: Entry,
        bob: Entry,
        carol: Entry,
        dan: Entry,
        erin: Entry,
    }

    impl Setup {
        fn new() -> Self {
            Self {
                rspace: RSpace::create().unwrap(),
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
        let rspace = setup.rspace;

        let cres = rspace.consume(vec!["friends"], vec![city_match], Printer, false);
        let pres = rspace.produce("friends", setup.alice, false);

        assert!(cres.is_none());
        assert!(pres.is_some());
        assert!(rspace.is_empty());

        let _ = rspace.clear();
    }

    #[test]
    fn test_produce_no_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let cres = rspace.consume(vec!["friends"], vec![city_match], Printer, false);
        let pres = rspace.produce("friends", setup.carol, false);

        assert!(cres.is_none());
        assert!(pres.is_none());
        assert!(!rspace.is_empty());

        let _ = rspace.clear();
    }

    #[test]
    fn test_consume_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let pres = rspace.produce("friends", setup.bob, false);
        let cres = rspace.consume(vec!["friends"], vec![name_match], Printer, false);

        assert!(pres.is_none());
        assert!(cres.is_some());
        assert!(rspace.is_empty());

        let _ = rspace.clear();
    }

    #[test]
    fn test_multiple_channels_consume_match() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let pres1 = rspace.produce("colleagues", setup.dan, false);
        let pres2 = rspace.produce("friends", setup.erin, false);

        let cres = rspace.consume(
            vec!["friends", "colleagues"],
            vec![state_match, state_match],
            Printer,
            false,
        );

        assert!(pres1.is_none());
        assert!(pres2.is_none());
        assert!(cres.is_some());
        assert_eq!(cres.unwrap().len(), 2);
        assert!(rspace.is_empty());

        let _ = rspace.clear();
    }

    #[test]
    fn test_persist_multiple_matches() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let _pres1 = rspace.produce("friends", setup.alice, false);
        let _pres2 = rspace.produce("friends", setup.bob, false);

        let cres = rspace.consume(vec!["friends"], vec![city_match], Printer, true);

        assert_eq!(cres.unwrap().len(), 1);
        assert!(!rspace.is_empty());

        let _ = rspace.clear();
    }
}
