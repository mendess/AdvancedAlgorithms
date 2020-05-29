use super::{
    super::{CounterArray, HyperLogLogCounter, B},
    CompactHyperLogLog,
};
use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

pub struct CompactHyperLogLogArray<T> {
    counters: Box<[CompactHyperLogLog<T>]>,
    //max_aux_bufs: MaxAuxBuffers,
}

impl<T> CompactHyperLogLogArray<T>
where
    T: Hash + Clone,
{
    pub fn new(b: B, vertices: usize, seed: u64) -> Self {
        Self {
            counters: vec![CompactHyperLogLog::new_with_seed(b, vertices, seed); vertices]
                .into_boxed_slice(),
            // max_aux_bufs: MaxAuxBuffers::new(vertices),
        }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
struct MaxAuxBuffers {
    acc: Box<[u8]>,
    mask: Box<[u8]>,
    msb_mask: Box<[u8]>,
}

impl MaxAuxBuffers {
    #[allow(dead_code)]
    fn new(len: usize) -> Self {
        Self {
            acc: vec![0; len].into(),
            mask: vec![0; len].into(),
            msb_mask: vec![0; len].into(),
        }
    }
}

impl<T> Deref for CompactHyperLogLogArray<T> {
    type Target = [CompactHyperLogLog<T>];
    fn deref(&self) -> &Self::Target {
        &self.counters
    }
}

impl<T> DerefMut for CompactHyperLogLogArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.counters
    }
}

impl<T> CounterArray<T> for CompactHyperLogLogArray<T>
where
    T: Hash + Clone,
{
    type Counter = CompactHyperLogLog<T>;

    #[inline]
    fn union_onto(&mut self, from: usize, other: &mut Self::Counter) -> bool {
        self[from].union_onto(other)
    }
}

impl<T> Clone for CompactHyperLogLogArray<T> {
    fn clone(&self) -> Self {
        Self {
            counters: self.counters.clone(),
        }
    }

    fn clone_from(&mut self, other: &Self) {
        self.counters.clone_from(&other.counters);
    }
}
