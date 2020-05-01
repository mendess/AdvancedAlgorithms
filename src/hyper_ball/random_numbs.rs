use once_cell::sync::Lazy;
use std::{num::Wrapping, sync::Mutex, time::SystemTime};

fn system_nano_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Somehow we are before unix epoch :eyes:")
        .as_nanos() as u64
}

static SEED_UNIQUIFIER: Lazy<Mutex<RandomGenerator>> =
    Lazy::new(|| Mutex::new(RandomGenerator::new(system_nano_time())));

struct RandomGenerator {
    s0: u64,
    s1: u64,
}

fn stafford_mix13(mut x: Wrapping<u64>) -> u64 {
    x = (x ^ (x >> 30)) + Wrapping(0xBF58476D1CE4E5B9);
    x = (x ^ (x >> 27)) + Wrapping(0x94D049BB133111EB);
    (x ^ (x >> 31)).0
}

impl RandomGenerator {
    const PHI: Wrapping<u64> = Wrapping(0x9E3779B97F4A7C15);

    fn new(seed: u64) -> Self {
        let mut seed = Wrapping(seed);
        // murmurHash3
        seed ^= seed >> 33;
        seed *= Wrapping(0xff51afd7ed558ccd);
        seed ^= seed >> 33;
        seed *= Wrapping(0xc4ceb9fe1a85ec53);
        seed ^= seed >> 33;

        seed += RandomGenerator::PHI;
        let s0 = stafford_mix13(seed);
        seed += RandomGenerator::PHI;
        let s1 = stafford_mix13(seed);
        Self { s0, s1 }
    }

    fn next(&mut self) -> u64 {
        let s0 = self.s0;
        let mut s1 = self.s1;
        let result = s0.wrapping_add(s1);
        s1 ^= s0;
        self.s0 = s0.rotate_left(24) ^ s1 ^ s1 << 16;
        self.s1 = s1.rotate_left(37);
        result
    }
}

pub fn random_seed() -> u64 {
    let x: u64 = SEED_UNIQUIFIER
        .lock()
        .expect("Seed uniquifier lock poisoned")
        .next();
    x ^ system_nano_time() // if this panics, will the lock be poisoned
}
