#[cfg(test)]
mod tests {
    use rspace_plus_plus::rspace::RSpace;
    use rspace_plus_plus::example::{Address, Entry, Name, Printer};

    struct Setup {
        rspace: RSpace<Entry, Printer>,
        entries: Vec<Entry>,
        emptyEntry: Entry,
        
    }

    impl Setup {
        fn new() -> Self {
            Self {
                rspace: RSpace::create().unwrap(),
                entries:  Vec::new(),
                emptyEntry: Entry {
                    name: Name {
                        first: "First".to_string(),
                        last: "Last".to_string(),
                    },
                    address: Address {
                        street: "Street".to_string(),
                        city: "City".to_string(),
                        state: "State".to_string(),
                        zip: "12345".to_string(),
                    },
                    email: "email@some.com".to_string(),
                    phone: "123-456-7890".to_string(),
                },
                
            }
        }
    }

    fn city_match(entry: Entry) -> bool {
        entry.address.city == "City"
    }

    fn name_match(entry: Entry) -> bool {
        entry.name.last == "Last"
    }

    fn state_match(entry: Entry) -> bool {
        entry.address.state == "State"
    }

    #[test]
    fn rspace_test_duplicate_keys() {
        let mut setup = Setup::new();
        let rspace = setup.rspace;
        setup.entries = vec![setup.emptyEntry.clone(),setup.emptyEntry.clone(),setup.emptyEntry.clone()];
        let binding = "chan1".to_string();
        let channels: Vec<&str> = vec! [&binding,&binding,&binding];

        let cres = rspace.put_once_non_durable_sequential(channels, vec! [city_match,city_match,city_match], Printer);
        rspace.print_data(&binding);
        let pres = rspace.get_once_non_durable_sequential(&binding, setup.emptyEntry);

        assert!(cres.is_none());
        assert!(pres.is_some());

        // let _ = rspace.clear();
    }
    //cargo test --test rspace_test -- --test-threads=1


}
