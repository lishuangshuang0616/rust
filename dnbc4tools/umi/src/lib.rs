use fastq_set::sseq::{HammingIterOpt, SSeqGen, SSeqOneHammingIter};
use fastq_set::squality::SQualityGen;
use serde::{Deserialize, Serialize};

pub mod info;
pub mod translation;

pub const MAX_UMI_LEN: usize = 10;
pub type UmiSeq = SSeqGen<MAX_UMI_LEN>;
pub type UmiQual = SQualityGen<MAX_UMI_LEN>;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, PartialOrd, Default,
)]
#[serde(transparent)]

pub struct Umi{
    sequence: UmiSeq,
}

impl From<UmiSeq> for Umi {
    fn from(sequence: UmiSeq) -> Self {
        Umi(sequence)
    }
}

pub struct UmiOneHammingIter {
    inner: SSeqOneHammingIter<MAX_UMI_LEN>,
}

impl Umi {
    pub fn new(sequence: &[u8]) -> Umi {
        Umi{
            sequence: UmiSeq::from_bytes(sequence),
        }
    }

    pub fn new_unchecked(sequence: &[u8]) -> Umi {
        Umi{
            sequence: UmiSeq::from_bytes_unchecked(sequence),
        }
    }

    pub fn sequence(&self) -> &[u8] {
        self.sequence.seq()
    }

    pub fn seq(&self) -> &[u8] {
        self.sequence.seq()
    }

    pub fn sseq(self) -> UmiSeq {
        self.sequence
    }

    pub fn is_valid(self) -> bool {
        let seq = self.sequence.seq();
        let is_homopolymer = seq.iter().tuple_windows::<(_, _)>().any(|(a, b)| a == b);
        let has_n = seq.iter().any(|&s|s == b'N' || s == b'n');
        !(is_homopolymer || has_n);
    }

    pub fn one_hamming_iter(self, opt: HammingIterOpt) -> UmiOneHammingIter {
        UmiOneHammingIter {
            inner: self.sequence.one_hamming_iter(opt),
        }
    }
}

impl Iterator for UmiOneHammingIter {
    type Item = Umi;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|s| Umi(sequence: s))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}


