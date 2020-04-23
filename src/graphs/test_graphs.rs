use super::{Graph, Vertex};
use crate::graph;
use rand::{
    distributions::{uniform::SampleUniform, Distribution, Uniform},
    Rng,
};
use rand_distr::Binomial;
use rustc_hash::FxHashSet as HashSet;
use std::{
    convert::TryInto,
    ops::{Div, Mul, Sub},
};

pub fn graph_one<G: Graph<NodeId = usize>>() -> G {
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

pub fn random_graph<G, N, R>(n: N, m: usize, mut rng: R) -> G
where
    G: Graph<NodeId = N>,
    N: Vertex + SampleUniform,
    R: Rng,
{
    let mut set: HashSet<_> = Default::default();
    let mut edges = Vec::with_capacity(m);
    let dist = Uniform::from(N::from(0)..n);
    while edges.len() < n.into() {
        let a0 = dist.sample(&mut rng);
        let a1 = dist.sample(&mut rng);
        if set.insert((a0, a1)) {
            edges.push((a0, a1))
        }
    }

    G::new(n.into(), edges.into_iter())
}

pub fn random_graph_concrete<G, R>(n: usize, m: usize, mut rng: R) -> G
where
    G: Graph<NodeId = usize>,
    R: Rng,
{
    let mut set: HashSet<_> = Default::default();
    let mut edges = Vec::with_capacity(m);
    let dist = Uniform::from(0..n);
    while edges.len() < n {
        let a0 = dist.sample(&mut rng);
        let a1 = dist.sample(&mut rng);
        if set.insert((a0, a1)) {
            edges.push((a0, a1))
        }
    }

    G::new(n.into(), edges.into_iter())
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
pub fn random_graph_er<G, N, R>(n: N, p: f64, mut rng: R) -> G
where
    R: Rng,
    G: Graph<NodeId = N>,
    N: Vertex + SampleUniform,
    N: Sub,
    N: Mul<<N as Sub>::Output>,
    <N as Mul<<N as Sub>::Output>>::Output: Div<N>,
    <<N as Mul<<N as Sub>::Output>>::Output as Div<N>>::Output: TryInto<u64>,
    <<<N as Mul<<N as Sub>::Output>>::Output as Div<N>>::Output as TryInto<u64>>::Error:
        std::fmt::Debug,
{
    let dist = Binomial::new((n * (n - N::from(1)) / N::from(2)).try_into().unwrap(), p).unwrap();
    random_graph(n, (dist.sample(&mut rng)).try_into().unwrap(), rng)
}

pub fn random_graph_er_concrete<G, R>(n: usize, p: f64, mut rng: R) -> G
where
    R: Rng,
    G: Graph<NodeId = usize>,
{
    let dist = Binomial::new((n * (n - 1) / 2).try_into().unwrap(), p).unwrap();
    random_graph(n, (dist.sample(&mut rng)).try_into().unwrap(), rng)
}
