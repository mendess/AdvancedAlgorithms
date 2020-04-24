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

pub fn graph_one<G: Graph<NodeId = N>, N: Vertex>() -> G {
    graph!(G = (10) {
       N::from(0) => N::from(1);
       N::from(0) => N::from(2);
       N::from(0) => N::from(3);
       N::from(0) => N::from(4);
       N::from(1) => N::from(0);
       N::from(1) => N::from(2);
       N::from(1) => N::from(3);
       N::from(1) => N::from(4);
       N::from(2) => N::from(0);
       N::from(2) => N::from(1);
       N::from(2) => N::from(3);
       N::from(2) => N::from(4);
       N::from(3) => N::from(0);
       N::from(3) => N::from(1);
       N::from(3) => N::from(2);
       N::from(3) => N::from(4);
       N::from(4) => N::from(0);
       N::from(4) => N::from(1);
       N::from(4) => N::from(2);
       N::from(4) => N::from(3);

       N::from(5) => N::from(6);
       N::from(5) => N::from(7);
       N::from(5) => N::from(8);
       N::from(5) => N::from(9);
       N::from(6) => N::from(5);
       N::from(6) => N::from(7);
       N::from(6) => N::from(8);
       N::from(6) => N::from(9);
       N::from(7) => N::from(5);
       N::from(7) => N::from(6);
       N::from(7) => N::from(8);
       N::from(7) => N::from(9);
       N::from(8) => N::from(5);
       N::from(8) => N::from(6);
       N::from(8) => N::from(7);
       N::from(8) => N::from(9);
       N::from(9) => N::from(5);
       N::from(9) => N::from(6);
       N::from(9) => N::from(7);
       N::from(9) => N::from(8);
       // the min cut
       N::from(2) => N::from(6);
       N::from(4) => N::from(5);
       N::from(3) => N::from(7);
    })
}

pub fn random_graph<G, N, R>(n: N, m: usize, mut rng: R) -> G
where
    G: Graph<NodeId = N>,
    N: Vertex + SampleUniform,
    R: Rng,
{
    let mut edges = Vec::with_capacity(m);
    let dist = Uniform::from(N::from(0)..n);
    let mut set = HashSet::with_capacity_and_hasher(m, Default::default());
    while edges.len() < edges.capacity() {
        let a0 = dist.sample(&mut rng);
        let a1 = dist.sample(&mut rng);
        if a0 != a1 && set.insert((a0, a1)) {
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
    let mut edges = Vec::with_capacity(m);
    let dist = Uniform::from(0..n);
    let mut set = HashSet::with_capacity_and_hasher(m, Default::default());
    while edges.len() < edges.capacity() {
        let a0 = dist.sample(&mut rng);
        let a1 = dist.sample(&mut rng);
        if a0 != a1 && set.insert((a0, a1)) {
            edges.push((a0, a1))
        }
    }

    G::new(n, edges.into_iter())
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
