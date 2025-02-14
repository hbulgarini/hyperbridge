#![cfg(test)]
#![allow(unused_parens)]

pub mod abi;
mod forge;
mod tests;

pub use crate::forge::{execute, runner};
pub use ethers::{abi::Token, types::U256, utils::keccak256};
// use ismp::mmr::{DataOrHash, MmrHasher};
use merkle_mountain_range::{util::MemMMR, Error, Merge};
use rs_merkle::Hasher;

#[derive(Clone)]
pub struct Keccak256;

impl Hasher for Keccak256 {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        keccak256(data)
    }
}

impl ismp::util::Keccak256 for Keccak256 {
    fn keccak256(bytes: &[u8]) -> H256
    where
        Self: Sized,
    {
        keccak256(bytes).into()
    }
}

struct MergeKeccak;

impl Merge for MergeKeccak {
    type Item = NumberHash;
    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Result<Self::Item, Error> {
        let mut concat = vec![];
        concat.extend(&lhs.0);
        concat.extend(&rhs.0);
        let hash = keccak256(&concat);
        Ok(NumberHash(hash.to_vec().into()))
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
struct NumberHash(pub Vec<u8>);

impl From<u32> for NumberHash {
    fn from(num: u32) -> Self {
        let hash = keccak256(&num.to_le_bytes());
        NumberHash(hash.to_vec())
    }
}

// pub fn unwrap_hash(item: &DataOrHash) -> [u8; 32] {
//     match item {
//         DataOrHash::Hash(h) => (*h).into(),
//         _ => panic!("not a hash"),
//     }
// }

use primitive_types::H256;

// pub type Mmr = MemMMR<DataOrHash, MmrHasher<Keccak256>>;
