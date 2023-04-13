use crate::shared::*;
use dashmap::DashMap;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub struct MemConcDB<D, K> {
    db: DashMap<String, Vec<u8>>,
    phantom: PhantomData<(D, K)>,
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
    > MemConcDB<D, K>
{
    pub fn create() -> Result<MemConcDB<D, K>, Box<dyn Error>> {
        let db = DashMap::new();

        Ok(MemConcDB {
            db,
            phantom: PhantomData,
        })
    }

    pub fn consume(
        &self,
        channels: Vec<&str>,
        patterns: Vec<Pattern<D>>,
        continuation: K,
        persist: bool,
    ) -> Option<Vec<OptionResult<D, K>>> {
        if channels.len() == patterns.len() {
            let mut results: Vec<OptionResult<D, K>> = vec![];
            let mut stopper = false;

            for i in 0..channels.len() {
                let data_prefix = format!("channel-{}-data", channels[i]);

                self.db.retain(|key, value| {
                    if key.starts_with(&data_prefix) && !stopper {
                        let produce_data = bincode::deserialize::<ProduceData<D>>(&value).unwrap();

                        if patterns[i](produce_data.data.clone()) {
                            stopper = true;
                            results.push(OptionResult {
                                continuation: continuation.clone(),
                                data: produce_data.data,
                            });

                            if !produce_data.persist {
                                false
                            } else {
                                true
                            }
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                });
                stopper = false;
            }

            if results.len() > 0 {
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

                    let kdata_hash = self.calculate_hash(&k_data);
                    let key = format!("channel-{}-continuation-{}", &channels[i], &kdata_hash);

                    // returns old key if one was found
                    let _old_key = self.db.insert(key, k_data_bytes);
                }

                None
            }
        } else {
            println!("channel and pattern vectors are not equal length!");
            None
        }
    }

    pub fn produce(&self, channel: &str, entry: D, persist: bool) -> Option<OptionResult<D, K>> {
        let continuation_prefix = format!("channel-{}-continuation", channel);
        let mut result = None;

        self.db.retain(|key, value| {
            if key.starts_with(&continuation_prefix) {
                let k_data = bincode::deserialize::<KData<Pattern<D>, K>>(&value).unwrap();
                let pattern = k_data.pattern;

                if pattern(entry.clone()) {
                    result = Some(OptionResult {
                        continuation: k_data.continuation,
                        data: entry.clone(),
                    });
                    if !k_data.persist {
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            } else {
                true
            }
        });

        if result.is_some() {
            return result;
        } else {
            let produce_data = ProduceData {
                data: entry.clone(),
                persist,
            };

            println!("\nNo matching continuation for {:?}", produce_data);

            let data_hash = self.calculate_hash(&produce_data);
            let key = format!("channel-{}-data-{}", &channel, &data_hash);
            let data_bytes = bincode::serialize(&produce_data).unwrap();

            // returns old key if one was found
            let _old_key = self.db.insert(key, data_bytes);

            None
        }
    }

    pub fn print_channel(&self, channel: &str) -> Result<(), Box<dyn Error>> {
        if !self.db.is_empty() {
            println!("\nCurrent store state:");

            let continuation_prefix = format!("channel-{}-continuation", channel);
            let data_prefix = format!("channel-{}-data", channel);

            for entry in self.db.iter() {
                let data_bytes = entry.value();
                let key = entry.key();

                if key.starts_with(&continuation_prefix) {
                    let k_data = bincode::deserialize::<KData<Pattern<D>, K>>(&data_bytes).unwrap();
                    println!("KEY: {:?} VALUE: {:?}", key, k_data);
                } else if key.starts_with(&data_prefix) {
                    let data = bincode::deserialize::<ProduceData<D>>(&data_bytes).unwrap();
                    println!("KEY: {:?} VALUE: {:?}", key, data);
                } else {
                    println!("KEY: {:?} VALUE: {:?}", key, data_bytes);
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

    pub fn clear(&self) -> Result<(), Box<dyn Error>> {
        let _ = self.db.clear();
        Ok(())
    }

    fn calculate_hash<T: Hash>(&self, t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}
