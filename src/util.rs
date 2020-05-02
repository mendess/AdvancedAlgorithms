pub mod disjoint_set;
pub mod bit_array;

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
