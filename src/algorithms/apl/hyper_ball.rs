use crate::{
    graphs::{csr::CSR, *},
    util::hyper_counters::{CounterArray, HyperLogLogCounter},
};

#[derive(Clone)]
struct HyperBall<H: CounterArray<usize>> {
    counters: H,
}

impl<H: CounterArray<usize> + Clone> HyperBall<H> {
    fn new(counters: H) -> Self {
        Self { counters }
    }
}

pub fn hyper_ball<H, E>(g: &CSR<E>, counters: H) -> f64
where
    H: CounterArray<usize> + Clone,
{
    let mut ball = HyperBall::new(counters);
    for v in g.nodes() {
        ball.counters[v].register(v);
    }
    let mut apls = vec![0.0; g.vertices()].into_boxed_slice();
    let mut modified = true;
    let mut t = 0;
    let mut new_counters = ball.counters.clone();
    while modified {
        modified = false;
        for (v, successors) in g.neighbourhoods().enumerate() {
            new_counters[v].clone_from(&ball.counters[v]);
            let a = &mut new_counters[v];
            for w in successors {
                modified = bool::max(ball.counters[w.to].union_onto(a), modified);
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
    use super::*;
    use crate::{
        graphs::{
            csr::CSR,
            test_graphs::{graph_one, GRAPH_ONE_APL},
        },
        util::hyper_counters::*,
    };
    const SEED: u64 = 0xBAD5EED;

    #[test]
    fn run_normal() {
        let g = graph_one::<CSR<()>>();
        let apl = hyper_ball(
            &g,
            vec![HyperLogLog::new_with_seed(B::B4, SEED); g.vertices()].into_boxed_slice(),
        );
        eprintln!("APL: {}", apl);
        approx::assert_relative_eq!(apl, GRAPH_ONE_APL, max_relative = 1.0);
    }

    #[test]
    fn run_compact() {
        let g = graph_one::<CSR<()>>();
        let apl = hyper_ball(&g, CompactHyperLogLogArray::new(B::B4, g.vertices(), SEED));
        eprintln!("APL: {}", apl);
        approx::assert_relative_eq!(apl, GRAPH_ONE_APL, max_relative = 1.0);
    }

    #[test]
    fn equivalence() {
        let g = graph_one::<CSR<()>>();
        assert_eq!(
            hyper_ball(
                &g,
                vec![HyperLogLog::new_with_seed(B::B4, SEED); g.vertices()].into_boxed_slice(),
            ),
            hyper_ball(&g, CompactHyperLogLogArray::new(B::B4, g.vertices(), SEED))
        )
    }
}
