use crate::graphs::{matrix::Adjacency, To};
use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
    Rng,
};

pub fn c_coef<R>(k: i32, node_indexes: &[usize], g: &Adjacency, mut rng: R) -> f64
where
    R: Rng,
{
    if node_indexes.is_empty() {
        return 0.0;
    }
    let indices = Uniform::from(0..node_indexes.len());
    let mut l = 0i32;
    for _ in 0..k {
        let j = node_indexes[indices.sample(&mut rng)];
        let (u, w) = g[j]
            .choose(&mut rng)
            .and_then(|To { to: u, .. }| loop {
                let w = &g[j].choose(&mut rng)?.to;
                if w != u {
                    break Some((u, w));
                }
            })
            .unwrap_or_else(|| panic!("Node '{}' with outdgree = {} < 2", j, g[j].len()));
        if g.has_link(*u, *w) {
            l += 1
        }
    }
    l as f64 / k as f64
}
