use super::{Graph, ToExactSizeIter, Vertex};
use crate::graph;
use itertools::Itertools;
use rand::{
    distributions::{uniform::SampleUniform, Distribution, Uniform},
    Rng,
};
use rand_distr::Binomial;
use std::{
    collections::HashSet,
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

pub fn random_graph<G, N, R>(n: N, m: usize, rng: R) -> G
where
    G: Graph<NodeId = N>,
    N: Vertex + SampleUniform,
    R: Rng,
{
    let mut set = HashSet::new();
    G::new(
        n.into(),
        Uniform::from(N::from(0)..n)
            .sample_iter(rng)
            .chunks(2)
            .into_iter()
            .map(|mut a| {
                let a0 = a.next().unwrap();
                let a1 = a.next().unwrap();
                (a0, a1)
            })
            .filter(|a| set.insert(*a))
            .take(m)
            .to_exact_size(m),
    )
}

pub fn random_graph_concrete<G, R>(n: usize, m: usize, rng: R) -> G
where
    G: Graph<NodeId = usize>,
    R: Rng,
{
    let mut set = HashSet::new();
    G::new(
        n.into(),
        Uniform::from(0..n)
            .sample_iter(rng)
            .chunks(2)
            .into_iter()
            .map(|mut a| {
                let a0 = a.next().unwrap();
                let a1 = a.next().unwrap();
                (a0, a1)
            })
            .filter(|a| set.insert(*a))
            .take(m)
            .to_exact_size(m),
    )
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
