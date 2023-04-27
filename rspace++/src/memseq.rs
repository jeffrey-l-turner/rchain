use crate::rtypes::rtypes;
use crate::shared::*;
use dashmap::DashMap;
use prost::Message;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub struct MemSeqDB<D: Message, K: Message> {
    db: DashMap<String, Vec<u8>>,
    phantom: PhantomData<(D, K)>,
}

impl<
        D: Clone + std::hash::Hash + std::fmt::Debug + std::default::Default + prost::Message,
        K: Clone + std::hash::Hash + std::fmt::Debug + std::default::Default + prost::Message,
    > MemSeqDB<D, K>
{
    pub fn create() -> Result<MemSeqDB<D, K>, Box<dyn Error>> {
        let db = DashMap::new();

        Ok(MemSeqDB {
            db,
            phantom: PhantomData,
        })
    }

    pub fn consume(&self, cdata: rtypes::Receive) -> Option<Vec<OptionResult>> {
        if cdata.channels.len() == cdata.patterns.len() {
            let mut results: Vec<OptionResult> = vec![];
            let mut stopper = false;

            for i in 0..cdata.channels.len() {
                let data_prefix = format!("channel-{}-data", cdata.channels[i]);

                self.db.retain(|key, value| {
                    println!("memconc consume retain keyval {:?}", key);
                    if key.starts_with(&data_prefix) && !stopper {
                        println!("memconc consume match keyval {:?}", key);
                        let pdata = rtypes::ProduceData::decode(value.as_slice()).unwrap();

                        // TODO: Implement better pattern/match schema
                        if cdata.patterns[i] == pdata.data.clone().unwrap().name.unwrap().last {
                            stopper = true;
                            // TODO: Add OptionResult to rtypes.proto
                            results.push(OptionResult {
                                continuation: cdata.continuation.clone(),
                                data: pdata.data.clone().unwrap(),
                            });

                            if !pdata.persistent {
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
                for i in 0..cdata.channels.len() {
                    let mut consume_data = rtypes::ConsumeData::default();
                    consume_data.pattern = cdata.patterns[i].clone();
                    consume_data.continuation = cdata.continuation.clone();
                    consume_data.persistent = cdata.persistent;

                    println!("\nNo matching data for {:?}", cdata);

                    let data_hash = self.calculate_hash(&consume_data);
                    let key = format!("channel-{}-continuation-{}", &cdata.channels[i], &data_hash);

                    let mut consume_data_buf = Vec::new();
                    consume_data_buf.reserve(consume_data.encoded_len());
                    consume_data.encode(&mut consume_data_buf).unwrap();

                    // returns old key if one was found
                    let _old_key = self.db.insert(key, consume_data_buf);
                }

                None
            }
        } else {
            println!("channel and pattern vectors are not equal length!");
            None
        }
    }

    pub fn produce(&self, pdata: rtypes::Send) -> Option<OptionResult> {
        let continuation_prefix = format!("channel-{}-continuation", pdata.chan);
        let mut result = None;
        let mut stopper = false;

        //TODO: make this more efficient...
        //right now it loops through whole db and doesnt stop after first match

        self.db.retain(|key, value| {
            println!("memconc produce retain keyval {:?}", key);
            if key.starts_with(&continuation_prefix) && !stopper {
                println!("memconc produce match keyval {:?}", key);
                let cdata = rtypes::ConsumeData::decode(value.as_slice()).unwrap();

                // TODO: Implement better pattern/match schema
                if cdata.pattern == pdata.data.as_ref().unwrap().name.as_ref().unwrap().last {
                    stopper = true;
                    // TODO: Add OptionResult to rtypes.proto
                    result = Some(OptionResult {
                        continuation: cdata.continuation,
                        data: pdata.data.clone().unwrap(),
                    });
                    if !cdata.persistent {
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
            let mut produce_data = rtypes::ProduceData::default();
            produce_data.data = pdata.data.clone();
            produce_data.persistent = pdata.persistent;

            println!("\nNo matching continuation for {:?}", pdata);

            let data_hash = self.calculate_hash(&produce_data);
            let key = format!("channel-{}-data-{}", &pdata.chan, &data_hash);

            let mut produce_data_buf = Vec::new();
            produce_data_buf.reserve(produce_data.encoded_len());
            produce_data.encode(&mut produce_data_buf).unwrap();

            // returns old key if one was found
            let _old_key = self.db.insert(key, produce_data_buf);

            None
        }
    }

    pub fn print_channel(&self, channel: &str) -> Result<(), Box<dyn Error>> {
        if !self.db.is_empty() {
            println!("\nCurrent store state:");

            let continuation_prefix = format!("channel-{}-continuation", channel);
            let data_prefix = format!("channel-{}-data", channel);

            for entry in self.db.iter() {
                let data_buf = entry.value();
                let key = entry.key();

                if key.starts_with(&continuation_prefix) {
                    let cdata = rtypes::ConsumeData::decode(data_buf.as_slice()).unwrap();
                    println!("KEY: {:?} VALUE: {:?}", key, cdata);
                } else if key.starts_with(&data_prefix) {
                    let pdata = rtypes::ProduceData::decode(data_buf.as_slice()).unwrap();
                    println!("KEY: {:?} VALUE: {:?}", key, pdata);
                } else {
                    println!("KEY: {:?} VALUE: {:?}", key, data_buf);
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

// impl<
// D: Clone
//     + std::hash::Hash
//     + std::fmt::Debug
//     + serde::Serialize
//     + for<'a> serde::Deserialize<'a>,
// K: Clone
//     + std::hash::Hash
//     + std::fmt::Debug
//     + serde::Serialize
//     + for<'a> serde::Deserialize<'a>
//     + 'static,
// > MyTrait<D, K> for MemSeqDB<D, K> {
//     fn my_method(&mut self) {
//         // implementation for MemSeqDB's my_method
//         println!("MemSeqDB my_method")
//     }
//     // implement more methods/functions here

//     fn consume(
//         &self,
//         channels: Vec<&str>,
//         patterns: Vec<Pattern<D>>,
//         continuation: K,
//         persist: bool,
//     ) -> Option<Vec<OptionResult<D, K>>> {
//         if channels.len() == patterns.len() {
//             let mut results: Vec<OptionResult<D, K>> = vec![];
//             let mut stopper = false;

//             for i in 0..channels.len() {
//                 let data_prefix = format!("channel-{}-data", channels[i]);

//                 self.db.retain(|key, value| {
//                     if key.starts_with(&data_prefix) && !stopper {
//                         let produce_data = bincode::deserialize::<ProduceData<D>>(&value).unwrap();

//                         if patterns[i](produce_data.data.clone()) {
//                             stopper = true;
//                             results.push(OptionResult {
//                                 continuation: continuation.clone(),
//                                 data: produce_data.data,
//                             });

//                             if !produce_data.persist {
//                                 false
//                             } else {
//                                 true
//                             }
//                         } else {
//                             true
//                         }
//                     } else {
//                         true
//                     }
//                 });
//                 stopper = false;
//             }

//             if results.len() > 0 {
//                 return Some(results);
//             } else {
//                 for i in 0..channels.len() {
//                     let k_data = KData {
//                         pattern: patterns[i],
//                         continuation: continuation.clone(),
//                         persist,
//                     };

//                     println!("\nNo matching data for {:?}", k_data);

//                     let k_data_bytes = bincode::serialize(&k_data).unwrap();

//                     let kdata_hash = self.calculate_hash(&k_data);
//                     let key = format!("channel-{}-continuation-{}", &channels[i], &kdata_hash);

//                     // returns old key if one was found
//                     let _old_key = self.db.insert(key, k_data_bytes);
//                 }

//                 None
//             }
//         } else {
//             println!("channel and pattern vectors are not equal length!");
//             None
//         }
//     }

//     fn produce(&self, channel: &str, entry: D, persist: bool) -> Option<OptionResult<D, K>> {
//         let continuation_prefix = format!("channel-{}-continuation", channel);
//         let mut result = None;
//         let mut stopper = false;

//         //TODO: make this more efficient...
//         //right now it loops through whole db and doesnt stop after first match
//         self.db.retain(|key, value| {
//             if key.starts_with(&continuation_prefix) && !stopper {
//                 let k_data = bincode::deserialize::<KData<Pattern<D>, K>>(&value).unwrap();
//                 let pattern = k_data.pattern;

//                 if pattern(entry.clone()) {
//                     stopper = true;
//                     result = Some(OptionResult {
//                         continuation: k_data.continuation,
//                         data: entry.clone(),
//                     });
//                     if !k_data.persist {
//                         false
//                     } else {
//                         true
//                     }
//                 } else {
//                     true
//                 }
//             } else {
//                 true
//             }
//         });

//         if result.is_some() {
//             return result;
//         } else {
//             let produce_data = ProduceData {
//                 data: entry.clone(),
//                 persist,
//             };

//             println!("\nNo matching continuation for {:?}", produce_data);

//             let data_hash = self.calculate_hash(&produce_data);
//             let key = format!("channel-{}-data-{}", &channel, &data_hash);
//             let data_bytes = bincode::serialize(&produce_data).unwrap();

//             // returns old key if one was found
//             let _old_key = self.db.insert(key, data_bytes);

//             None
//         }
//     }

//     fn clear(&self) -> Result<(), Box<dyn Error>> {
//         let _ = self.db.clear();
//         Ok(())
//     }

//     fn is_empty(&self) -> bool {
//         return self.db.is_empty();
//     }

//     fn print_channel(&self, channel: &str) -> Result<(), Box<dyn Error>> {
//         if !self.db.is_empty() {
//             println!("\nCurrent store state:");

//             let continuation_prefix = format!("channel-{}-continuation", channel);
//             let data_prefix = format!("channel-{}-data", channel);

//             for entry in self.db.iter() {
//                 let data_bytes = entry.value();
//                 let key = entry.key();

//                 if key.starts_with(&continuation_prefix) {
//                     let k_data = bincode::deserialize::<KData<Pattern<D>, K>>(&data_bytes).unwrap();
//                     println!("KEY: {:?} VALUE: {:?}", key, k_data);
//                 } else if key.starts_with(&data_prefix) {
//                     let data = bincode::deserialize::<ProduceData<D>>(&data_bytes).unwrap();
//                     println!("KEY: {:?} VALUE: {:?}", key, data);
//                 } else {
//                     println!("KEY: {:?} VALUE: {:?}", key, data_bytes);
//                 }
//             }
//         } else {
//             println!("\nDatabase is empty")
//         }

//         Ok(())
//     }

// }
