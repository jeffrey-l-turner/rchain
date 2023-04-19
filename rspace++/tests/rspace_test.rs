#[cfg(test)]
mod tests {
    use rspace_plus_plus::rspace::RSpace;
    use rspace_plus_plus::example::{Address, Entry, Name, Printer};
    use std::thread;

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
                    pos: 1,
                    posStr: "1".to_string(),
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

        let cres = rspace.put_once_non_durable_sequential(channels, vec! [city_match,state_match,name_match], Printer);
        rspace.print_data(&binding);
        let pres = rspace.get_once_non_durable_sequential(&binding, setup.emptyEntry);
        rspace.print_data(&binding);

        println!("\npres {:?}", pres);
        
        
        assert!(cres.is_none());
        assert!(pres.is_some());

        // let _ = rspace.clear();
    }

    // #[test]
    // fn rspace_test_memory_concurrent() {
    //     let mut setup = Setup::new();
    //     let rspace = setup.rspace;
    //     setup.entries = vec![setup.emptyEntry.clone(),setup.emptyEntry.clone(),setup.emptyEntry.clone()];
    //     let binding = "chan1".to_string();
    //     let channels: Vec<&str> = vec! [&binding];

    //     // Create 10 threads that write to the hashmap
    // let mut handles = Vec::new();
    // for i in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let cres = rspace.put_once_non_durable_concurrent(channels, vec! [city_match], Printer);
    //         map.insert(i, i * 10);
    //     });
    //     handles.push(handle);
    // }

    // // Wait for all the threads to complete
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // // Check that all the values were written correctly
    // for i in 0..10 {
    //     let map = map.lock().unwrap();
    //     assert_eq!(*map.get(&i).unwrap(), i * 10);
    // }

    // // Create 10 threads that read from the hashmap
    // let mut handles = Vec::new();
    // for _ in 0..10 {
    //     let map = Arc::clone(&map);
    //     let handle = thread::spawn(move || {
    //         let map = map.lock().unwrap();
    //         for i in 0..10 {
    //             assert_eq!(*map.get(&i).unwrap(), i * 10);
    //         }
    //     });
    //     handles.push(handle);
    // }

    // // Wait for all the threads to complete
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    //     let cres = rspace.put_once_non_durable_concurrent(channels, vec! [city_match], Printer);
    //     rspace.print_data(&binding);
    //     let pres = rspace.get_once_non_durable_sequential(&binding, setup.emptyEntry);


    // }
    // //cargo test --test rspace_test -- --test-threads=1


}
