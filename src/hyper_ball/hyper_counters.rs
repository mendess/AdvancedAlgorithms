use rustc_hash::FxHasher;
use std::{
    hash::{BuildHasher, BuildHasherDefault, Hash, Hasher},
    marker::PhantomData,
    ops::RangeInclusive,
};

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

const TWO_POW_32: f64 = (1u64 << 32) as f64; //4294967296.0;

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
}

impl<T, H> HyperLogLog<T, H> {
    pub fn new_with_hasher(b: B, hasher: H) -> Self {
        let m = b.m();
        Self {
            b,
            m_minus_1: m as u64 - 1,
            hasher,
            alpha_mm: b.alpha() * (m * m) as f64,
            registers: vec![0; m].into(),
            seed: super::random_numbs::random_seed(),
            _marker: PhantomData,
        }
    }

    /// Estimate the cardinality of the tracked multi set
    ///
    /// The estimate is has an error of ±1.04/√m
    ///
    /// For a range of possible values, taking this error into account,
    /// see [`HyperLogLog::estimate_range`].
    pub fn estimate(&self) -> f64 {
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

    pub fn estimate_range(&self) -> RangeInclusive<f64> {
        let e = self.estimate();
        let error = self.error();
        (e + error.0)..=(e + error.1)
    }

    pub fn error(&self) -> (f64, f64) {
        (
            -1.04 / f64::sqrt(self.registers.len() as f64),
            1.04 / f64::sqrt(self.registers.len() as f64),
        )
    }

    pub fn union_onto(&self, other: &mut Self) -> bool {
        other
            .registers
            .iter_mut()
            .zip(self.registers.iter())
            .any(|(a, b)| {
                let old_a = *a;
                *a = u8::max(*a, *b);
                old_a != *a
            })
    }
}

impl<T: Hash, H: BuildHasher> HyperLogLog<T, H> {
    pub fn register(&mut self, v: T) {
        let mut hasher = self.hasher.build_hasher();
        v.hash(&mut hasher);
        let x = jenkins(hasher.finish(), self.seed);
        // The original algorithm specifies that the first b bits of the
        // hashed value are to be used for indexing. Because it's simpler to use
        // the last b bits instead, and the hash function is "random" anyway,
        // we can instead look at the hash value in reverse.
        let j = (x & self.m_minus_1) as usize;
        // Removing the trailing b bytes that have already been considered
        let r = u64::trailing_zeros(x >> self.b);
        // And using the position of the right most 1-bit, counting from the end.
        self.registers[j] = self.registers[j].max((r + 1) as u8);
    }
}

fn jenkins(x: u64, seed: u64) -> u64 {
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
                let diffs = run(N, B::[<B $b>]);
                let avg = print_diffs(N, &diffs);
                assert!(avg >= (N - N / 16) as f64 && avg <= (N + N / 16) as f64);
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
            let mut log = HyperLogLog::new(b);
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
