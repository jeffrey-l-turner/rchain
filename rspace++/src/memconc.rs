#![allow(dead_code)]

use crate::rtypes::rtypes;
use dashmap::DashMap;
use prost::Message;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

// TODO: Change to better naming throughout

pub struct MemConcDB<D: Message, K: Message> {
    db: DashMap<String, Vec<u8>>,
    phantom: PhantomData<(D, K)>,
}

impl<
        D: Clone + std::hash::Hash + std::fmt::Debug + std::default::Default + prost::Message,
        K: Clone + std::hash::Hash + std::fmt::Debug + std::default::Default + prost::Message,
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
        receive: rtypes::Receive,
        persistent: bool,
    ) -> Option<Vec<rtypes::OptionResult>> {
        if receive.channels.len() == receive.patterns.len() {
            let mut results: Vec<rtypes::OptionResult> = vec![];
            let mut stopper = false;

            for i in 0..receive.channels.len() {
                let data_prefix = format!("channel-{}-data", receive.channels[i]);

                self.db.retain(|key, value| {
                    println!("memconc consume retain keyval {:?}", key);
                    if key.starts_with(&data_prefix) && !stopper {
                        println!("memconc consume match keyval {:?}", key);
                        let pdata = rtypes::ProduceData::decode(value.as_slice()).unwrap();

                        // TODO: Implement better pattern/match schema
                        if receive.patterns[i] == pdata.match_case {
                            stopper = true;

                            let mut option_result = rtypes::OptionResult::default();
                            option_result.continuation = receive.continuation.clone();
                            option_result.data = pdata.data.clone();

                            results.push(option_result);

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
                for i in 0..receive.channels.len() {
                    let mut consume_data = rtypes::ConsumeData::default();
                    consume_data.pattern = receive.patterns[i].clone();
                    consume_data.continuation = receive.continuation.clone();
                    consume_data.persistent = persistent;

                    println!("\nNo matching data for {:?}", receive);

                    let data_hash = self.calculate_hash(&consume_data);
                    let key = format!(
                        "channel-{}-continuation-{}",
                        &receive.channels[i], &data_hash
                    );

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

    pub fn produce(&self, send: rtypes::Send, persistent: bool) -> Option<rtypes::OptionResult> {
        let continuation_prefix = format!("channel-{}-continuation", send.chan);
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
                if cdata.pattern == send.match_case {
                    stopper = true;

                    let mut option_result = rtypes::OptionResult::default();
                    option_result.continuation = cdata.continuation.clone();
                    option_result.data = send.data.clone();

                    result = Some(option_result);
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
            produce_data.data = send.data.clone();
            produce_data.match_case = send.match_case.clone();
            produce_data.persistent = persistent;

            println!("\nNo matching continuation for {:?}", send);

            let data_hash = self.calculate_hash(&produce_data);
            let key = format!("channel-{}-data-{}", &send.chan, &data_hash);

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
