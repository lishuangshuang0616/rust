#![deny(
    missing_docs,
    missing_copy_implementations,
    non_upper_case_globals,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]


#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate metric_derive;

#[cfg(test)]
#[macro_use]
extern crate proptest;

use ahash::AHasher;
use anyhow::{Context, Error};
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Borrow;
use std::collections::{hash_map, HashMap, HashSet};
use std::fs::File;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};
use std::io::{Read, Write};
use std::iter::FromIterator;
use std::path::Path;

pub mod count_metric;
pub use crate::count_metric::CountMetric;
pub mod percent_metric;
pub use crate::percent_metric::PercentMetric;
pub mod histogram;
pub use crate::histogram::SimpleHistogram;
pub mod mean_metric;
pub use crate::mean_metric::MeanMetric;
pub mod collections;
pub mod num;
pub mod option;


pub struct TxHasher(AHasher);
impl Hasher for TxHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0.finish()
    }
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }
}

impl Default for TxHasher {
    fn default() -> Self {
        const PI: [u64; 4] = [
            0x243f_6a88_85a3_08d3,
            0x1319_8a2e_0370_7344,
            0xa409_3822_299f_31d0,
            0x082e_fa98_ec4e_6c89,
        ];

        const PI2: [u64; 4] = [
            0x4528_6e02_2f83_b9c5,
            0xbebf_b14c_a9a5_49f7,
            0xc6a5_fe4a_58b5_c2a2,
            0x2a2b_3b2b_eb4f_d8b5,
        ];

        const SEED: [u64; 4] = [
            PI[0] ^ PI2[0] ^ 0x6c62_272e_07bb_0142,
            PI[1] ^ PI2[1],
            PI[2] ^ PI2[2] ^ 0x517c_c1b7_2722_0a95,
            PI[3] ^ PI2[3],
        ];
        
        TxHasher(ahash::RandomState::with_seeds(SEED[0], SEED[1], SEED[2], SEED[3]).build_hasher())
    }
}

pub type TxBuildHasher = BuildHasherDefault<TxHasher>;
pub type TxHashMap<K, V> = HashMap<K, V, TxBuildHasher>;
pub type TxHashSet<T> = HashSet<T, TxBuildHasher>;

