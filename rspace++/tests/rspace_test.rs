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

    #[test]
    fn rspace_test_memory_concurrent() {
        let mut setup = Setup::new();
        let rspace = Arc::new(setup.rspace);
        setup.entries = vec![setup.emptyEntry.clone(),setup.emptyEntry.clone(),setup.emptyEntry.clone()];

        println!("\n -= -= -= -= starting =- =- =- =- \n");
        // Create 10 threads that write to the hashmap
        let mut handles = Vec::new();
        for i in 0..10 {
            let binding = "chan1".to_string();
            let mut e: Entry = setup.emptyEntry.clone();
            e.pos = i;
            e.posStr = i.to_string();
            let rspace_clone = Arc::clone(&rspace);
            let handle = thread::spawn(move || {
                //putting this one on allows the output to go through but its nonsense
                //let pres = rspace_clone.put_once_non_durable_sequential(vec! [&binding], vec! [pos_match],Printer);
                
                //this one makes sense to put in the data by calling get knowing it puts the entry in the db
                //printouts during test show them going as threaded and out of order
                let pres = rspace_clone.get_once_non_durable_sequential(&binding, e);

                // let v= pres.unwrap().data;
                // println!("i: {}", i);
                // println!("v pos: {}", v.pos);
                // println!("v posStr: {}", v.posStr);
            });
            handles.push(handle);
        }

        // Wait for all the threads to complete
        println!("\ncompleting puts\n");
        for handle in handles {
            handle.join().unwrap();
        }
        println!("\nended puts\n");

        println!("\nbegin print state of db\n");
        rspace.print_data(&"chan1".to_string());
        println!("\nend print state of db\n");

        // Check that all the values were written correctly
        // for i in 0..10 {
        //     let binding = "chan1".to_string();
        //     let mut e: Entry = setup.emptyEntry.clone();
        //     e.pos = i;
        //     e.posStr = i.to_string();
        //     let pres = rspace.get_once_non_durable_sequential(&binding, e);
        //     // let v= pres.data;
        //     // println!("i: {}", i);
        //     // println!("v pos: {}", v.pos);
        //     // println!("v posStr: {}", v.posStr);
        //     println!("pres {:?}", pres);
        //     //assert_eq!(v.pos, i);
        // }

        // // Create 10 threads that read from the hashmap
        let mut handles = Vec::new();
        for i in 0..10 {
            let binding = "chan1".to_string();
            let mut e: Entry = setup.emptyEntry.clone();
            e.pos = i;
            e.posStr = i.to_string();
            let rspace_clone = Arc::clone(&rspace);
            let handle = thread::spawn(move || {
                //putting this one on allows the output to go through but its nonsense
                //let pres = rspace_clone.put_once_non_durable_sequential(vec! [&binding], vec! [pos_match],Printer);
                
                //this one makes sense to put in the data by calling get knowing it puts the entry in the db
                //printouts during test show them going as threaded and out of order
                let pres = rspace_clone.get_once_non_durable_sequential(&binding, e);

                // let v= pres.unwrap().data;
                // println!("i: {}", i);
                // println!("v pos: {}", v.pos);
                // println!("v posStr: {}", v.posStr);
            });
            handles.push(handle);
        }

        // Wait for all the threads to complete
        println!("\ncompleting gets\n");
        for handle in handles {
            handle.join().unwrap();
        }
        println!("\nended gets\n");

        //     let cres = rspace.put_once_non_durable_concurrent(channels, vec! [city_match], Printer);
        //     rspace.print_data(&binding);
        //     let pres = rspace.get_once_non_durable_sequential(&binding, setup.emptyEntry);

        println!("\nbegin print state of db\n");
        rspace.print_data(&"chan1".to_string());
        println!("\nend print state of db\n");


    }
    // //cargo test --test rspace_test -- --test-threads=1


}
