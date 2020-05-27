pub mod disjoint_set;
pub mod bit_array;
pub mod random_numbs;
pub mod hyper_counters;

pub struct ExactSizeIter<I> {
    pub iter: I,
    pub size: usize,
}

impl<I: Iterator> Iterator for ExactSizeIter<I> {
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match (self.size, self.iter.next()) {
            (0, _) => None,
            (_, None) => panic!("Passed iterator was smaller than expected"),
            (_, i) => {
                self.size -= 1;
                i
            }
        }
    }
}

impl<I: Iterator> ExactSizeIterator for ExactSizeIter<I> {
    #[inline]
    fn len(&self) -> usize {
        self.size
    }
}

pub trait ToExactSizeIter: Iterator + Sized {
    #[inline]
    fn to_exact_size(self, size: usize) -> ExactSizeIter<Self> {
        ExactSizeIter { iter: self, size }
    }
}

impl<I: Iterator> ToExactSizeIter for I {}

pub fn jenkins(x: u64, seed: u64) -> u64 {
    use std::num::Wrapping;
    let mut a = Wrapping(seed.wrapping_add(x));
    let mut b = Wrapping(seed);
    let mut c = Wrapping(0x9e3779b97f4a7c13); /* the golden ratio; an arbitrary value */
    a -= b;
    a -= c;
    a ^= c >> 43;
    b -= c;
    b -= a;
    b ^= a << 9;
    c -= a;
    c -= b;
    c ^= b >> 8;
    a -= b;
    a -= c;
    a ^= c >> 38;
    b -= c;
    b -= a;
    b ^= a << 23;
    c -= a;
    c -= b;
    c ^= b >> 5;
    a -= b;
    a -= c;
    a ^= c >> 35;
    b -= c;
    b -= a;
    b ^= a << 49;
    c -= a;
    c -= b;
    c ^= b >> 11;
    a -= b;
    a -= c;
    a ^= c >> 12;
    b -= c;
    b -= a;
    b ^= a << 18;
    c -= a;
    c -= b;
    c ^= b >> 22;
    c.0
}

