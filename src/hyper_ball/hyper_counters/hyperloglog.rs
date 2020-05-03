use super::{HyperLogLogCounter, B};
use crate::util::{jenkins, random_numbs};
use rustc_hash::FxHasher;
use std::{
    hash::{BuildHasher, BuildHasherDefault, Hash, Hasher},
    marker::PhantomData,
    ops::RangeInclusive,
};

const TWO_POW_32: f64 = (1u64 << 32) as f64;

/// Parameters
/// - Word size [16, 32, 64]
/// - b
/// - m = 2^b
#[derive(Clone, Debug)]
pub struct HyperLogLog<T, H = BuildHasherDefault<FxHasher>> {
    registers: Box<[u8]>,
    m_minus_1: u64,
    b: B,
    hasher: H,
    alpha_mm: f64,
    seed: u64,
    _marker: PhantomData<T>,
}

impl<T> HyperLogLog<T, BuildHasherDefault<FxHasher>> {
    pub fn new(b: B) -> Self {
        Self::new_with_hasher(b, Default::default())
    }

    pub fn new_with_seed(b: B, seed: u64) -> Self {
        Self::new_with_hasher_and_seed(b, Default::default(), seed)
    }
}

impl<T, H> HyperLogLog<T, H> {
    pub fn new_with_hasher(b: B, hasher: H) -> Self {
        Self::new_with_hasher_and_seed(b, hasher, random_numbs::random_seed())
    }

    pub fn new_with_hasher_and_seed(b: B, hasher: H, seed: u64) -> Self {
        let m = b.m();
        Self {
            b,
            m_minus_1: m as u64 - 1,
            hasher,
            alpha_mm: b.alpha() * (m * m) as f64,
            registers: vec![0; m].into(),
            seed,
            _marker: PhantomData,
        }
    }

    pub fn error(&self) -> (f64, f64) {
        (
            -1.04 / f64::sqrt(self.registers.len() as f64),
            1.04 / f64::sqrt(self.registers.len() as f64),
        )
    }

    pub fn state(&self) -> Box<[u8]> {
        self.registers.clone()
    }
}

impl<T: Hash, H: BuildHasher> HyperLogLog<T, H> {
    pub fn estimate_range(&self) -> RangeInclusive<f64> {
        let e = self.estimate();
        let error = self.error();
        (e + error.0)..=(e + error.1)
    }
}

impl<T: Hash, H: BuildHasher> HyperLogLogCounter<T> for HyperLogLog<T, H> {
    fn register(&mut self, v: T) {
        let mut hasher = self.hasher.build_hasher();
        v.hash(&mut hasher);
        let x = jenkins(hasher.finish(), self.seed);
        // The original algorithm specifies that the first b bits of the
        // hashed value are to be used for indexing. Because it's simpler to use
        // the last b bits instead, and the hash function is "random" anyway,
        // we can instead look at the hash value in reverse.
        let j = (x & self.m_minus_1) as usize;
        // Removing the trailing b bytes that have already been considered
        // And using the position of the right most 1-bit, counting from the end.
        let r = u64::trailing_zeros(x >> self.b);
        self.registers[j] = self.registers[j].max((r + 1) as u8);
    }

    /// Estimate the cardinality of the tracked multi set
    ///
    /// The estimate is has an error of ±1.04/√m
    ///
    /// For a range of possible values, taking this error into account,
    /// see [`HyperLogLog::estimate_range`].
    fn estimate(&self) -> f64 {
        let m = self.registers.len() as f64;
        let e = self.alpha_mm / {
            let e = self
                .registers
                .iter()
                .map(|&j| 1.0 / f64::from(1u32 << j)) // 1 << j == 2 ^ j
                .sum::<f64>();
            e
        };
        if e <= (5.0 / 2.0) * m {
            let n_eq_0 = self.registers.iter().filter(|x| **x == 0).count();
            if n_eq_0 != 0 {
                m * f64::ln(m / n_eq_0 as f64)
            } else {
                e
            }
        } else if e <= (1.0 / 30.0) * TWO_POW_32 {
            e
        } else {
            -TWO_POW_32 * f64::log2(1.0 - e / TWO_POW_32)
        }
    }

    fn union_onto(&mut self, other: &Self) -> bool {
        self.registers
            .iter_mut()
            .zip(other.registers.iter())
            .any(|(a, b)| {
                let old_a = *a;
                *a = u8::max(*a, *b);
                old_a != *a
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    const ESTIMATE_SIZE: usize = 1;

    macro_rules! make_test {
        ($($b:expr),*) => {
            paste::item! {
            $(
            #[test]
            fn [<test_100k_b $b>]() {
                const N: usize = 100 * 1000;
                const N_SKEW: usize = N / 16;
                let diffs = run(N, B::[<B $b>]);
                let avg = print_diffs(N, &diffs);
                assert!(avg >= (N - N_SKEW) as f64 && avg <= (N + N_SKEW) as f64,
                    "Expected a value between {} and {} but got {}",
                    N - N_SKEW,
                    N + N_SKEW,
                    avg);
            }
            )*
            }
        };
    }

    // make_test!(4, 5, 6, 7, 8);
    make_test!(9, 10, 11, 12, 13, 14, 15);

    #[cfg(test)]
    fn print_diffs(n: usize, d: &[f64]) -> f64 {
        eprintln!("For N = {}", n);
        if d.len() > 3 {
            eprintln!(
                "[{} ... {}]",
                &d[..3]
                    .iter()
                    .format_with(", ", |d, f| f(&format_args!("{:.2}", d))),
                &d[(d.len() - 3)..]
                    .iter()
                    .format_with(", ", |d, f| f(&format_args!("{:.2}", d))),
            );
        } else {
            eprintln!(
                "[{}]",
                d.iter()
                    .format_with(", ", |d, f| f(&format_args!("{:.2}", d)))
            )
        }
        let avg = d.iter().sum::<f64>() / d.len() as f64;
        eprintln!("avg: {}", avg);
        avg
    }

    #[cfg(test)]
    fn run(n: usize, b: B) -> [f64; ESTIMATE_SIZE] {
        let mut diffs = [0.; ESTIMATE_SIZE];
        diffs.iter_mut().for_each(|i| {
            let mut log = HyperLogLog::new_with_seed(b, 0x156820568);
            for i in 0..n {
                log.register(i);
                //eprintln!("inserting: {} est: {:?}", i, log.estimate_range());
            }
            let est = log.estimate();
            *i = est;
        });
        diffs.sort_by_key(|f| (f * 10000f64) as isize);
        diffs
    }
}
