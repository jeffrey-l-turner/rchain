use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use crate::shared::*;

pub struct MemSeqDB<D, K> {
    db: HashMap<String, Vec<u8>>,
    phantom: PhantomData<(D, K)>,
}

#[derive(Debug)]
struct KeyDataTracker {
    iVal: usize,
    key: String,
}

impl<
        D: Clone
            + std::hash::Hash
            + std::fmt::Debug
            + serde::Serialize
            + for<'a> serde::Deserialize<'a>,
        K: Clone
            + std::hash::Hash
            + std::fmt::Debug
            + serde::Serialize
            + for<'a> serde::Deserialize<'a>
            + 'static,
    > MemSeqDB<D, K>
{
    pub fn create() -> Result<MemSeqDB<D, K>, Box<dyn Error>> {
        let db = HashMap::new();

        Ok(MemSeqDB {
            db,
            phantom: PhantomData,
        })
    }

    pub fn consume(
        &mut self,
        channels: Vec<&str>,
        patterns: Vec<Pattern<D>>,
        continuation: K,
        persist: bool,
    ) -> Option<Vec<OptionResult<D, K>>> {
        if channels.len() == patterns.len() {
            println!("memseq consume called");
            let mut results: Vec<OptionResult<D, K>> = vec![];

            for i in 0..channels.len() {
                let data_prefix = format!("channel-{}-data", channels[i]);

                //find the keys in the hashmap that start with the data_prefix
                for (k,v) in &self.db {
                    if k.starts_with(&data_prefix) {

                        match self.db.get(k) {
                            Some(value) => {
                                let produce_data: ProduceData<D> =
                                    bincode::deserialize::<ProduceData<D>>(&value).unwrap();
                                if patterns[i](produce_data.data.clone()) {
                                    if !produce_data.persist {
                                        self.db.remove(&k.clone());
                                    }
    
                                    results.push(OptionResult {
                                        continuation: continuation.clone(),
                                        data: produce_data.data,
                                    });
                                    break;
                                } else {
                                    //println!("consume NO PATTERN MATCH");
                                }
                            }
                            None => println!("Key not found"),
                        }
                    }
                }
            }

            

            if results.len() > 0 {
                println!("returning results with length {}", results.len());
                // for res in results.clone() {
                //     println!("returning results with data {:?}", res.data);
                // }
                return Some(results);
            } else {
                for i in 0..channels.len() {
                    let k_data = KData {
                        pattern: patterns[i],
                        continuation: continuation.clone(),
                        persist,
                    };

                    println!("\nNo matching data for {:?}", k_data);

                    let k_data_bytes = bincode::serialize(&k_data).unwrap();

                    // opening a write transaction
                    let kdata_hash = self.calculate_hash(&k_data);
                    let key = format!("channel-{}-continuation-{}", &channels[i], &kdata_hash);

                    let _ = self.db.insert(key, k_data_bytes);
                }

                None
            }
        } else {
            println!("channel and pattern vectors are not equal length!");
            None
        }
    }

    pub fn produce(
        &mut self,
        channel: &str,
        entry: D,
        persist: bool,
    ) -> Option<OptionResult<D, K>> {

        println!("produce called in memseq");

        let continuation_prefix = format!("channel-{}-continuation", channel);
        let mut valid_keys: Vec<String> = Vec::new();

        //find the keys in the hashmap that start with the continuation_prefix
        for (k,v) in &self.db {
            if k.starts_with(&continuation_prefix) {
                valid_keys.push(k.to_string());
            }
        }

        // Loop over the keys in the hashmap that start with the continuation_prefix
        for key in valid_keys {
            println!("produce Key: {}", key);
            match self.db.get(&key) {
                Some(value) => {
                    let k_data_bytes = value;
                    let k_data: KData<Pattern<D>, K> =
                        bincode::deserialize::<KData<Pattern<D>, K>>(k_data_bytes).unwrap();
                    let pattern = k_data.pattern;

                    if pattern(entry.clone()) {
                        if !k_data.persist {
                            self.db.remove(&key);
                        }

                        return Some(OptionResult {
                            continuation: k_data.continuation,
                            data: entry.clone(),
                        });
                    } else {
                        //println!("produce NO PATTERN MATCH");
                    }
                }
                None => println!("Key not found"),
            }
        }

        let produce_data = ProduceData {
            data: entry.clone(),
            persist,
        };

        println!("\nNo matching continuation for {:?}", produce_data);

        let data_hash = self.calculate_hash(&produce_data);
        let key = format!("channel-{}-data-{}", &channel, &data_hash);
        let data_bytes = bincode::serialize(&produce_data).unwrap();

        let _ = self.db.insert(key, data_bytes);

        None
    }

    pub fn print_channel(&self, channel: &str) -> Result<(), Box<dyn Error>> {
        let continuation_prefix = format!("channel-{}-continuation", channel);
        let data_prefix = format!("channel-{}-data", channel);

        if !self.db.is_empty() {
            println!("\nCurrent channel state for \"{}\":", channel);

            for key in self
                .db
                .keys()
                .take_while(|key| key.starts_with(&continuation_prefix))
            {
                println!("continuation Key: {}", key);
                match self.db.get(key) {
                    Some(value) => {
                        let k_data_bytes = value;
                        let k_data: KData<Pattern<D>, K> =
                            bincode::deserialize::<KData<Pattern<D>, K>>(k_data_bytes).unwrap();
                        println!("KEY: {:?} VALUE: {:?}", key, k_data);
                    }
                    None => println!("Key not found"),
                }
            }

            for key in self
                .db
                .keys()
                .take_while(|key| key.starts_with(&data_prefix))
            {
                println!("data Key: {}", key);
                match self.db.get(key) {
                    Some(value) => {
                        let data: D = bincode::deserialize::<D>(value).unwrap();
                        println!("KEY: {:?} VALUE: {:?}", key, data);
                    }
                    None => println!("Key not found"),
                }
            }
        } else {
            println!("\nDatabase is empty")
        }

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        return self.db.is_empty();
    }

    pub fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        self.db.clear();
        Ok(())
    }

    fn calculate_hash<T: Hash>(&self, t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

// impl<D, K> MyTrait<D,K> for MemSeqDB<D, K> {
//     fn my_method(&self) {
//         // implementation for MemSeqDB's my_method
//         println!("MemSeqDB my_method")
//     }

//     fn clear(&mut self) -> Result<(), Box<dyn Error>> {
//         self.db.clear();
//         Ok(())
//     }
//     // implement more methods/functions here
// }
