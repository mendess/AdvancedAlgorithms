use super::{HyperLogLogCounter, B};
use crate::util::{bit_array::BitArray, jenkins, random_numbs};
use rustc_hash::FxHasher;
use std::{
    convert::*,
    f64,
    hash::{BuildHasher, BuildHasherDefault, Hash, Hasher},
    marker::PhantomData,
};

const TWO_POW_32: f64 = (1u64 << 32) as f64;

#[derive(Clone)]
pub struct CompactHyperLogLog<T, H = BuildHasherDefault<FxHasher>> {
    registers: BitArray,
    build_hasher: H,
    b: B,
    m_minus_1: u64,
    alpha_mm: f64,
    seed: u64,
    _marker: PhantomData<T>,
}

fn register_size(n: usize) -> u8 {
    u8::max(
        5,
        f64::ceil(f64::ln(f64::ln(n as f64) / f64::consts::LN_2) / f64::consts::LN_2) as u8,
    )
}

impl<T> CompactHyperLogLog<T, BuildHasherDefault<FxHasher>> {
    pub fn new(b: B, expected_elements: usize) -> Self {
        Self::new_with_hasher(b, Default::default(), expected_elements)
    }

    pub fn new_with_seed(b: B, expected_elements: usize, seed: u64) -> Self {
        Self::new_with_hasher_and_seed(b, Default::default(), expected_elements, seed)
    }
}

impl<T, H> CompactHyperLogLog<T, H> {
    pub fn new_with_hasher_and_seed(
        b: B,
        build_hasher: H,
        expected_elements: usize,
        seed: u64,
    ) -> Self {
        let m = b.m();
        Self {
            b,
            m_minus_1: m as u64 - 1,
            build_hasher,
            alpha_mm: b.alpha() * (m * m) as f64,
            registers: BitArray::new(register_size(expected_elements), m),
            seed,
            _marker: PhantomData,
        }
    }

    pub fn new_with_hasher(b: B, build_hasher: H, expected_elements: usize) -> Self {
        Self::new_with_hasher_and_seed(
            b,
            build_hasher,
            expected_elements,
            random_numbs::random_seed(),
        )
    }

    pub fn state(&self) -> Box<[u8]> {
        (0..self.registers.len())
            .map(|i| self.registers.get(i))
            .collect::<Vec<_>>()
            .into()
    }
}

impl<T, H> HyperLogLogCounter<T> for CompactHyperLogLog<T, H>
where
    T: Hash,
    H: BuildHasher,
{
    fn register(&mut self, v: T) {
        let mut hasher = self.build_hasher.build_hasher();
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
        assert!(
            r < (1 << self.registers.register_size()) - 1,
            "Max reg size {0} ({0:08b}), but r is = {1} ({1:08b})",
            (1 << self.registers.register_size()) - 1,
            r
        );
        let max = u8::max(self.registers.get(j), (r + 1) as u8);
        self.registers.set(j, max)
    }

    fn estimate(&self) -> f64 {
        let m = self.registers.len() as f64;
        let e = self.alpha_mm / {
            let e = self
                .registers
                .iter()
                .map(|j| 1.0 / f64::from(1u32 << j)) // 1 << j == 2 ^ j
                .sum::<f64>();
            e
        };
        if e <= (5.0 / 2.0) * m {
            let n_eq_0 = self.registers.iter().filter(|x| *x == 0).count();
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

    #[inline]
    fn union_onto(&mut self, other: &Self) -> bool {
        self.registers.max(&other.registers)
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
            let mut log = CompactHyperLogLog::new_with_seed(b, n, 0x156820568);
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
