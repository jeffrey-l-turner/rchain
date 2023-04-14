use std::error::Error;
use crate::diskseq::DiskSeqDB;
use crate::memconc::MemConcDB;
use crate::memseq::MemSeqDB;

pub struct RSpace<D, K> {
    diskseq: DiskSeqDB<D,K>,
    memseq: MemSeqDB<D,K>,
    memconc: MemConcDB<D,K>,
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
        let ms = MemSeqDB::create().unwrap();
        let mc = MemConcDB::create().unwrap();

        Ok(RSpace {
            diskseq: ds,
            memseq: ms,
            memconc: mc,
        })
    }

    pub fn process_command() {

    }

}