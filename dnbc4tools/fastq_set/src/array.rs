use serde::de::{self, Visitor};
use std::marker::PhantomData;




pub trait ArrayContent {
    fn validate_bytes(bytes: &[u8]);
    fn expected_contents() -> &'static str;
}

#[derive(Clone, Copy, PartialOrd, Ord, Eq)]
pub struct ByteArray<T, const N: usize>
where
    T: ArrayContent,
{
    bytes: [u8; N],
    length: u8,
    phantom: std::marker::PhantomData<T>,
}

impl<T, const N: usize> ByteArray<T, N>
where
    T: ArrayContent,
{
    pub fn new() -> Self {
        ByteArray {
            bytes: [0; N],
            length: 0,
            phantom: std::marker::PhantomData,
        }
    }

    
}