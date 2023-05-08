#![allow(dead_code)]

use crate::rtypes::rtypes;
use heed::types::*;
use heed::{Database, Env, EnvOpenOptions};
use prost::Message;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fs;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::path::Path;

// TODO: Change to better naming throughout

/*
See RSpace.scala and Tuplespace.scala in rspace/
*/
#[repr(C)]
pub struct DiskSeqDB<D: Message, K: Message> {
    env: Env,
    db: Database<Str, SerdeBincode<Vec<u8>>>,
    phantom: PhantomData<(D, K)>,
}

impl<
        D: Clone + std::hash::Hash + std::fmt::Debug + std::default::Default + prost::Message,
        K: Clone + std::hash::Hash + std::fmt::Debug + std::default::Default + prost::Message,
    > DiskSeqDB<D, K>
{
    pub fn create() -> Result<DiskSeqDB<D, K>, Box<dyn Error>> {
        fs::create_dir_all(Path::new("target").join("DiskSeqDB"))?;
        let env = EnvOpenOptions::new().open(Path::new("target").join("DiskSeqDB"))?;

        // open the default unamed database
        let db = env.create_database(None)?;

        Ok(DiskSeqDB {
            env,
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
            let rtxn = self.env.read_txn().unwrap();

            for i in 0..receive.channels.len() {
                let data_prefix = format!("channel-{}-data", receive.channels[i]);
                let mut iter_data = self.db.prefix_iter(&rtxn, &data_prefix).unwrap();
                let mut iter_data_option = iter_data.next().transpose().unwrap();

                while iter_data_option.is_some() {
                    let iter_data_unwrap = iter_data_option.unwrap();
                    let pdata_buf = iter_data_unwrap.1;
                    let pdata = rtypes::ProduceData::decode(pdata_buf.as_slice()).unwrap();

                    // TODO: Implement better pattern/match schema
                    if receive.patterns[i] == pdata.match_case {
                        if !pdata.persistent {
                            let mut wtxn = self.env.write_txn().unwrap();
                            let _ = self.db.delete(&mut wtxn, iter_data_unwrap.0);
                            wtxn.commit().unwrap();
                        }

                        let mut option_result = rtypes::OptionResult::default();
                        option_result.continuation = receive.continuation.clone();
                        option_result.data = pdata.data.clone();

                        results.push(option_result);
                        break;
                    }
                    iter_data_option = iter_data.next().transpose().unwrap();
                }
                drop(iter_data);
            }
            rtxn.commit().unwrap();

            if results.len() > 0 {
                return Some(results);
            } else {
                for i in 0..receive.channels.len() {
                    let mut consume_data = rtypes::ConsumeData::default();
                    consume_data.pattern = receive.patterns[i].clone();
                    consume_data.continuation = receive.continuation.clone();
                    consume_data.persistent = persistent;

                    println!("\nNo matching data for {:?}", receive);

                    // opening a write transaction
                    let mut wtxn = self.env.write_txn().unwrap();

                    let data_hash = self.calculate_hash(&consume_data);
                    let key = format!(
                        "channel-{}-continuation-{}",
                        &receive.channels[i], &data_hash
                    );

                    let mut consume_data_buf = Vec::new();
                    consume_data_buf.reserve(consume_data.encoded_len());
                    consume_data.encode(&mut consume_data_buf).unwrap();

                    let _ = self.db.put(&mut wtxn, &key, &consume_data_buf);
                    wtxn.commit().unwrap();
                }

                None
            }
        } else {
            println!("channel and pattern vectors are not equal length!");
            None
        }
    }

    pub fn produce(&self, send: rtypes::Send, persistent: bool) -> Option<rtypes::OptionResult> {
        let rtxn = self.env.read_txn().unwrap();

        let continuation_prefix = format!("channel-{}-continuation", send.chan);
        let mut iter_continuation = self.db.prefix_iter(&rtxn, &continuation_prefix).unwrap();
        let mut iter_continuation_option = iter_continuation.next().transpose().unwrap();

        while iter_continuation_option.is_some() {
            let iter_data = iter_continuation_option.unwrap();
            let cdata_buf = iter_data.1;
            let cdata = rtypes::ConsumeData::decode(cdata_buf.as_slice()).unwrap();

            // TODO: Implement better pattern/match schema
            if cdata.pattern == send.match_case {
                if !cdata.persistent {
                    let mut wtxn = self.env.write_txn().unwrap();
                    let _ = self.db.delete(&mut wtxn, iter_data.0);
                    wtxn.commit().unwrap();
                }

                let mut option_result = rtypes::OptionResult::default();
                option_result.continuation = cdata.continuation.clone();
                option_result.data = send.data.clone();

                return Some(option_result);
            }
            iter_continuation_option = iter_continuation.next().transpose().unwrap();
        }
        drop(iter_continuation);
        rtxn.commit().unwrap();

        let mut produce_data = rtypes::ProduceData::default();
        produce_data.data = send.data.clone();
        produce_data.match_case = send.match_case.clone();
        produce_data.persistent = persistent;

        println!("\nNo matching continuation for {:?}", send);

        let mut wtxn = self.env.write_txn().unwrap();

        let data_hash = self.calculate_hash(&produce_data);
        let key = format!("channel-{}-data-{}", &send.chan, &data_hash);

        let mut produce_data_buf = Vec::new();
        produce_data_buf.reserve(produce_data.encoded_len());
        produce_data.encode(&mut produce_data_buf).unwrap();

        let _ = self.db.put(&mut wtxn, &key, &produce_data_buf);
        wtxn.commit().unwrap();

        None
    }

    pub fn print_channel(&self, channel: &str) -> Result<(), Box<dyn Error>> {
        let rtxn = self.env.read_txn()?;

        let continuation_prefix = format!("channel-{}-continuation", channel);
        let mut iter_continuation = self.db.prefix_iter(&rtxn, &continuation_prefix)?;

        let data_prefix = format!("channel-{}-data", channel);
        let mut iter_data = self.db.prefix_iter(&rtxn, &data_prefix)?;

        if !self.db.is_empty(&rtxn)? {
            println!("\nCurrent channel state for \"{}\":", channel);

            let mut iter_continuation_option = iter_continuation.next().transpose()?;
            while iter_continuation_option.is_some() {
                let key = iter_continuation_option.as_ref().unwrap().0;
                let cdata_buf = &iter_continuation_option.as_ref().unwrap().1;
                let cdata = rtypes::ConsumeData::decode(cdata_buf.as_slice()).unwrap();
                println!("KEY: {:?} VALUE: {:?}", key, cdata);
                iter_continuation_option = iter_continuation.next().transpose()?;
            }

            let mut iter_data_option = iter_data.next().transpose()?;
            while iter_data_option.is_some() {
                let key = iter_data_option.as_ref().unwrap().0;
                let pdata_buf = &iter_data_option.as_ref().unwrap().1;
                let pdata = rtypes::ProduceData::decode(pdata_buf.as_slice()).unwrap();
                println!("KEY: {:?} VALUE: {:?}", key, pdata);
                iter_data_option = iter_data.next().transpose()?;
            }
        } else {
            println!("\nDatabase is empty")
        }

        drop(iter_continuation);
        drop(iter_data);
        rtxn.commit()?;

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        let rtxn = self.env.read_txn().unwrap();
        return self.db.is_empty(&rtxn).unwrap();
    }

    pub fn clear(&self) -> Result<(), Box<dyn Error>> {
        let mut wtxn = self.env.write_txn()?;
        let _ = self.db.clear(&mut wtxn)?;
        wtxn.commit()?;

        Ok(())
    }

    fn calculate_hash<T: Hash>(&self, t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}
