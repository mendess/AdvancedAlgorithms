use crate::graphs::{EdgeListGraph, Graph, MutableGraph};
use rand::{seq::SliceRandom, Rng};

type NodeId = <EdgeList as Graph>::NodeId;
type Edge = super::Edge<NodeId>;

pub struct EdgeList {
    edges: Vec<Edge>,
    n_vertices: usize,
}

impl Graph for EdgeList {
    type NodeId = usize;

    fn new<I>(n_vertices: usize, edges: I) -> Self
    where
        I: ExactSizeIterator<Item = Edge>,
    {
        Self {
            n_vertices,
            edges: edges.collect(),
        }
    }

    fn vertices(&self) -> usize {
        self.n_vertices
    }

    fn edges(&self) -> usize {
        self.edges.len()
    }

    fn random_edge<R: Rng>(&self, mut rng: R) -> Edge {
        *self.edges.choose(&mut rng).unwrap()
    }
}

impl EdgeListGraph for EdgeList {
    fn as_edges(&self) -> &[Edge] {
        &self.edges[..]
    }
    fn as_edges_mut(&mut self) -> &mut [Edge] {
        &mut self.edges[..]
    }
    fn into_edges(self) -> Vec<Edge> {
        self.edges
    }
}

impl MutableGraph for EdgeList {
    fn parcial<I>(n_vertices: usize, n_links: usize, edges: I) -> Self
    where
        I: IntoIterator<Item = Edge>,
    {
        let mut elist = Vec::with_capacity(n_links);
        elist.extend(edges);
        Self {
            n_vertices,
            edges: elist,
        }
    }

    fn add_link(&mut self, from: Self::NodeId, to: Self::NodeId) -> bool {
        if self.edges.capacity() == self.edges.len() {
            false
        } else {
            self.edges.push((from, to));
            true
        }
    }
}
