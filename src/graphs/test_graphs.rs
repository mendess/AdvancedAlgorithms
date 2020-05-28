use super::{FromEdges, Mutable, RandomAccess, To, WMutable};
use crate::graph;
use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
    Rng,
};
use rand_distr::Binomial;
use rustc_hash::FxHashSet as HashSet;
use std::convert::TryInto;

pub const GRAPH_ONE_MIN_CUT: [(usize, usize); 3] = [(2_usize, 6_usize), (3, 7), (4, 5)];
pub const GRAPH_ONE_APL: f64 = 3.1555555555555554;
pub fn graph_one<G: FromEdges<EdgeWeight = ()>>() -> G {
    graph!(G = (10) {
       0 => 1;
       0 => 2;
       0 => 3;
       0 => 4;
       1 => 0;
       1 => 2;
       1 => 3;
       1 => 4;
       2 => 0;
       2 => 1;
       2 => 3;
       2 => 4;
       3 => 0;
       3 => 1;
       3 => 2;
       3 => 4;
       4 => 0;
       4 => 1;
       4 => 2;
       4 => 3;

       5 => 6;
       5 => 7;
       5 => 8;
       5 => 9;
       6 => 5;
       6 => 7;
       6 => 8;
       6 => 9;
       7 => 5;
       7 => 6;
       7 => 8;
       7 => 9;
       8 => 5;
       8 => 6;
       8 => 7;
       8 => 9;
       9 => 5;
       9 => 6;
       9 => 7;
       9 => 8;
       // the min cut
       2 => 6;
       4 => 5;
       3 => 7;
    })
}

pub fn random_graph<R>(n: usize, m: usize, mut rng: R) -> Vec<(usize, usize)>
where
    R: Rng,
{
    let dist = Uniform::from(0..n);
    let mut edges = Vec::with_capacity(m);
    let mut set = HashSet::with_capacity_and_hasher(m, Default::default());
    while edges.len() < edges.capacity() {
        let a0 = dist.sample(&mut rng);
        let a1 = dist.sample(&mut rng);
        if a0 != a1 && set.insert((a0, a1)) {
            edges.push((a0, a1))
        }
    }
    edges.sort_unstable();
    edges
}

/// Generates a graph using the Evdos Ronmi method.
///
/// ```norun
/// G(N,P) where
///
///        (N)(N - 1)
/// E[M] = ---------- P
///            2
///
/// G(N,M) <=> G(N,P)
/// ```
pub fn random_graph_er<R>(n: usize, p: f64, mut rng: R) -> Vec<(usize, usize)>
where
    R: Rng,
{
    let dist = Binomial::new((n * (n - 1) / 2).try_into().unwrap(), p).unwrap();
    random_graph(n, (dist.sample(&mut rng)).try_into().unwrap(), rng)
}

pub fn clustered<G, R>(n: usize, d: usize, o: usize, mut rng: R) -> G
where
    R: Rng,
    G: WMutable + Mutable + RandomAccess + FromEdges,
{
    assert!(n > 2);
    assert!(d > 1);
    let mut g = graph![G = (n) { 0 => 1 }];
    let mut vertices = Vec::with_capacity(n);
    vertices.push(0);
    vertices.push(1);
    for v in 2..n {
        vertices.push(v);
        g.add_vertex(v);
        for _ in 0..usize::min(v, d) {
            let u = loop {
                let u = *vertices
                    .choose_weighted(&mut rng, |&i| g.neighbours(i).len())
                    .unwrap();
                if !g.has_link(v, u) {
                    break u;
                }
            };
            g.add_link(v, u);
        }
        for _ in 0..o {
            let (&u, &w) = g
                .neighbours(v)
                .choose(&mut rng)
                .and_then(|To { to: u, .. }| loop {
                    let w = &g.neighbours(v).choose(&mut rng).unwrap().to;
                    if u != w {
                        break Some((u, w));
                    }
                })
                .unwrap();
            if !g.has_link(u, w) {
                g.add_link(u, w);
            }
        }
    }
    g
}
