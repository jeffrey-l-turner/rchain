#[cfg(test)]
mod tests {
    use rspace_plus_plus::rspace::RSpace;
    use rspace_plus_plus::example::{Address, Entry, Name, Printer};
    use std::thread;
    use std::sync::Arc;

    struct Setup {
        rspace: RSpace<Entry, Printer>,
        entries: Vec<Entry>,
        emptyEntry: Entry,
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

    fn pos_match(entry: Entry) -> bool {
        entry.pos.to_string() == entry.posStr
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

    #[test]
    fn memseq_test_consume_persist_existing_matches() {
        let setup = Setup::new();
        let rspace = setup.rspace;

        let _pres1 = rspace.get_once_non_durable_sequential("friends", setup.alice.clone());
        let _pres2 = rspace.get_once_non_durable_sequential("friends", setup.bob);
        rspace.print_data("friends");
        let cres1 = rspace.put_always_non_durable_sequential(vec!["friends"], vec![city_match], Printer);
        rspace.print_data("friends");
        assert_eq!(cres1.unwrap().len(), 1);
        assert!(!rspace.is_db_empty());

        let cres2: Option<Vec<rspace_plus_plus::shared::OptionResult<Entry, Printer>>> = rspace.put_always_non_durable_sequential(vec!["friends"], vec![city_match], Printer);

        assert_eq!(cres2.unwrap().len(), 1);
        assert!(rspace.is_db_empty());

        let cres3 = rspace.put_always_non_durable_sequential(vec!["friends"], vec![city_match], Printer);

        assert!(cres3.is_none());
        assert!(!rspace.is_db_empty());

        let pres3 = rspace.get_once_non_durable_sequential("friends", setup.alice.clone());

        assert!(pres3.is_some());
        assert!(!rspace.is_db_empty());

        let _ = rspace.clear_db();
    }

    // fn memseq_test_multiple_channels_consume_match() {
    //     let setup = Setup::new();
    //     let rspace = setup.rspace;

    //     let pres1 = rspace.get_once_non_durable_sequential("colleagues", setup.dan);
    //     let pres2 = rspace.get_once_non_durable_sequential("friends", setup.erin);

    //     let cres = rspace.put_once_non_durable_sequential(
    //         vec!["friends", "colleagues"],
    //         vec![state_match, state_match],
    //         Printer,
    //     );

    //     assert!(pres1.is_none());
    //     assert!(pres2.is_none());
    //     assert!(cres.is_some());
    //     assert_eq!(cres.unwrap().len(), 2);
    //     assert!(rspace.is_db_empty());

    //     let _ = rspace.clear_db();
    // }

    // fn memseq_test_consume_match() {
    //     let setup = Setup::new();
    //     let rspace = setup.rspace;

    //     let pres = rspace.get_once_non_durable_sequential("friends", setup.bob);
    //     let cres = rspace.put_once_non_durable_sequential(vec!["friends"], vec![name_match], Printer);

    //     assert!(pres.is_none());
    //     assert!(cres.is_some());
    //     assert!(rspace.is_db_empty());

    //     let _ = rspace.clear_db();
    // }

    // fn memseq_test_produce_match() {
    //     let setup = Setup::new();
    //     let rspace = setup.rspace;

    //     let cres = rspace.put_once_non_durable_sequential(vec!["friends"], vec![city_match], Printer);
    //     let pres = rspace.get_once_non_durable_sequential("friends", setup.alice);

    //     assert!(cres.is_none());
    //     assert!(pres.is_some());
    //     assert!(rspace.is_db_empty());

    //     let _ = rspace.clear_db();
    // }

    // fn memseq_test_produce_no_match() {
    //     let setup = Setup::new();
    //     let rspace = setup.rspace;

    //     let cres = rspace.put_once_non_durable_sequential(vec!["friends"], vec![city_match], Printer);
    //     let pres = rspace.get_once_non_durable_sequential("friends", setup.carol);

    //     assert!(cres.is_none());
    //     assert!(pres.is_none());
    //     assert!(rspace.is_db_empty());

    //     let _ = rspace.clear_db();
    // }

    // fn memseq_test_consume_persist() {
    //     let setup = Setup::new();
    //     let rspace = setup.rspace;

    //     let cres = rspace.put_always_non_durable_sequential(vec!["friends"], vec![city_match], Printer);

    //     assert!(cres.is_none());
    //     assert!(!rspace.is_db_empty());

    //     let pres = rspace.get_once_non_durable_sequential("friends", setup.alice.clone());

    //     assert!(pres.is_some());
    //     assert!(rspace.is_db_empty());

    //     let _ = rspace.clear_db();
    // }

    // fn memseq_test_produce_persist() {
    //     let setup = Setup::new();
    //     let rspace = setup.rspace;

    //     let pres = rspace.get_always_non_durable_sequential("friends", setup.alice);

    //     assert!(pres.is_none());
    //     assert!(!rspace.is_db_empty());

    //     let cres = rspace.put_once_non_durable_sequential(vec!["friends"], vec![city_match], Printer);

    //     assert!(cres.is_some());
    //     assert_eq!(cres.unwrap().len(), 1);
    //     assert!(rspace.is_db_empty());

    //     let _ = rspace.clear_db();
    // }

    // fn memseq_test_produce_persist_existing_matches() {
    //     let setup = Setup::new();
    //     let rspace = setup.rspace;

    //     let cres1 = rspace.put_once_non_durable_sequential(vec!["friends"], vec![city_match], Printer);

    //     assert!(cres1.is_none());
    //     assert!(!rspace.is_db_empty());

    //     let pres1 = rspace.get_always_non_durable_sequential("friends", setup.alice.clone());

    //     assert!(pres1.is_some());
    //     assert!((rspace.is_db_empty()));

    //     let pres2 = rspace.get_always_non_durable_sequential("friends", setup.alice.clone());
    //     let _cres2 = rspace.put_once_non_durable_sequential(vec!["friends"], vec![city_match], Printer);

    //     assert!(pres2.is_none());
    //     assert!(rspace.is_db_empty());

    //     let _ = rspace.clear_db();
    // }

}
