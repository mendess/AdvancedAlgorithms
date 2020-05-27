use super::{To, Graph, WEdge, WFromEdges, WMutable};
use itertools::Itertools;
use std::{
    fmt::{self, Debug},
    ops::Index,
};

type Neighbours<E> = Vec<To<E>>;

#[derive(Clone)]
pub struct Adjacency<E = ()> {
    matrix: Vec<Neighbours<E>>,
    n_edges: usize,
}

impl<E> Graph for Adjacency<E> {
    type EdgeWeight = E;
    fn vertices(&self) -> usize {
        self.matrix.len()
    }

    fn edges(&self) -> usize {
        self.n_edges
    }
}

impl<E> WFromEdges for Adjacency<E> {
    fn from_edges<I, Iter>(n: usize, list: I) -> Self
    where
        I: IntoIterator<IntoIter = Iter, Item = WEdge<E>>,
        Iter: ExactSizeIterator<Item = WEdge<E>>,
    {
        let edges = list.into_iter();
        let mut s = Self {
            matrix: Vec::with_capacity(n),
            n_edges: 0,
        };
        edges.for_each(|(from, to, w)| {
            s.add_weighed_link(from, to, w);
        });
        s
    }
}

impl<E> Adjacency<E> {
    pub fn new() -> Self {
        Self {
            matrix: Default::default(),
            n_edges: 0,
        }
    }

    pub fn with_capacity(n: usize) -> Self {
        Self {
            matrix: Vec::with_capacity(n),
            n_edges: 0,
        }
    }

    pub fn neighbours(&self, node: usize) -> impl Iterator<Item = &To<E>> {
        self.matrix[node].iter()
    }

    pub fn neighbourhoods(&self) -> impl Iterator<Item = (usize, impl Iterator<Item = &To<E>>)> {
        self.matrix
            .iter()
            .enumerate()
            .map(|(start, neigh)| (start, neigh.iter()))
    }

    pub fn has_link(&self, from: usize, to: usize) -> bool {
        self.matrix[from].iter().any(|n| n.to == to)
    }

    pub fn add_vertex(&mut self, from: usize) {
        self.matrix.resize_with(from + 1, Default::default);
    }
}

impl<E> WMutable for Adjacency<E> {
    fn add_weighed_link(&mut self, from: usize, to: usize, weight: E) -> bool {
        match self.matrix.get_mut(from) {
            Some(neigh) => neigh.push(To { to, weight }),
            None => {
                self.matrix.resize_with(from + 1, Default::default);
                self.matrix[from].push(To { to, weight });
            }
        }
        self.n_edges += 1;
        true
    }
}

impl<E> Index<usize> for Adjacency<E> {
    type Output = Neighbours<E>;
    fn index(&self, u: usize) -> &Self::Output {
        &self.matrix[u]
    }
}

impl Debug for Adjacency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.neighbourhoods()
            .try_for_each(|(i, s)| writeln!(f, "{:?}: {:?}", i, s.format(" -> ")))
    }
}
