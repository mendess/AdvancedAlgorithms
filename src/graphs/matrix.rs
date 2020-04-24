use super::{ContractableGraph, Graph, MutableGraph, Vertex};
use itertools::Itertools;
use rand::{seq::IteratorRandom, Rng};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug},
    ops::Index,
};

type Neighbours<N> = HashSet<N>;

#[derive(Clone)]
pub struct Adjacency<N = usize> {
    matrix: HashMap<N, Neighbours<N>>,
    missing_links: usize,
    n_edges: usize,
    n_vertices: usize,
}

type NodeId<N> = <Adjacency<N> as Graph>::NodeId;
type Edge<N> = super::Edge<NodeId<N>>;

impl<N> Graph for Adjacency<N>
where
    N: Vertex,
{
    type NodeId = N;

    fn new<I>(n_vertices: usize, edges: I) -> Self
    where
        I: ExactSizeIterator<Item = Edge<N>>,
    {
        let mut s = Self {
            matrix: Default::default(),
            missing_links: edges.len(),
            n_edges: edges.len(),
            n_vertices,
        };
        for i in 0..n_vertices {
            s.matrix.insert(N::from(i), Default::default());
        }
        edges.for_each(|(from, to)| {
            s.add_link(from, to);
        });
        s
    }

    fn vertices(&self) -> usize {
        self.n_vertices
    }

    fn edges(&self) -> usize {
        self.n_edges
    }

    fn random_edge<R: Rng>(&self, mut rng: R) -> Edge<N> {
        self.matrix
            .iter()
            .flat_map(|(&i, neigh)| neigh.iter().map(move |&j| (i, j)))
            .choose(&mut rng)
            .expect("Graph was empty")
    }
}

impl<N> MutableGraph for Adjacency<N>
where
    N: Vertex,
{
    fn parcial<I>(n_vertices: usize, n_links: usize, edges: I) -> Self
    where
        I: IntoIterator<Item = Edge<N>>,
    {
        let mut s = Self {
            matrix: Default::default(),
            missing_links: n_links,
            n_edges: n_links,
            n_vertices,
        };
        for i in 0..n_vertices {
            s.matrix.insert(N::from(i), Default::default());
        }
        edges.into_iter().for_each(|(from, to)| {
            s.add_link(from, to);
        });
        s
    }

    fn add_link(&mut self, from: N, to: N) -> bool {
        if self.missing_links == 0 {
            false
        } else if let Some(neigh) = self.matrix.get_mut(&from) {
            neigh.insert(to);
            self.missing_links -= 1;
            true
        } else {
            false
        }
    }
}

impl<N> ContractableGraph for Adjacency<N>
where
    N: Vertex,
{
    fn contract(&mut self, (start, end): Edge<N>) {
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
}

impl<N> Index<N> for Adjacency<N>
where
    N: Vertex,
{
    type Output = Neighbours<N>;
    fn index(&self, u: N) -> &Self::Output {
        &self.matrix[&u]
    }
}

impl<N> Debug for Adjacency<N>
where
    N: Vertex + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.neighbourhoods()
            .try_for_each(|(i, s)| writeln!(f, "{:?}: {:?}", i, s.format(" -> ")))
    }
}

impl<N> Adjacency<N>
where
    N: Vertex,
{
    pub fn neighbours(&self, node: NodeId<N>) -> impl Iterator<Item = &<Self as Graph>::NodeId> {
        self.matrix[&node].iter()
    }

    pub fn neighbourhoods(&self) -> impl Iterator<Item = (&N, impl Iterator<Item = &N>)> {
        self.matrix
            .iter()
            .map(|(start, neigh)| (start, neigh.iter()))
    }
}

#[cfg(test)]
mod tests {
    use crate::graphs::{matrix::Adjacency, MutableGraph};
    #[test]
    fn add_link() {
        let mut g = Adjacency::empty(3, 6);
        assert!(g.add_link(1, 2));
        assert!(g[1].contains(&2));
    }

    #[test]
    #[should_panic]
    fn add_invalid_link() {
        let mut g = Adjacency::empty(3, 2);
        assert!(g.add_link(3, 1));
    }

    #[test]
    #[should_panic]
    fn add_invalid_link2() {
        let mut g = Adjacency::empty(3, 2);
        assert!(g.add_link(0, 2));
        assert!(g.add_link(0, 3));
        assert!(g.add_link(0, 4));
    }
}
