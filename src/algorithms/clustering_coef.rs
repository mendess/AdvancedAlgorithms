use crate::graphs::{RandomAccess, To};
use rand::{seq::SliceRandom, Rng};

pub fn c_coef<G, R>(k: i32, node_indexes: &[usize], g: &G, mut rng: R) -> f64
where
    R: Rng,
    G: RandomAccess,
{
    if node_indexes.is_empty() {
        return 0.0;
    }
    let mut l = 0i32;
    for _ in 0..k {
        let j = *node_indexes.choose(&mut rng).unwrap();
        let (u, w) = g
            .neighbours(j)
            .choose(&mut rng)
            .and_then(|To { to: u, .. }| match &g.neighbours(j) {
                [w, _] if w != u => Some((*u, w.to)),
                [_, w] if w != u => Some((*u, w.to)),
                a => loop {
                    let w = &a.choose(&mut rng)?.to;
                    if w != u {
                        return Some((*u, *w));
                    }
                },
            })
            .unwrap_or_else(|| {
                panic!("Node '{}' with outdgree = {} < 2", j, g.neighbours(j).len())
            });
        if g.has_link(u, w) {
            l += 1
        }
    }
    l as f64 / k as f64
}
