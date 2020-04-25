use super::{Edge, FromEdges, Graph};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug},
    ops::Index,
};

type Neighbours<N> = HashSet<N>;

#[derive(Clone)]
pub struct Adjacency {
    matrix: HashMap<usize, Neighbours<usize>>,
    n_edges: usize,
    n_vertices: usize,
}

impl Graph for Adjacency {
    fn vertices(&self) -> usize {
        self.n_vertices
    }

    fn edges(&self) -> usize {
        self.n_edges
    }
}

impl FromEdges for Adjacency {
    fn from_edges<I>(n: usize, edges: I) -> Self
    where
        I: ExactSizeIterator<Item = Edge>,
    {
        let mut s = Self {
            matrix: Default::default(),
            n_edges: edges.len(),
            n_vertices: n,
        };
        for i in 0..n {
            s.matrix.insert(i, Default::default());
        }
        edges.for_each(|(from, to, (), ())| {
            s.add_link(from, to);
        });
        s
    }
}

impl Adjacency {
    pub fn new() -> Self {
        Self {
            matrix: HashMap::default(),
            n_edges: 0,
            n_vertices: 0,
        }
    }
    pub fn add_link(&mut self, from: usize, to: usize) -> bool {
        match self.matrix.get_mut(&from) {
            Some(neigh) => neigh.insert(to),
            None => self
                .matrix
                .insert(from, {
                    let mut m = HashSet::with_capacity(1);
                    m.insert(to);
                    m
                })
                .is_some(),
        }
    }

    pub fn contract(&mut self, (start, end, (), ()): Edge) {
        let old_neigh = self.matrix.remove(&end).expect("End doesn't exist");
        if let Some(neigh) = self.matrix.get_mut(&start) {
            neigh.extend(old_neigh.iter().filter(|&n| *n != start));
        }
        for end1 in old_neigh {
            if let Some(ends) = self.matrix.get_mut(&end1) {
                ends.remove(&end1);
                ends.insert(start);
            }
        }
    }

    pub fn neighbours(&self, node: usize) -> impl Iterator<Item = &usize> {
        self.matrix[&node].iter()
    }

    pub fn neighbourhoods(&self) -> impl Iterator<Item = (&usize, impl Iterator<Item = &usize>)> {
        self.matrix
            .iter()
            .map(|(start, neigh)| (start, neigh.iter()))
    }
}

impl Adjacency {}

impl Index<usize> for Adjacency {
    type Output = Neighbours<usize>;
    fn index(&self, u: usize) -> &Self::Output {
        &self.matrix[&u]
    }
}

impl Debug for Adjacency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.neighbourhoods()
            .try_for_each(|(i, s)| writeln!(f, "{:?}: {:?}", i, s.format(" -> ")))
    }
}
