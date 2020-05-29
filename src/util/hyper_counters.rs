pub mod compact_hyperloglog;
pub mod hyperloglog;

pub use compact_hyperloglog::{array::CompactHyperLogLogArray, CompactHyperLogLog};
pub use hyperloglog::HyperLogLog;

use std::ops::DerefMut;

pub trait HyperLogLogCounter<T> {
    fn register(&mut self, t: T);
    fn estimate(&self) -> f64;
    fn union_onto(&self, other: &mut Self) -> bool;
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum B {
    B4 = 4,
    B5 = 5,
    B6 = 6,
    B7 = 7,
    B8 = 8,
    B9 = 9,
    B10 = 10,
    B11 = 11,
    B12 = 12,
    B13 = 13,
    B14 = 14,
    B15 = 15,
}

impl B {
    fn alpha(self) -> f64 {
        match self {
            B::B4 => 0.673,
            B::B5 => 0.697,
            B::B6 => 0.709,
            _ => 0.7213 / (1.0 + 1.079 / self.m() as f64),
        }
    }

    fn m(self) -> usize {
        1usize << self
    }
}

macro_rules! sh_b {
    ($($t:ty),*) => {
        $(
        impl ::std::ops::Shl<B> for $t {
            type Output = $t;
            fn shl(self, rhs: B) -> Self::Output {
                self << rhs as u8
            }
        }

        impl ::std::ops::Shr<B> for $t {
            type Output = $t;
            fn shr(self, rhs: B) -> Self::Output {
                self >> rhs as u8
            }
        }
        )*
    };
}
sh_b!(u8, u16, u32, u64, usize);

pub trait CounterArray<T>
where
    Self: Clone,
    Self: DerefMut<Target = [<Self as CounterArray<T>>::Counter]>,
{
    type Counter: HyperLogLogCounter<T> + Clone;

    fn union_onto(&mut self, from: usize, other: &mut Self::Counter) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn equivalence() {
        let seed = crate::util::random_numbs::random_seed();
        let mut h1 = HyperLogLog::new_with_seed(B::B4, seed);
        let mut h2 = CompactHyperLogLog::new_with_seed(B::B4, 10_000, seed);
        for i in 0..10_000 {
            let value = rand::random::<i32>();
            h1.register(value);
            h2.register(value);
            assert_eq!(
                h1.state(),
                h2.state(),
                "Equivalence test failed at i = {}",
                i
            );
        }
    }

    #[test]
    fn equivalence_after_union() {
        let seed = crate::util::random_numbs::random_seed();
        let mut h1s = vec![HyperLogLog::new_with_seed(B::B4, seed); 16];
        let mut h2s = vec![CompactHyperLogLog::new_with_seed(B::B4, 10_000, seed)];
        for i in 0..10_000 {
            let value = rand::random::<i32>();
            for (h1, h2) in h1s.iter_mut().zip(h2s.iter_mut()) {
                h1.register(value);
                h2.register(value);
                assert_eq!(
                    h1.state(),
                    h2.state(),
                    "Equivalence test failed at i = {}",
                    i
                );
            }
        }
        for i in 0..500 {
            let (h1, t) = h1s.split_at_mut(1);
            for w in t {
                w.union_onto(&mut h1[0]);
            }
            let (h2, t) = h2s.split_at_mut(1);
            for w in t {
                w.union_onto(&mut h2[0]);
            }
            assert_eq!(
                h1[0].state(),
                h2[0].state(),
                "Equivalence test failed at i = {}",
                i
            );
        }
    }
}
