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
    let mut apls = vec![0.0; g.vertices()].into_boxed_slice();
    let mut modified = true;
    let mut t = 0;
    let mut new_counters: Box<[H]> = ball.counters.clone();
    while modified {
        modified = false;
        for (v, successors) in g.neighbourhoods().enumerate() {
            new_counters[v].clone_from(&ball.counters[v]);
            let a = &mut new_counters[v];
            for &w in successors {
                modified = bool::max(ball.counters[w].union_onto(a), modified);
            }
            apls[v] += t as f64 * (a.estimate() - ball.counters[v].estimate());
        }
        new_counters
            .iter_mut()
            .zip(ball.counters.iter_mut())
            .for_each(|(new, old)| std::mem::swap(new, old));
        t += 1;
    }
    apls.iter().sum::<f64>() / apls.len() as f64
}

#[cfg(test)]
mod tests {
    use super::{hyper_counters::*, *};
    use crate::graphs::{csr::CSR, test_graphs::{GRAPH_ONE_APL, graph_one}};
    const SEED: u64 = 0xBAD5EED;

    #[test]
    fn run_normal() {
        let g = graph_one::<CSR>();
        let apl = hyper_ball(&g, || HyperLogLog::new_with_seed(B::B4, SEED));
        eprintln!("APL: {}", apl);
        approx::assert_relative_eq!(apl, GRAPH_ONE_APL, max_relative = 1.0);
    }

    #[test]
    fn run_compact() {
        let g = graph_one::<CSR>();
        let apl = hyper_ball(&g, || {
            CompactHyperLogLog::new_with_seed(B::B4, g.vertices(), SEED)
        });
        eprintln!("APL: {}", apl);
        approx::assert_relative_eq!(apl, GRAPH_ONE_APL, max_relative = 1.0);
    }

    #[test]
    fn equivalence() {
        let g = graph_one::<CSR>();
        assert_eq!(
            hyper_ball(&g, || HyperLogLog::new_with_seed(B::B4, SEED)),
            hyper_ball(&g, || CompactHyperLogLog::new_with_seed(
                B::B4,
                g.vertices(),
                SEED
            ))
        )
    }
}
