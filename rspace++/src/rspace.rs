use crate::diskconc::DiskConcDB;
use crate::diskseq::DiskSeqDB;
use crate::memconc::MemConcDB;
use crate::memseq::MemSeqDB;
use crate::shared::{OptionResult, Pattern};
use std::error::Error;

// See https://docs.google.com/document/d/1yWdvJwsq4Ft7elzKBM0dehh4RFoQ-vXt-1TAUTLLxMY/edit
#[repr(C)]
pub struct RSpace<D, K> {
    diskseq: DiskSeqDB<D, K>,
    diskconc: DiskConcDB<D, K>,
    memseq: MemSeqDB<D, K>,
    memconc: MemConcDB<D, K>,
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
    > RSpace<D, K>
{
    pub fn create() -> Result<RSpace<D, K>, Box<dyn Error>> {
        let ds = DiskSeqDB::create().unwrap();
        let dc = DiskConcDB::create().unwrap();
        let ms = MemSeqDB::create().unwrap();
        let mc = MemConcDB::create().unwrap();

        Ok(RSpace {
            diskseq: ds,
            diskconc: dc,
            memseq: ms,
            memconc: mc,
        })
    }

    // Verb Set 1
    pub fn get_once_durable_concurrent(
        &self,
        channel: &str,
        entry: D,
    ) -> Option<OptionResult<D, K>> {
        return self.diskconc.produce(channel, entry, false);
    }

    pub fn get_once_non_durable_concurrent(
        &self,
        channel: &str,
        entry: D,
    ) -> Option<OptionResult<D, K>> {
        return self.memconc.produce(channel, entry, false);
    }

    pub fn get_once_durable_sequential(
        &self,
        channel: &str,
        entry: D,
    ) -> Option<OptionResult<D, K>> {
        return self.diskseq.produce(channel, entry, false);
    }

    pub fn get_once_non_durable_sequential(
        &self,
        channel: &str,
        entry: D,
    ) -> Option<OptionResult<D, K>> {
        return self.memseq.produce(channel, entry, false);
    }

    // Verb Set 2
    pub fn get_always_durable_concurrent(
        &self,
        channel: &str,
        entry: D,
    ) -> Option<OptionResult<D, K>> {
        return self.diskconc.produce(channel, entry, true);
    }

    pub fn get_always_non_durable_concurrent(
        &self,
        channel: &str,
        entry: D,
    ) -> Option<OptionResult<D, K>> {
        return self.memconc.produce(channel, entry, true);
    }

    pub fn get_always_durable_sequential(
        &self,
        channel: &str,
        entry: D,
    ) -> Option<OptionResult<D, K>> {
        return self.diskseq.produce(channel, entry, true);
    }

    pub fn get_always_non_durable_sequential(
        &self,
        channel: &str,
        entry: D,
    ) -> Option<OptionResult<D, K>> {
        return self.memseq.produce(channel, entry, true);
    }

    // Verb Set 3
    pub fn put_once_durable_concurrent(
        &self,
        channels: Vec<&str>,
        patterns: Vec<Pattern<D>>,
        continuation: K,
    ) -> Option<Vec<OptionResult<D, K>>> {
        return self
            .diskconc
            .consume(channels, patterns, continuation, false);
    }

    pub fn put_once_non_durable_concurrent(
        &self,
        channels: Vec<&str>,
        patterns: Vec<Pattern<D>>,
        continuation: K,
    ) -> Option<Vec<OptionResult<D, K>>> {
        return self
            .memconc
            .consume(channels, patterns, continuation, false);
    }

    pub fn put_once_durable_sequential(
        &self,
        channels: Vec<&str>,
        patterns: Vec<Pattern<D>>,
        continuation: K,
    ) -> Option<Vec<OptionResult<D, K>>> {
        return self
            .diskseq
            .consume(channels, patterns, continuation, false);
    }

    pub fn put_once_non_durable_sequential(
        &self,
        channels: Vec<&str>,
        patterns: Vec<Pattern<D>>,
        continuation: K,
    ) -> Option<Vec<OptionResult<D, K>>> {
        return self.memseq.consume(channels, patterns, continuation, false);
    }

    // Verb Set 4
    pub fn put_always_durable_concurrent(
        &self,
        channels: Vec<&str>,
        patterns: Vec<Pattern<D>>,
        continuation: K,
    ) -> Option<Vec<OptionResult<D, K>>> {
        return self
            .diskconc
            .consume(channels, patterns, continuation, true);
    }

    pub fn put_always_non_durable_concurrent(
        &self,
        channels: Vec<&str>,
        patterns: Vec<Pattern<D>>,
        continuation: K,
    ) -> Option<Vec<OptionResult<D, K>>> {
        return self.memconc.consume(channels, patterns, continuation, true);
    }

    pub fn put_always_durable_sequential(
        &self,
        channels: Vec<&str>,
        patterns: Vec<Pattern<D>>,
        continuation: K,
    ) -> Option<Vec<OptionResult<D, K>>> {
        return self.diskseq.consume(channels, patterns, continuation, true);
    }

    pub fn put_always_non_durable_sequential(
        &self,
        channels: Vec<&str>,
        patterns: Vec<Pattern<D>>,
        continuation: K,
    ) -> Option<Vec<OptionResult<D, K>>> {
        return self.memseq.consume(channels, patterns, continuation, true);
    }

    pub fn print_data(
        &self,
        channel: &str,
    ) -> ()
    {
        let _ = self.memseq.print_channel(channel);
        let _ = self.memconc.print_channel(channel);
        let _ = self.diskseq.print_channel(channel);
        let _ = self.diskconc.print_channel(channel);
    }
}
