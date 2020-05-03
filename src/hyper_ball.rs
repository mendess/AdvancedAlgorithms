pub mod hyper_counters;

use crate::graphs::{csr::CSR, *};
use hyper_counters::HyperLogLogCounter;

#[derive(Clone)]
struct HyperBall<H: HyperLogLogCounter<usize>> {
    counters: Box<[H]>,
}

impl<H: HyperLogLogCounter<usize> + Clone> HyperBall<H> {
    fn new<F>(n: usize, f: F) -> Self
    where
        F: Fn() -> H,
    {
        Self {
            counters: vec![f(); n].into(),
        }
    }

    #[inline]
    fn union(&mut self, a: usize, b: usize) -> bool {
        if a == b {
            return false;
        }
        let a_ptr = &mut self.counters[a] as *mut H;
        let b_ptr = &mut self.counters[b] as *mut H;
        unsafe { (*a_ptr).union_onto(&mut *b_ptr) }
    }
}

pub fn hyper_ball<F, H>(g: &CSR, f: F) -> f64
where
    H: HyperLogLogCounter<usize> + Clone,
    F: Fn() -> H,
{
    let mut ball = HyperBall::new(g.vertices(), f);
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
    use super::{hyper_counters::*, *};
    use crate::graphs::{csr::CSR, test_graphs::graph_one};

    #[test]
    fn run_normal() {
        let g = graph_one::<CSR>();
        let apl = hyper_ball(&g, || HyperLogLog::new(B::B4));
        eprintln!("APL: {}", apl);
        // assert!(3. < apl && apl < 4.);
    }

    #[test]
    fn run_compact() {
        let g = graph_one::<CSR>();
        let apl = hyper_ball(&g, || CompactHyperLogLog::new(B::B4, g.vertices()));
        eprintln!("APL: {}", apl);
        // assert!(3. < apl && apl < 4.);
    }
}
