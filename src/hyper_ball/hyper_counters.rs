pub mod compact_hyperloglog;
pub mod hyperloglog;

pub use compact_hyperloglog::CompactHyperLogLog;
pub use hyperloglog::HyperLogLog;

pub trait HyperLogLogCounter<T> {
    fn register(&mut self, t: T);
    fn estimate(&self) -> f64;
    fn union_onto(&mut self, other: &Self) -> bool;
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn equivalence() {
        let seed = crate::util::random_numbs::random_seed();
        let mut h1 = HyperLogLog::new_with_seed(B::B4, seed);
        let mut h2 = CompactHyperLogLog::new_with_seed(B::B4, 1_0000, seed);
        for i in 0..1_0000 {
            h1.register(i);
            h2.register(i);
            assert_eq!(h1.state(), h2.state(), "Equivalence test failed at i = {}", i);
        }
    }
}
