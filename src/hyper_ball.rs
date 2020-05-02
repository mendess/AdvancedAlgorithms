pub mod hyper_counters;
pub mod random_numbs;

use crate::graphs::{csr::CSR, *};
use hyper_counters::{HyperLogLog, B};

struct HyperBall {
    counters: Box<[HyperLogLog<usize>]>,
}

impl HyperBall {
    fn new(n: usize, b: B) -> Self {
        Self {
            counters: vec![HyperLogLog::new(b); n].into(),
        }
    }

    #[inline]
    fn union(&mut self, a: usize, b: usize) -> bool {
        if a == b {
            return false;
        }
        let a_ptr = &mut self.counters[a] as *mut HyperLogLog<usize>;
        let b_ptr = &mut self.counters[b] as *mut HyperLogLog<usize>;
        unsafe { (*a_ptr).union_onto(&mut *b_ptr) }
    }
}

pub fn hyper_ball(g: &CSR) -> f64 {
    let mut ball = HyperBall::new(g.vertices(), B::B4);
    for v in g.nodes() {
        ball.counters[v].register(v);
    }
    let mut modified = true;
    while modified {
        modified = false;
        for (v, successors) in g.neighbourhoods().enumerate() {
            for &w in successors {
                modified = bool::max(modified, ball.union(v, w));
            }
            // TODO: Calc something here
        }
    }
    0.0
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphs::{csr::CSR, test_graphs::graph_one};

    #[test]
    fn run() {
        let g = graph_one::<CSR>();
        let apl = hyper_ball(&g);
        eprintln!("APL: {}", apl);
        // assert!(3. < apl && apl < 4.);
    }
}
