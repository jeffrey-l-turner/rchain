use crate::shared::*;
use heed::types::*;
use heed::{Database, Env, EnvOpenOptions};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fs;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::path::Path;

/*
See RSpace.scala and Tuplespace.scala in rspace/
*/
pub struct DiskSeqDB<D, K> {
    env: Env,
    db: Database<Str, SerdeBincode<Vec<u8>>>,
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
        channels: Vec<&str>,
        patterns: Vec<Pattern<D>>,
        continuation: K,
        persist: bool,
    ) -> Option<Vec<OptionResult<D, K>>> {
        if channels.len() == patterns.len() {
            let mut results: Vec<OptionResult<D, K>> = vec![];
            let rtxn = self.env.read_txn().unwrap();

            for i in 0..channels.len() {
                let data_prefix = format!("channel-{}-data", channels[i]);
                let mut iter_data = self.db.prefix_iter(&rtxn, &data_prefix).unwrap();
                let mut iter_data_option = iter_data.next().transpose().unwrap();

                while iter_data_option.is_some() {
                    let iter_data_unwrap = iter_data_option.unwrap();
                    let data_bytes = iter_data_unwrap.1;
                    let produce_data: ProduceData<D> =
                        bincode::deserialize::<ProduceData<D>>(&data_bytes).unwrap();

                    if patterns[i](produce_data.data.clone()) {
                        if !produce_data.persist {
                            let mut wtxn = self.env.write_txn().unwrap();
                            let _ = self.db.delete(&mut wtxn, iter_data_unwrap.0);
                            wtxn.commit().unwrap();
                        }

                        results.push(OptionResult {
                            continuation: continuation.clone(),
                            data: produce_data.data,
                        });
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
                for i in 0..channels.len() {
                    let k_data = KData {
                        pattern: patterns[i],
                        continuation: continuation.clone(),
                        persist,
                    };

                    println!("\nNo matching data for {:?}", k_data);

                    let k_data_bytes = bincode::serialize(&k_data).unwrap();

                    // opening a write transaction
                    let mut wtxn = self.env.write_txn().unwrap();

                    let kdata_hash = self.calculate_hash(&k_data);
                    let key = format!("channel-{}-continuation-{}", &channels[i], &kdata_hash);

                    let _ = self.db.put(&mut wtxn, &key, &k_data_bytes);
                    wtxn.commit().unwrap();
                }

                None
            }
        } else {
            println!("channel and pattern vectors are not equal length!");
            None
        }
    }

    pub fn produce(&self, channel: &str, entry: D, persist: bool) -> Option<OptionResult<D, K>> {
        let rtxn = self.env.read_txn().unwrap();

        let continuation_prefix = format!("channel-{}-continuation", channel);
        let mut iter_continuation = self.db.prefix_iter(&rtxn, &continuation_prefix).unwrap();
        let mut iter_continuation_option = iter_continuation.next().transpose().unwrap();

        while iter_continuation_option.is_some() {
            let iter_data = iter_continuation_option.unwrap();
            let k_data_bytes = iter_data.1;
            let k_data: KData<Pattern<D>, K> =
                bincode::deserialize::<KData<Pattern<D>, K>>(&k_data_bytes).unwrap();
            let pattern = k_data.pattern;

            if pattern(entry.clone()) {
                if !k_data.persist {
                    let mut wtxn = self.env.write_txn().unwrap();
                    let _ = self.db.delete(&mut wtxn, iter_data.0);
                    wtxn.commit().unwrap();
                }

                return Some(OptionResult {
                    continuation: k_data.continuation,
                    data: entry.clone(),
                });
            }
            iter_continuation_option = iter_continuation.next().transpose().unwrap();
        }
        drop(iter_continuation);
        rtxn.commit().unwrap();

        let produce_data = ProduceData {
            data: entry.clone(),
            persist,
        };

        println!("\nNo matching continuation for {:?}", produce_data);

        let mut wtxn = self.env.write_txn().unwrap();

        let data_hash = self.calculate_hash(&produce_data);
        let key = format!("channel-{}-data-{}", &channel, &data_hash);
        let data_bytes = bincode::serialize(&produce_data).unwrap();

        let _ = self.db.put(&mut wtxn, &key, &data_bytes);
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
                let k_data_bytes = &iter_continuation_option.as_ref().unwrap().1;
                let key = iter_continuation_option.as_ref().unwrap().0;
                let k_data = bincode::deserialize::<KData<Pattern<D>, K>>(&k_data_bytes).unwrap();
                println!("KEY: {:?} VALUE: {:?}", key, k_data);
                iter_continuation_option = iter_continuation.next().transpose()?;
            }

            let mut iter_data_option = iter_data.next().transpose()?;
            while iter_data_option.is_some() {
                let data_bytes = &iter_data_option.as_ref().unwrap().1;
                let key = iter_data_option.as_ref().unwrap().0;
                let data = bincode::deserialize::<D>(&data_bytes).unwrap();
                println!("KEY: {:?} VALUE: {:?}", key, data);
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
> MyTrait<D, K> for DiskSeqDB<D, K> {
    fn my_method(&mut self) {
        // implementation for MemSeqDB's my_method
        println!("DiskSeqDB my_method")
    }
    // implement more methods/functions here
    fn consume(
        &self,
        channels: Vec<&str>,
        patterns: Vec<Pattern<D>>,
        continuation: K,
        persist: bool,
    ) -> Option<Vec<OptionResult<D, K>>> {
        if channels.len() == patterns.len() {
            let mut results: Vec<OptionResult<D, K>> = vec![];
            let rtxn = self.env.read_txn().unwrap();

            for i in 0..channels.len() {
                let data_prefix = format!("channel-{}-data", channels[i]);
                let mut iter_data = self.db.prefix_iter(&rtxn, &data_prefix).unwrap();
                let mut iter_data_option = iter_data.next().transpose().unwrap();

                while iter_data_option.is_some() {
                    let iter_data_unwrap = iter_data_option.unwrap();
                    let data_bytes = iter_data_unwrap.1;
                    let produce_data: ProduceData<D> =
                        bincode::deserialize::<ProduceData<D>>(&data_bytes).unwrap();

                    if patterns[i](produce_data.data.clone()) {
                        if !produce_data.persist {
                            let mut wtxn = self.env.write_txn().unwrap();
                            let _ = self.db.delete(&mut wtxn, iter_data_unwrap.0);
                            wtxn.commit().unwrap();
                        }

                        results.push(OptionResult {
                            continuation: continuation.clone(),
                            data: produce_data.data,
                        });
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
                for i in 0..channels.len() {
                    let k_data = KData {
                        pattern: patterns[i],
                        continuation: continuation.clone(),
                        persist,
                    };

                    println!("\nNo matching data for {:?}", k_data);

                    let k_data_bytes = bincode::serialize(&k_data).unwrap();

                    // opening a write transaction
                    let mut wtxn = self.env.write_txn().unwrap();

                    let kdata_hash = self.calculate_hash(&k_data);
                    let key = format!("channel-{}-continuation-{}", &channels[i], &kdata_hash);

                    let _ = self.db.put(&mut wtxn, &key, &k_data_bytes);
                    wtxn.commit().unwrap();
                }

                None
            }
        } else {
            println!("channel and pattern vectors are not equal length!");
            None
        }
    }

    fn produce(&self, channel: &str, entry: D, persist: bool) -> Option<OptionResult<D, K>> {
        let rtxn = self.env.read_txn().unwrap();

        let continuation_prefix = format!("channel-{}-continuation", channel);
        let mut iter_continuation = self.db.prefix_iter(&rtxn, &continuation_prefix).unwrap();
        let mut iter_continuation_option = iter_continuation.next().transpose().unwrap();

        while iter_continuation_option.is_some() {
            let iter_data = iter_continuation_option.unwrap();
            let k_data_bytes = iter_data.1;
            let k_data: KData<Pattern<D>, K> =
                bincode::deserialize::<KData<Pattern<D>, K>>(&k_data_bytes).unwrap();
            let pattern = k_data.pattern;

            if pattern(entry.clone()) {
                if !k_data.persist {
                    let mut wtxn = self.env.write_txn().unwrap();
                    let _ = self.db.delete(&mut wtxn, iter_data.0);
                    wtxn.commit().unwrap();
                }

                return Some(OptionResult {
                    continuation: k_data.continuation,
                    data: entry.clone(),
                });
            }
            iter_continuation_option = iter_continuation.next().transpose().unwrap();
        }
        drop(iter_continuation);
        rtxn.commit().unwrap();

        let produce_data = ProduceData {
            data: entry.clone(),
            persist,
        };

        println!("\nNo matching continuation for {:?}", produce_data);

        let mut wtxn = self.env.write_txn().unwrap();

        let data_hash = self.calculate_hash(&produce_data);
        let key = format!("channel-{}-data-{}", &channel, &data_hash);
        let data_bytes = bincode::serialize(&produce_data).unwrap();

        let _ = self.db.put(&mut wtxn, &key, &data_bytes);
        wtxn.commit().unwrap();

        None
    }

    fn clear(&self) -> Result<(), Box<dyn Error>> {
        let mut wtxn = self.env.write_txn()?;
        let _ = self.db.clear(&mut wtxn)?;
        wtxn.commit()?;

        Ok(())
    }

    fn is_empty(&self) -> bool {
        let rtxn = self.env.read_txn().unwrap();
        return self.db.is_empty(&rtxn).unwrap();
    }

    fn print_channel(&self, channel: &str) -> Result<(), Box<dyn Error>> {
        let rtxn = self.env.read_txn()?;

        let continuation_prefix = format!("channel-{}-continuation", channel);
        let mut iter_continuation = self.db.prefix_iter(&rtxn, &continuation_prefix)?;

        let data_prefix = format!("channel-{}-data", channel);
        let mut iter_data = self.db.prefix_iter(&rtxn, &data_prefix)?;

        if !self.db.is_empty(&rtxn)? {
            println!("\nCurrent channel state for \"{}\":", channel);

            let mut iter_continuation_option = iter_continuation.next().transpose()?;
            while iter_continuation_option.is_some() {
                let k_data_bytes = &iter_continuation_option.as_ref().unwrap().1;
                let key = iter_continuation_option.as_ref().unwrap().0;
                let k_data = bincode::deserialize::<KData<Pattern<D>, K>>(&k_data_bytes).unwrap();
                println!("KEY: {:?} VALUE: {:?}", key, k_data);
                iter_continuation_option = iter_continuation.next().transpose()?;
            }

            let mut iter_data_option = iter_data.next().transpose()?;
            while iter_data_option.is_some() {
                let data_bytes = &iter_data_option.as_ref().unwrap().1;
                let key = iter_data_option.as_ref().unwrap().0;
                let data = bincode::deserialize::<D>(&data_bytes).unwrap();
                println!("KEY: {:?} VALUE: {:?}", key, data);
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
}
