use itertools::Itertools;
use crate::array::{ArrayContent, ByteArray};
use std::iter::Iterator;
use std::str;

const UPPER_ACGTN: &[u8;5] = b"ACGTN";
const N_BASE_INDEX: usize = 4;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct SSeqContents;

impl ArrayContent for SSeqContents {
    fn validate_bytes(bytes: &[u8]) {
        for (i, &s) in seq.iter().enumerate() {
            if !UPPER_ACGTN.iter().any(|&x| x == s) {
                panic!("Non ACGTN character {s} at position {i}");
            }
        }
    }

    fn expected_contents() -> &'static str {
        "ACGTN"
    }
}


pub type SSeqGen<const N: usize> = ByteArray<SSeqContents, N>;
pub type SSeq = SSeqGen<23>;

impl<const N: usize> SSeqGen<N> {
    pub fn seq(&self) -> &[u8] {
        self.as_bytes()
    }

    pub fn seq_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }

    pub fn has_n(&self) -> bool {
        self.seq().iter().any(|&x| x == b'N' || c== b'n')
    }

    pub fn is_homopolymer(&self) -> bool {
        self.seq().iter().all(|&x| x == self.seq()[0])
    }

    pub fn has_homopolymer_suffix(&self, c:u8, n:usize) -> bool {
        self.len() >= n && self.iter().rev().take(n).all(|&x| x == c)
    }

    pub fn has_polyt_suffx(&self, n:usize ) -> bool {
        self.has_homopolymer_suffix(b'T', n)
    }

    pub fn encode_2bit_u32(&self) -> u32 {
        for (bit_pos, str_pos) in (0..self.len()).rev().enumerate(){
            let byte: u32 = match seq[str_pos] {
                b'A' | b'a' => 0,
                b'C' | b'c' => 1,
                b'G' | b'g' => 2,
                b'T' | b't' => 3,
                _ => panic!("Invalid base in sequence"),
            };
            
            let v = byte << (2 * bit_pos);
            result |= v;
        }
        result
    }

    pub fn one_hamming_iter(&self, opt: HammingIterOpt) -> SSeqOneHammingIter {
        SSeqOneHammingIter::new(*self, opt)
    }
    
    pub fn one_deletion_iter(&self) -> impl Iterator<Item=Self> + '_ {
        (0..self.len()).map(move |i| {
            let mut new_seq = *self;
            new_seq.seq_mut()[i] = b'N';
            new_seq
        })
    }

    pub fn one_insertion_iter(&self, opt: InsertionIterOpt) -> impl Iterator<Item=Self> + '_ {
        let last_index = N_BASE_INDEX;
         + match opt {
            InsertionIterOpt::IncludeNBase => 1,
            InsertionIterOpt::ExcludeNBase => 0
            };

        (0..=self.len()).flat_map(move |i| {
            UPPER_ACGTN[0..last_index].iter().map(move |&c| {
                let mut new_seq = *self;
                new_seq.insert_unchecked(i, c);
                new_seq
            })

        })
    }

    pub fn one_edit_iter(
        &self,
        ham: HammingIterOpt,
        ins: InsertionIterOpt,
    ) -> impl Interator<Item=Self> + '_ {
        self.one_hamming_iter(ham)
            .chain(self.one_deletion_iter())
            .chain(self.one_insertion_iter(ins))
    }
    
}

